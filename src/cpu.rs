use crate::display::Display;
use crate::ram::Ram;
use std::fmt;

const START_ROM: usize = 512; // 0x200

pub struct Cpu {
    ram: Ram,
    display: Display,
    pc: usize,
    v: [u8; 16],
    i: usize,
    sp: usize,
    stack: [usize; 16],
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
            sp: 0,
            i: 0,
            stack: [0; 16],
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
    pub fn execute_cycle(&mut self) {
        let opcode = self.fetch_op();
        self.decode_op(opcode);
        self.update_timers();
    }
    fn fetch_op(&mut self) -> u16 {
        // Load from self.pc (2 bytes), so fetch two successive bytes
        let hi_byte = self.ram.read_byte(self.pc) as u16;
        let lo_byte = self.ram.read_byte(self.pc + 1) as u16;
        let opcode = hi_byte << 8 | lo_byte;
        println!("[{:#X}]", opcode);
        opcode
    }
    fn decode_op(&mut self, opcode: u16) {
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
                0xE0 => self.op_00e0(), // 00E0: Clear display
                0xEE => self.op_00ee(), // 00EE: Return from subroutine
                _ => self.op_0nnn(nnn), // 0nnn: jump to a machine code routine at nnn
            },
            (0x01, _, _, _) => self.op_1nnn(nnn), // 1nnn: JP addrl Jump to location nnn. The interpreter sets the program counter to nnn.
            (0x02, _, _, _) => self.op_2nnn(nnn), // 2nnn: Call subroutine at nnn. The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
            (0x03, _, _, _) => self.op_3xkk(kk), // 3xkk: Skip next instruction if Vx = kk. The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
            (0x04, _, _, _) => self.op_4xkk(kk), // 4xkk: Skip next instruction if Vx != kk. The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
            (0x05, _, _, _) => self.op_5xy0(), // 5xy0: Skip next instruction if Vx = Vy. The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
            (0x06, _, _, _) => self.op_6xkk(kk), // 6xkk: Set Vx = kk. The interpreter puts the value kk into register Vx.
            (0x07, _, _, _) => self.op_7xkk(kk), // 7xkk: Set Vx = Vx + kk. Adds the value kk to the value of register Vx, then stores the result in Vx.
            (0x08, _, _, _) => match n {
                0x00 => self.op_8xy0(), //  8xy0: Set Vx = Vy. Stores the value of register Vy in register Vx.
                0x01 => self.op_8xy1(), //  8xy1: Set Vx = Vx OR Vy. Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
                0x02 => self.op_8xy2(), //  8xy2: Set Vx = Vx AND Vy. Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
                0x03 => self.op_8xy3(), //  8xy3: Set Vx = Vx XOR Vy. Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
                0x04 => self.op_8xy4(x, y), //  8xy4: Set Vx = Vx + Vy, set VF = carry. The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
                0x05 => self.op_8xy5(), //  8xy5: Set Vx = Vx - Vy, set VF = NOT borrow. If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                0x06 => self.op_8xy6(), //  8xy6: Set Vx = Vx SHR 1. If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                0x07 => self.op_8xy7(), //  8xy7: Set Vx = Vy - Vx, set VF = NOT borrow. If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
                0x0E => self.op_8xy8(), //  8xyE: Set Vx = Vx SHL 1. If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                _ => {}
            },
            (0x09, _, _, _) => self.op_9xy0(), // 9xy0: Skip next instruction if Vx != Vy. The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
            (0x0A, _, _, _) => self.op_annn(nnn), // ANNN: Set I to NNN
            (0x0B, _, _, _) => self.op_bnnn(nnn), // Bnnn: Jump to location nnn + V0. The program counter is set to nnn plus the value of V0.
            (0x0C, _, _, _) => self.op_cxkk(kk), // Cxkk: Set Vx = random byte AND kk. The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
            (0x0D, _, _, _) => self.op_dxyn(n), // Dxyn: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision. The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
            (0x0E, _, _, _) => match kk {
                0x9E => self.op_ex9e(), //  Ex9E:  Skip next instruction if key with the value of Vx is pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
                0xA1 => self.op_exa1(), //  ExA1: Skip next instruction if key with the value of Vx is not pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
                _ => {}
            },
            (0x0F, _, _, _) => match kk {
                0x07 => self.op_fx07(), //  Fx07: Set Vx = delay timer value. The value of DT is placed into Vx.
                0x0A => self.op_fx0a(), //  Fx0A: Wait for a key press, store the value of the key in Vx. All execution stops until a key is pressed, then the value of that key is stored in Vx.
                0x15 => self.op_fx15(), //  Fx15: Set delay timer = Vx. DT is set equal to the value of Vx.
                0x18 => self.op_fx18(), //  Fx18: Set sound timer = Vx. ST is set equal to the value of Vx.
                0x1E => self.op_fx1e(), //  Fx1E: Set I = I + Vx. The values of I and Vx are added, and the results are stored in I.
                0x29 => self.op_fx29(), //  Fx29: Set I = location of sprite for digit Vx. The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
                0x33 => self.op_fx33(x), //  Fx33: Store BCD representation of Vx in memory locations I, I+1, and I+2. The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
                0x55 => self.op_fx55(), //  Fx55: Store registers V0 through Vx in memory starting at location I. The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
                0x65 => self.op_fx65(), //  Fx65: Read registers V0 through Vx from memory starting at location I. The interpreter reads values from memory starting at location I into registers V0 through Vx.
                _ => {}
            },
            _ => println!("Unrecognized opcode {:?}", nibbles),
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
            self.delay_timer -= 1;
        }
    }

    fn op_00e0(&mut self) {
        self.display.clear();
    }
    fn op_00ee(&mut self) {}
    fn op_0nnn(&mut self, nnn: u16) {}

    fn op_1nnn(&mut self, nnn: u16) {}

    fn op_2nnn(&mut self, nnn: u16) {
        self.stack[self.sp] = self.pc + 2;
        self.sp += 1;
        self.pc = nnn as usize;
    }

    fn op_3xkk(&mut self, kk: u8) {}
    fn op_4xkk(&mut self, kk: u8) {}
    fn op_5xy0(&mut self) {} // 5xy0: Skip next instruction if Vx = Vy. The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn op_6xkk(&mut self, kk: u8) {} // 6xkk: Set Vx = kk. The interpreter puts the value kk into register Vx.
    fn op_7xkk(&mut self, kk: u8) {}

    fn op_8xy0(&mut self) {} //  8xy0: Set Vx = Vy. Stores the value of register Vy in register Vx.
    fn op_8xy1(&mut self) {} //  8xy1: Set Vx = Vx OR Vy. Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn op_8xy2(&mut self) {} //  8xy2: Set Vx = Vx AND Vy. Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn op_8xy3(&mut self) {} //  8xy3: Set Vx = Vx XOR Vy. Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.

    //  8xy4: Set Vx = Vx + Vy, set VF = carry. The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let vx = self.read_reg(x) as u16;
        let vy = self.read_reg(y) as u16;
        let result = vx + vy;
        let carry_flag = if result > 0xFF { 1 } else { 0 };
        self.write_reg(x, result as u8);
        self.write_reg(0x0F, carry_flag);
        self.pc += 2;
    }
    fn op_8xy5(&mut self) {} //  8xy5: Set Vx = Vx - Vy, set VF = NOT borrow. If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn op_8xy6(&mut self) {} //  8xy6: Set Vx = Vx SHR 1. If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn op_8xy7(&mut self) {} //  8xy7: Set Vx = Vy - Vx, set VF = NOT borrow. If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn op_8xy8(&mut self) {} //  8xyE: Set Vx = Vx SHL 1. If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.

    fn op_9xy0(&mut self) {} // 9xy0: Skip next instruction if Vx != Vy. The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.

    // ANNN => sets I to last 12 bits of opcode
    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn as usize;
        self.pc += 2;
        println!("{:?}", self);
    }

    fn op_bnnn(&mut self, nnn: u16) {} // Bnnn: Jump to location nnn + V0. The program counter is set to nnn plus the value of V0.
    fn op_cxkk(&mut self, kk: u8) {} // Cxkk: Set Vx = random byte AND kk. The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    fn op_dxyn(&mut self, n: u8) {} // Dxyn: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision. The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.

    fn op_ex9e(&mut self) {} //  Ex9E:  Skip next instruction if key with the value of Vx is pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    fn op_exa1(&mut self) {} //  ExA1: Skip next instruction if key with the value of Vx is not pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.

    fn op_fx07(&mut self) {} //  Fx07: Set Vx = delay timer value. The value of DT is placed into Vx.
    fn op_fx0a(&mut self) {} //  Fx0A: Wait for a key press, store the value of the key in Vx. All execution stops until a key is pressed, then the value of that key is stored in Vx.
    fn op_fx15(&mut self) {} //  Fx15: Set delay timer = Vx. DT is set equal to the value of Vx.
    fn op_fx18(&mut self) {} //  Fx18: Set sound timer = Vx. ST is set equal to the value of Vx.
    fn op_fx1e(&mut self) {} //  Fx1E: Set I = I + Vx. The values of I and Vx are added, and the results are stored in I.
    fn op_fx29(&mut self) {} //  Fx29: Set I = location of sprite for digit Vx. The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.

    //  Fx33: Store BCD representation of Vx in memory locations I, I+1, and I+2. The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    fn op_fx33(&mut self, x: usize) {
        self.ram.write_byte(self.i, self.v[x] / 100);
        self.ram.write_byte(self.i + 1, (self.v[x] % 100) / 10);
        self.ram.write_byte(self.i + 2, self.v[x] % 10);
        self.pc += 2;
    }
    fn op_fx55(&mut self) {} //  Fx55: Store registers V0 through Vx in memory starting at location I. The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    fn op_fx65(&mut self) {} //  Fx65: Read registers V0 through Vx from memory starting at location I. The interpreter reads values from memory starting at location I into registers V0 through Vx.

    fn write_reg(&mut self, index: usize, value: u8) {
        self.v[index] = value;
    }

    fn read_reg(&mut self, index: usize) -> u8 {
        self.v[index]
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
