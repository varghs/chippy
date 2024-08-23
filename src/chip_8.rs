use std::path::Path;
use std::fs::File;

pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    index: usize,
    pc: usize,
    stack: [u16; 16],
    sp: usize,
    dt: u8,
    st: u8,
    keypad: [u8; 16],
    video: [u32; 64 * 32],
    opcode: u16,
}

const START_ADDRESS = 0x200;

impl Chip8 {
    pub fn load_rom(&mut self, path: Path) -> Result<(),  {
        let bytes = File::open(path)
    }
}