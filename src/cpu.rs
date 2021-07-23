use crate::bus::Bus;
use crate::display::Display;
use crate::ram::Ram;
use rand;
use rand::Rng;
use std::fmt;

const START_ROM: usize = 512; // 0x200

pub struct Cpu {
    display: Display,
    ram: Ram,
    pc: usize,
    v: [u8; 16],
    i: usize,
    stack: Vec<usize>,
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            ram: Ram::new(),
            display: Display::new(),
            pc: 0x200,
            v: [0x00; 16],
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let buffer_size = rom.len();
        for i in 0..buffer_size {
            self.ram.write_byte(i + START_ROM, rom[i]);
        }
    }
    pub fn execute_cycle(&mut self, bus: &Bus) {
        let opcode = self.fetch_op();
        self.decode_op(opcode, bus);
        self.update_timers();
        // println!("{:?}", self);
    }
    fn fetch_op(&mut self) -> u16 {
        // Load from self.pc (2 bytes), so fetch two successive bytes
        let hi_byte = self.ram.read_byte(self.pc) as u16;
        let lo_byte = self.ram.read_byte(self.pc + 1) as u16;
        let opcode = hi_byte << 8 | lo_byte;
        // println!("[{:#X}]", opcode);
        opcode
    }
    fn decode_op(&mut self, opcode: u16, bus: &Bus) {
        // Break up 2byte opcode into nibbles and bytes
        let nibbles = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        );
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as u8;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;
        match nibbles {
            //
            (0x00, _, _, _) => match kk {
                0xE0 => self.op_00e0(), // 00E0 - CLS: Clear display
                0xEE => self.op_00ee(), // 00EE - RET : Return from subroutine
                _ => println!("Unrecognized opcode {:?}", opcode),
            },
            (0x01, _, _, _) => self.op_1nnn(nnn), // 1NNN - JP addr: Jump to location nnn.
            (0x02, _, _, _) => self.op_2nnn(nnn), // 2NNN - CALL addr: Call subroutine at nnn.
            (0x03, _, _, _) => self.op_3xkk(x, kk), // 3XKK - SE Vx, byte: Skip next instruction if Vx = kk.
            (0x04, _, _, _) => self.op_4xkk(x, kk), // 4XKK - SNE Vx, byte: Skip next instruction if Vx != kk.
            (0x05, _, _, _) => self.op_5xy0(x, y), // 5XY0 - SE Vx, Vy: Skip next instruction if Vx = Vy.
            (0x06, _, _, _) => self.op_6xkk(x, kk), // 6XKK - LD Vx, byte: Set Vx = kk.
            (0x07, _, _, _) => self.op_7xkk(x, kk), // 7XKK - ADD Vx, byte: Set Vx = Vx + kk.
            (0x08, _, _, _) => match n {
                0x00 => self.op_8xy0(x, y), //  8XY0 - LD Vx, Vy: Set Vx = Vy.
                0x01 => self.op_8xy1(x, y), //  8XY1 - OR Vx, Vy: Set Vx = Vx OR Vy.
                0x02 => self.op_8xy2(x, y), //  8XY2 - AND Vx, Vy: Set Vx = Vx AND Vy.
                0x03 => self.op_8xy3(x, y), //  8XY3 - XOR Vx, Vy: Set Vx = Vx XOR Vy.
                0x04 => self.op_8xy4(x, y), //  8XY4 - ADD Vx, Vy: Set Vx = Vx + Vy, set VF = carry.
                0x05 => self.op_8xy5(x, y), //  8XY5 - SUB Vx, Vy: Set Vx = Vx - Vy, set VF = NOT borrow.
                0x06 => self.op_8xy6(x),    //  8XY6 - SHR Vx: Set Vx = Vx SHR 1.
                0x07 => self.op_8xy7(x, y), //  8XY7 - SUBN Vx, Vy: Set Vx = Vy - Vx, set VF = NOT borrow.
                0x0E => self.op_8xye(x),    //  8XYE - SHL Vx: Set Vx = Vx SHL 1.
                _ => println!("Unrecognized opcode {:?}", opcode),
            },
            (0x09, _, _, _) => self.op_9xy0(x, y), // 9XY0 - SNE Vx, Vy: Skip next instruction if Vx != Vy.
            (0x0A, _, _, _) => self.op_annn(nnn),  // ANNN - LD I, addr: Set I to NNN
            (0x0B, _, _, _) => self.op_bnnn(nnn),  // BNNN - JP V0, addr: Jump to location nnn + V0.
            (0x0C, _, _, _) => self.op_cxkk(x, kk), // CXKK - RND Vx, byte: Set Vx = random byte AND kk.
            (0x0D, _, _, _) => self.op_dxyn(x, y, n), // DXYN - DRW, Vx, Vy, nibble: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            (0x0E, _, _, _) => match kk {
                0x9E => self.op_ex9e(x, bus), //  Ex9E - SKP Vx:  Skip next instruction if key with the value of Vx is pressed.
                0xA1 => self.op_exa1(x, bus), //  EXA1 - SKNP Vx: Skip next instruction if key with the value of Vx is not pressed.

                _ => println!("Unrecognized opcode {:?}", opcode),
            },
            (0x0F, _, _, _) => match kk {
                0x07 => self.op_fx07(x), //  FX07 - LD Vx, DT: Set Vx = delay timer value. The value of DT is placed into Vx.
                0x0A => self.op_fx0a(x, bus), //  FX0A - LD Vx, K: Wait for a key press, store the value of the key in Vx.
                0x15 => self.op_fx15(x),      //  FX15 - LD DT, Vx: Set delay timer = Vx.
                0x18 => self.op_fx18(x),      //  FX18 - LD ST, Vx: Set sound timer = Vx.
                0x1E => self.op_fx1e(x),      //  FX1E - ADD I, Vx: Set I = I + Vx.
                0x29 => self.op_fx29(x), //  FX29 - LD F, Vx: Set I = location of sprite for digit Vx.
                0x33 => self.op_fx33(x), //  FX33 - LD B, Vx: Store BCD representation of Vx in memory locations I, I+1, and I+2.
                0x55 => self.op_fx55(x), //  FX55 - LD [I], Vx: Store registers V0 through Vx in memory starting at location I.
                0x65 => self.op_fx65(x), //  FX65 - Ld Vx, [I]: Read registers V0 through Vx from memory starting at location I.
                _ => println!("Unrecognized opcode {:?}", opcode),
            },
            _ => println!("Unrecognized opcode {:?}", opcode),
        }
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }

    fn op_00e0(&mut self) {
        self.display.clear();
        self.pc += 2;
    }

    // Return from subroutine
    fn op_00ee(&mut self) {
        let address = self.stack.pop().unwrap();
        self.pc = address;
    }

    fn op_0nnn(&mut self, nnn: u16) {
        self.pc = nnn as usize;
    }

    //  The interpreter sets the program counter to nnn.
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn as usize;
    }

    // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    fn op_2nnn(&mut self, nnn: u16) {
        self.stack.push(self.pc + 2);
        self.pc = nnn as usize;
    }

    //  The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn op_3xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    //  The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    //  The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // The interpreter puts the value kk into register Vx.
    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
        self.pc += 2;
    }

    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn op_7xkk(&mut self, x: usize, kk: u8) {
        let result = self.v[x] as u16 + kk as u16;
        self.v[x] = result as u8;
        self.pc += 2;
    }

    // Stores the value of register Vy in register Vx.
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
        self.pc += 2;
    }

    // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
        self.pc += 2;
    }

    // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
        self.pc += 2;
    }

    // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
        self.pc += 2;
    }

    //  The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let result = self.v[x] as u16 + self.v[y] as u16;
        let carry_flag = if result > 0xFF { 1 } else { 0 };
        self.v[x] = result as u8;
        self.v[0xf] = carry_flag;
        self.pc += 2;
    }

    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn op_8xy5(&mut self, x: usize, y: usize) {
        let result = self.v[x] - self.v[y];
        let carry_flag = if result < 0 { 1 } else { 0 };
        self.v[x] = result;
        self.v[0xf] = carry_flag;
        self.pc += 2;
    }

    //  If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn op_8xy6(&mut self, x: usize) {
        self.v[0xf] = x as u8 & 0x1;
        self.v[x] >>= 1;
        self.pc += 2;
    }

    //  If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn op_8xy7(&mut self, x: usize, y: usize) {
        let result = self.v[y] - self.v[x];
        let carry_flag = if result < 0 { 1 } else { 0 };
        self.v[x] = result;
        self.v[0xf] = carry_flag;
        self.pc += 2;
    }

    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    fn op_8xye(&mut self, x: usize) {
        self.v[0xf] = (self.v[x] & 0b10000000) >> 7;
        self.v[x] = self.v[x] << 1;
        self.pc += 2;
    }

    // 9xy0: Skip next instruction if Vx != Vy.
    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // ANNN => sets I to last 12 bits of opcode
    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn as usize;
        self.pc += 2;
    }

    // Bnnn: Jump to location nnn + V0. The program counter is set to nnn plus the value of V0.
    fn op_bnnn(&mut self, nnn: u16) {
        self.pc = nnn as usize + self.v[0] as usize;
    }

    // Cxkk: Set Vx = random byte AND kk.
    // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    fn op_cxkk(&mut self, x: usize, kk: u8) {
        let mut rng = rand::thread_rng();
        self.v[x] = rng.gen::<u8>() & kk;
        self.pc += 2;
    }

    // Dxyn: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    // The interpreter reads n bytes from memory, starting at the address stored in I.
    // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    // Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    // If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    fn op_dxyn(&mut self, x: usize, y: usize, n: u8) {
        self.display.draw(self.v[x] as usize, self.v[y] as usize, n);
        self.pc += 2;
    }

    //  ExA1: Skip next instruction if key with the value of Vx is NOT pressed.
    fn op_exa1(&mut self, x: usize, bus: &Bus) {
        if !bus.is_key_pressed(self.v[x]) {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // Skip next instruction if key with the value of Vx is pressed.
    fn op_ex9e(&mut self, x: usize, bus: &Bus) {
        self.v[0] = 0; // X key
        self.v[1] = 7; //
        if bus.is_key_pressed(self.v[x]) {
            println!("Pressed");
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // The value of DT is placed into Vx.
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
        self.pc += 2;
    }

    //  Fx0A: Wait for a key press, store the value of the key in Vx.
    // All execution stops until a key is pressed, then the value of that key is stored in Vx.
    fn op_fx0a(&mut self, x: usize, bus: &Bus) {
        if let Some(key_pressed) = bus.key_pressed {
            self.v[x] = key_pressed;
        }
        self.pc += 2;
    }

    //  Fx15: Set delay timer = Vx. DT is set equal to the value of Vx.
    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.v[x];
        self.pc += 2;
    }

    //  Fx18: Set sound timer = Vx. ST is set equal to the value of Vx.
    fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.v[x];
        self.pc += 2;
    }

    //  Fx1E: Set I = I + Vx. The values of I and Vx are added, and the results are stored in I.
    fn op_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as usize;
        self.pc += 2;
    }

    //  Fx29: Set I = location of sprite for digit Vx.
    // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    fn op_fx29(&mut self, x: usize) {
        self.i = (self.v[x] as usize) * 5;
        self.pc += 2;
    }

    //  Fx33: Store BCD representation of Vx in memory locations I, I+1, and I+2.
    // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    fn op_fx33(&mut self, x: usize) {
        self.ram.write_byte(self.i, self.v[x] / 100);
        self.ram.write_byte(self.i + 1, (self.v[x] % 100) / 10);
        self.ram.write_byte(self.i + 2, self.v[x] % 10);
        self.pc += 2;
    }

    //  Fx55: Store registers V0 through Vx in memory starting at location I.
    // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    fn op_fx55(&mut self, x: usize) {
        for idx in 0..x + 1 {
            let val = self.v[idx];
            self.ram.write_byte(self.i + idx, val);
        }
        self.pc += 2;
    }

    //  Fx65: Read registers V0 through Vx from memory starting at location I.
    // The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn op_fx65(&mut self, x: usize) {
        for idx in 0..x + 1 {
            let val = self.ram.read_byte(self.i + idx);
            self.v[idx] = val;
        }
        self.pc += 2;
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PC: {:#X}\n", self.pc)?;
        for (idx, reg) in self.v.iter().enumerate() {
            write!(f, "V{}:{:X}  ", idx, *reg)?;
        }
        write!(f, "\n")?;
        write!(f, "I: {:#X}", self.i)
    }
}

#[cfg(test)]
#[path = "./tests/cpu_tests.rs"]
mod cpu_tests;
