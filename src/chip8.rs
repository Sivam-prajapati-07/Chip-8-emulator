// src/chip8.rs

pub const START_ADDRESS: u16 = 0x200;
pub const FONTSET_START_ADDRESS: u16 = 0x050;

pub const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub index_reg: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            memory: [0; 4096],
            registers: [0; 16],
            index_reg: 0,
            pc: START_ADDRESS,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        };

        for i in 0..FONTSET.len() {
            chip8.memory[FONTSET_START_ADDRESS as usize + i] = FONTSET[i];
        }

        chip8
    }

    pub fn load_rom(&mut self, rom_bytes: &[u8]) {
        for (i, &byte) in rom_bytes.iter().enumerate() {
            let addr = (START_ADDRESS as usize) + i;
            if addr < 4096 {
                self.memory[addr] = byte;
            }
        }
    }

    pub fn step(&mut self) {
        if (self.pc as usize) < 4094 {
            let op_high = self.memory[self.pc as usize] as u16;
            let op_low = self.memory[(self.pc + 1) as usize] as u16;
            let _opcode = (op_high << 8) | op_low;

            self.pc += 2;
        }
    }
}