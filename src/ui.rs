// src/ui.rs

use crate::chip8::Chip8;
use eframe::egui;

pub struct EmulatorApp {
    pub chip8: Chip8,
    pub selected_page: usize,
    pub auto_play: bool,
    pub step_count: u64,
}

impl Default for EmulatorApp {
    fn default() -> Self {
        let mut chip8 = Chip8::new();
        // A mock sequence of CHIP-8 opcodes:
        // 0x00E0 (CLS), 0x6001 (V0 = 1), 0x7001 (V0 += 1), 0x610A (V1 = 10), 0x1200 (JUMP 0x200)
        let mock_rom = [0x00, 0xE0, 0x60, 0x01, 0x70, 0x01, 0x61, 0x0A, 0x12, 0x00];
        chip8.load_rom(&mock_rom);

        Self {
            chip8,
            selected_page: 0x200,
            auto_play: false,
            step_count: 0,
        }
    }
}

impl EmulatorApp {
    // Translates the 2-byte hex opcode into readable human text
    fn decode_current_opcode_preview(&self) -> (u16, String) {
        if (self.chip8.pc as usize) < 4094 {
            let hi = self.chip8.memory[self.pc_usize()] as u16;
            let lo = self.chip8.memory[self.pc_usize() + 1] as u16;
            let opcode = (hi << 8) | lo;

            let desc = match opcode & 0xF000 {
                0x0000 => match opcode {
                    0x00E0 => "DISPLAY: Clear screen (CLS)".to_string(),
                    0x00EE => "FLOW: Return from subroutine (RET)".to_string(),
                    _ => format!("SYS: Call RCA 1802 program at 0x{:03X}", opcode & 0x0FFF),
                },
                0x1000 => format!("JUMP: Go to address 0x{:03X}", opcode & 0x0FFF),
                0x6000 => format!("SET: Register V{:X} = 0x{:02X}", (opcode & 0x0F00) >> 8, opcode & 0x00FF),
                0x7000 => format!("ADD: Register V{:X} += 0x{:02X}", (opcode & 0x0F00) >> 8, opcode & 0x00FF),
                0xA000 => format!("INDEX: Set I = 0x{:03X}", opcode & 0x0FFF),
                0xD000 => format!("DRAW: Render sprite at (V{:X}, V{:X}) height {}", (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4, opcode & 0x000F),
                _ => format!("EXECUTE: Raw Opcode 0x{:04X}", opcode),
            };
            (opcode, desc)
        } else {
            (0, "END OF MEMORY".to_string())
        }
    }

    fn pc_usize(&self) -> usize {
        self.chip8.pc as usize
    }
}

impl eframe::App for EmulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.auto_play {
            self.chip8.step();
            self.step_count += 1;
            ctx.request_repaint(); // Request next frame for continuous animation
        }

        // Top Header Bar
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("⚡ CHIP-8 VIRTUAL MACHINE ARCHITECTURE").strong().color(egui::Color32::from_rgb(0, 220, 255)));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Executed Cycles: {}", self.step_count));
                });
            });
        });

        // Left Telemetry / Registers Panel
        egui::SidePanel::left("control_panel").min_width(280.0).show(ctx, |ui| {
            ui.add_space(6.0);
            ui.heading("Execution Controls");
            
            ui.horizontal(|ui| {
                if ui.button(if self.auto_play { "⏸ Pause Engine" } else { "▶ Run Engine" }).clicked() {
                    self.auto_play = !self.auto_play;
                }
                if ui.button("⏭ Single Step").clicked() {
                    self.chip8.step();
                    self.step_count += 1;
                }
                if ui.button("🔄 Reset PC").clicked() {
                    self.chip8.pc = 0x200;
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.heading("Active Instruction Decoder");

            let (opcode, description) = self.decode_current_opcode_preview();
            egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(25, 30, 40)).show(ui, |ui| {
                ui.label(egui::RichText::new(format!("OPCODE: 0x{:04X}", opcode)).size(16.0).strong().color(egui::Color32::from_rgb(255, 100, 100)));
                ui.label(egui::RichText::new(description).color(egui::Color32::from_rgb(120, 220, 255)));
                ui.label(format!("Target Address in RAM: 0x{:03X}", self.chip8.pc));
            });

            ui.add_space(10.0);
            ui.separator();
            ui.heading("Internal Registers");

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("PC: 0x{:03X}", self.chip8.pc)).strong());
                ui.label(egui::RichText::new(format!("I: 0x{:03X}", self.chip8.index_reg)).strong());
                ui.label(format!("SP: {}", self.chip8.sp));
            });

            ui.add_space(6.0);
            ui.label("V0–VF General Purpose Registers:");
            egui::Grid::new("registers_grid").striped(true).show(ui, |ui| {
                for i in 0..16 {
                    let val = self.chip8.registers[i];
                    ui.label(format!("V{:X}:", i));
                    
                    // Value with visual color indication
                    let color = if val > 0 { egui::Color32::YELLOW } else { egui::Color32::LIGHT_GRAY };
                    ui.label(egui::RichText::new(format!("0x{:02X}", val)).color(color));

                    if (i + 1) % 4 == 0 {
                        ui.end_row();
                    }
                }
            });
        });

        // Central Memory Inspector Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Physical Memory Mapping (4096 Bytes)");

            // Visual Memory Map breakdown bar
            ui.horizontal(|ui| {
                ui.label("Memory Layout:");
                ui.colored_label(egui::Color32::from_rgb(100, 100, 255), "■ 0x000-0x1FF Interpreter & Fonts");
                ui.colored_label(egui::Color32::from_rgb(0, 255, 128), "■ 0x200-0xFFF ROM & Program Data");
            });

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                if ui.button("📍 Jump to Fontset (0x050)").clicked() {
                    self.selected_page = 0x050;
                }
                if ui.button("📍 Jump to ROM Entry (0x200)").clicked() {
                    self.selected_page = 0x200;
                }
                if ui.button("📍 Follow PC Pointer").clicked() {
                    self.selected_page = (self.chip8.pc as usize / 8) * 8;
                }
            });

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("memory_view").striped(true).show(ui, |ui| {
                    let start = self.selected_page;
                    let end = (start + 128).min(4096);

                    for chunk_start in (start..end).step_by(8) {
                        ui.label(egui::RichText::new(format!("0x{:03X}:", chunk_start)).monospace().strong());

                        for offset in 0..8 {
                            let addr = chunk_start + offset;
                            let val = self.chip8.memory[addr];
                            let is_pc = self.pc_usize() == addr || (self.pc_usize() + 1) == addr;

                            let (text_color, bg_color) = if is_pc {
                                (egui::Color32::WHITE, egui::Color32::from_rgb(220, 40, 40))
                            } else if val != 0 {
                                (egui::Color32::from_rgb(0, 255, 150), egui::Color32::TRANSPARENT)
                            } else {
                                (egui::Color32::DARK_GRAY, egui::Color32::TRANSPARENT)
                            };

                            let mut label_text = egui::RichText::new(format!("{:02X}", val)).monospace().color(text_color);
                            if bg_color != egui::Color32::TRANSPARENT {
                                label_text = label_text.background_color(bg_color);
                            }

                            ui.label(label_text);
                        }
                        ui.end_row();
                    }
                });
            });
        });
    }
}