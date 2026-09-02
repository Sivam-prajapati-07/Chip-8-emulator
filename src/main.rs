// src/main.rs

mod chip8;
mod ui;

use ui::EmulatorApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 480.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "CHIP-8 Visual Architecture",
        options,
        Box::new(|_cc| Box::new(EmulatorApp::default())),
    )
}