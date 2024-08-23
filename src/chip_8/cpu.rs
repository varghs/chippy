use std::io::Read;
use std::path::Path;
use std::fs::File;
use rand::{distributions::Uniform, rngs::ThreadRng};

pub const VIDEO_WIDTH: usize = 64;
pub const VIDEO_HEIGHT: usize = 32;
pub struct CPU {
    pub(super) registers: [u8; 16],
    pub(super) memory: [u8; 4096],
    pub(super) index: usize,
    pub(super) pc: usize,
    pub(super) stack: [u16; 16],
    pub(super) sp: usize,
    pub(super) dt: u8,
    pub(super) st: u8,
    pub(super) keypad: [u8; 16],
    pub(super) video: [u32; VIDEO_WIDTH * VIDEO_HEIGHT],
    pub(super) opcode: u16,
    pub(super) rng: ThreadRng,
    pub(super) rand_dist: Uniform<u8>,
}

const START_ADDRESS: usize = 0x200;
const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] =
[
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

pub const FONTSET_START_ADDRESS: usize = 0x50;

impl CPU {
    pub fn new() -> Self {
        let mut cpu = CPU {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: START_ADDRESS,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            keypad: [0; 16],
            video: [0; 64 * 32],
            opcode: 0,
            rng: rand::thread_rng(),
            rand_dist: Uniform::new(0, 255),
        };

        cpu.load_fonts();

        cpu
    }

    fn load_fonts(&mut self) {
        self.memory[FONTSET_START_ADDRESS..(FONTSET_START_ADDRESS + FONTSET_SIZE)].copy_from_slice(&FONTSET);
    }

    pub fn load_rom(&mut self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = File::open(path)?;
        file.read(&mut self.memory[START_ADDRESS..])?;
        Ok(())
    }

    fn cycle(&mut self) {
        self.opcode = ((self.memory[self.pc] as u16) << 8) | self.memory[self.pc + 1] as u16;
        self.pc += 2;

        self.execute();

        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }
    }

    fn execute(&mut self) {
        let d1 = (self.opcode & 0xF000) >> 12;
        let d2 = (self.opcode & 0x0F00) >> 8;
        let d3 = (self.opcode & 0x00F0) >> 4;
        let d4 = self.opcode & 0x000F;

        match (d1, d2, d3, d4) {
            (0x0, 0x0, 0xE, 0x0) => self.op_00E0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00EE(),
            (0x1, _, _, _) => self.op_1nnn(),
            (0x2, _, _, _) => self.op_2nnn(),
            (0x3, _, _, _) => self.op_3xkk(),
            (0x4, _, _, _) => self.op_4xkk(),
            (5, _, _, 0) => self.op_5xy0(),
            (6, _, _, _) => self.op_6xkk(),
            (7, _, _, _) => self.op_7xkk(),
            (8, _, _, 0) => self.op_8xy0(),
            (8, _, _, 1) => self.op_8xy1(),
            (8, _, _, 2) => self.op_8xy2(),
            (8, _, _, 3) => self.op_8xy3(),
            (8, _, _, 4) => self.op_8xy4(),
            (8, _, _, 5) => self.op_8xy5(),
            (8, _, _, 6) => self.op_8xy6(),
            (8, _, _, 7) => self.op_8xy7(),
            (8, _, _, 0xE) => self.op_8xyE(),
            (9, _, _, 0) => self.op_9xy0(),
            (0xA, _, _, _) => self.op_Annn(),
            (0xB, _, _, _) => self.op_Bnnn(),
            (0xC, _, _, _) => self.op_Cxkk(),
            (0xD, _, _, _) => self.op_Dxyn(),
            (0xE, _, 9, 0xE) => self.op_Ex9E(),
            (0xE, _, 0xA, 1) => {},
            (0xF, _, 0, 7) => {},
            (0xF, _, 0, 0xA) => {},
            (0xF, _, 1, 5) => {},
            (0xF, _, 1, 8) => {},
            (0xF, _, 1, 0xE) => {},
            (0xF, _, 2, 9) => {},
            (0xF, _, 3, 3) => {},
            (0xF, _, 5, 5) => {},
            (0xF, _, 6, 5) => {},
            _ => panic!("Invalid opcode: {:04X}", self.opcode),
        }
    }
}