use rand::distributions::Distribution;

use super::cpu::{CPU, VIDEO_HEIGHT, VIDEO_WIDTH, FONTSET_START_ADDRESS};

impl CPU {
    #[allow(non_snake_case)]
    pub(super) fn op_00E0(&mut self) {
        self.video.fill(0);
    }

    #[allow(non_snake_case)]
    pub(super) fn op_00EE(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp] as usize;
    }

    pub(super) fn op_1nnn(&mut self) {
        self.pc = (self.opcode & 0x0FFF) as usize;
    }

    pub(super) fn op_2nnn(&mut self) {
        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = (self.opcode & 0x0FFF) as usize;
    }

    pub(super) fn op_3xkk(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let kk = (self.opcode & 0x00FF) as u8;

        if self.registers[x as usize] == kk {
            self.pc += 2;
        }
    }

    pub(super) fn op_4xkk(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let kk = (self.opcode & 0x00FF) as u8;

        if self.registers[x as usize] != kk {
            self.pc += 2;
        }
    }

    pub(super) fn op_5xy0(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        if self.registers[x as usize] == self.registers[y as usize] {
            self.pc += 2;
        }
    }

    pub(super) fn op_6xkk(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let kk = (self.opcode & 0x00FF) as u8;

        self.registers[x as usize] = kk;
    }

    pub(super) fn op_7xkk(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let kk = (self.opcode & 0x00FF) as u8;

        self.registers[x as usize] += kk;
    }

    pub(super) fn op_8xy0(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        self.registers[x as usize] = self.registers[y as usize];
    }

    pub(super) fn op_8xy1(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        self.registers[x as usize] |= self.registers[y as usize];
    }

    pub(super) fn op_8xy2(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        self.registers[x as usize] &= self.registers[y as usize];
    }

    pub(super) fn op_8xy3(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        self.registers[x as usize] ^= self.registers[y as usize];
    }

    pub(super) fn op_8xy4(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;
        let carry;

        (self.registers[x as usize], carry) = self.registers[x as usize].overflowing_add(self.registers[y as usize]);

        self.registers[0xF] = carry as u8;
    }

    pub(super) fn op_8xy5(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;
        let carry;

        (self.registers[x as usize], carry) = self.registers[x as usize].overflowing_sub(self.registers[y as usize]);

        self.registers[0xF] = !carry as u8;
    }

    pub(super) fn op_8xy6(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;

        self.registers[0xF] = self.registers[x as usize] & 0x1;
        self.registers[x as usize] >>= 1;
    }

    pub(super) fn op_8xy7(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;
        let carry;

        (self.registers[x as usize], carry) = self.registers[y as usize].overflowing_sub(self.registers[x as usize]);

        self.registers[0xF] = !carry as u8;
    }

    #[allow(non_snake_case)]
    pub(super) fn op_8xyE(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;

        self.registers[0xF] = (self.registers[x as usize] & 0x80) >> 7;
        self.registers[x as usize] <<= 1;
    }

    pub(super) fn op_9xy0(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        if self.registers[x as usize] != self.registers[y as usize] {
            self.pc += 2;
        }
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Annn(&mut self) {
        self.index = (self.opcode & 0x0FF) as usize;
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Bnnn(&mut self) {
        self.pc = (self.registers[0] as usize) + ((self.opcode & 0x0FF) as usize);
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Cxkk(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let kk = (self.opcode & 0x00FF) as u8;

        self.registers[x as usize] = self.rand_dist.sample(&mut self.rng) & kk;
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Dxyn(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;
        let n = self.opcode & 0x000F;

        let x_pos = (self.registers[x as usize] as usize) % VIDEO_WIDTH;
        let y_pos = (self.registers[y as usize] as usize) % VIDEO_HEIGHT;

        self.registers[0xF] = 0;

        for row in 0..n as usize {
            let sprite_byte = self.memory[self.index + row];

            for col in 0..8 as usize {
                let sprite_pixel = sprite_byte & (0x80 >> col);
                let screen_pixel = &mut self.video[(y_pos + row) * VIDEO_WIDTH + (x_pos + col)];

                if sprite_pixel != 0 {
                    if *screen_pixel != 0xFFFFFFFF {
                        self.registers[0xF] = 1;
                    }

                    *screen_pixel ^= 0xFFFFFFFF;
                }
            }
        }
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Ex9E(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let key = self.registers[x as usize];

        if self.keypad[key as usize] != 0 {
            self.pc += 2;
        }
    }

    #[allow(non_snake_case)]
    pub(super) fn op_ExA1(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let key = self.registers[x as usize];

        if self.keypad[key as usize] == 0 {
            self.pc += 2;
        }
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Fx07(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        self.registers[x as usize] = self.dt;
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Fx0A(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let key_pressed = self.registers.iter().position(|num| *num != 0);

        if let Some(idx_key) = key_pressed {
            self.registers[x as usize] = idx_key as u8;
        } else {
            self.pc -= 2;
        }
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Fx15(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;

        self.dt = self.registers[x as usize];
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Fx18(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        
        self.st = self.registers[x as usize];
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Fx1E(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        
        self.index += self.registers[x as usize] as usize;
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Fx29(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let digit = self.registers[x as usize] as usize;
        
        self.index = FONTSET_START_ADDRESS + (5 * digit);
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Fx33(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let mut val = self.registers[x as usize];

        self.memory[self.index + 2] = val % 10;
        val /= 10;

        self.memory[self.index + 1] = val % 10;
        val /= 10;

        self.memory[self.index] = val % 10;
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Fx55(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;

        self.memory[self.index..(self.index + 1 + x as usize)].copy_from_slice(&self.registers[..(1 + x as usize)]);
    }

    #[allow(non_snake_case)]
    pub(super) fn op_Fx65(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;

        self.registers[..(1 + x as usize)].copy_from_slice(&self.memory[self.index..(self.index + 1 + x as usize)]);
    }
}