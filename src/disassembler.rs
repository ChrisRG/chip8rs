use crate::ram::Ram;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const START_ROM: usize = 512; // 0x200

pub struct Disassembler {
    pub ram: Ram,
    rom_size: usize,
}

impl Disassembler {
    pub fn new(rom: &Vec<u8>) -> Self {
        Self {
            ram: Ram::new(rom),
            rom_size: rom.len() + START_ROM,
        }
    }

    pub fn run(&self, rom_name: String) -> Result<(), String> {
        let mut opcode_buffer = Vec::new();
        for idx in START_ROM..self.rom_size {
            // Check opcodes only at even addresses to prevent overflow
            // Possible problems since some ROMs include binary data at various addresses
            if idx & 1 == 0 && idx + 1 < self.rom_size {
                let opcode = self.fetch_op(idx);
                let instruction = format!("{}", self.decode_op(opcode));
                opcode_buffer.push(instruction);
            }
        }
        match self.write_file(rom_name, opcode_buffer) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error {}", e)),
        }
    }

    fn write_file(&self, rom_name: String, buffer: Vec<String>) -> std::io::Result<()> {
        let file_name = self.parse_path(rom_name);
        let path = Path::new(&file_name);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(e) => panic!("Couldn't create {}: {}", display, e),
            Ok(file) => file,
        };

        writeln!(file, "{}", buffer.join("\n"))?;
        Ok(())
    }

    fn parse_path(&self, rom_name: String) -> String {
        let file_name: Vec<_> = rom_name.split(".ch8").collect();
        return format!("{}.chasm", file_name[0]);
    }

    fn fetch_op(&self, idx: usize) -> u16 {
        let hi_byte = self.ram.memory[idx];
        let lo_byte = self.ram.memory[idx + 1];
        return (hi_byte as u16) << 8 | lo_byte as u16;
    }

    fn decode_op(&self, opcode: u16) -> String {
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
        let result = match nibbles {
            (0x00, _, _, _) => match kk {
                0xE0 => String::from("CLS"), // 00E0 - CLS: Clear display
                0xEE => String::from("RET"), // 00EE - RET : Return from subroutine
                _ => format!("{:x}", opcode),
            },
            (0x01, _, _, _) => format!("JP {}", nnn), // 1NNN - JP addr: Jump to location nnn.
            (0x02, _, _, _) => format!("CALL {}", nnn), // 2NNN - CALL addr: Call subroutine at nnn.
            (0x03, _, _, _) => format!("SE V{}, {}", x, kk), // 3XKK - SE Vx, byte: Skip next instruction if Vx = kk.
            (0x04, _, _, _) => format!("SNE V{}, {}", x, kk), // 4XKK - SNE Vx, byte: Skip next instruction if Vx != kk.
            (0x05, _, _, _) => format!("SE V{}, V{}", x, y), // 5XY0 - SE Vx, Vy: Skip next instruction if Vx = Vy.
            (0x06, _, _, _) => format!("LD V{}, {}", x, kk), // 6XKK - LD Vx, byte: Set Vx = kk.
            (0x07, _, _, _) => format!("ADD V{}, {}", x, kk), // 7XKK - ADD Vx, byte: Set Vx = Vx + kk.
            (0x08, _, _, _) => match n {
                0x00 => format!("LD V{}, V{}", x, y), //  8XY0 - LD Vx, Vy: Set Vx = Vy.
                0x01 => format!("OR V{}, V{}", x, y), //  8XY1 - OR Vx, Vy: Set Vx = Vx OR Vy.
                0x02 => format!("AND V{}, V{}", x, y), //  8XY2 - AND Vx, Vy: Set Vx = Vx AND Vy.
                0x03 => format!("XOR V{}, V{}", x, y), //  8XY3 - XOR Vx, Vy: Set Vx = Vx XOR Vy.
                0x04 => format!("ADD V{}, V{}", x, y), //  8XY4 - ADD Vx, Vy: Set Vx = Vx + Vy, set VF = carry.
                0x05 => format!("SUB V{}, V{}", x, y), //  8XY5 - SUB Vx, Vy: Set Vx = Vx - Vy, set VF = NOT borrow.
                0x06 => format!("SHR V{}", x),         //  8XY6 - SHR Vx: Set Vx = Vx SHR 1.
                0x07 => format!("SUBN V{} V{}", x, y), //  8XY7 - SUBN Vx, Vy: Set Vx = Vy - Vx, set VF = NOT borrow.
                0x0E => format!("SHL V{}", x),         //  8XYE - SHL Vx: Set Vx = Vx SHL 1.
                _ => format!("{:x}", opcode),
            },
            (0x09, _, _, _) => format!("SNE V{}, V{}", x, y), // 9XY0 - SNE Vx, Vy: Skip next instruction if Vx != Vy.
            (0x0A, _, _, _) => format!("LD I, {}", nnn),      // ANNN - LD I, addr: Set I to NNN
            (0x0B, _, _, _) => format!("JP V0, {}", nnn), // BNNN - JP V0, addr: Jump to location nnn + V0.
            (0x0C, _, _, _) => format!("RND V{}, {}", x, kk), // CXKK - RND Vx, byte: Set Vx = random byte AND kk.
            (0x0D, _, _, _) => format!("DRW V{}, V{}, {}", x, y, n), // DXYN - DRW, Vx, Vy, nibble: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            (0x0E, _, _, _) => match kk {
                0x9E => format!("SKP V{}", x), //  Ex9E - SKP Vx:  Skip next instruction if key with the value of Vx is pressed.
                0xA1 => format!("SKNP V{}", x), //  EXA1 - SKNP Vx: Skip next instruction if key with the value of Vx is not pressed.
                _ => format!("{:x}", opcode),
            },
            (0x0F, _, _, _) => match kk {
                0x07 => format!("LD V{}, DT", x), //  FX07 - LD Vx, DT: Set Vx = delay timer value. The value of DT is placed into Vx.
                0x0A => format!("LD V{}, K", x), //  FX0A - LD Vx, K: Wait for a key press, store the value of the key in Vx.
                0x15 => format!("LD DT, V{}", x), //  FX15 - LD DT, Vx: Set delay timer = Vx.
                0x18 => format!("LD ST, V{}", x), //  FX18 - LD ST, Vx: Set sound timer = Vx.
                0x1E => format!("ADD I, V{}", x), //  FX1E - ADD I, Vx: Set I = I + Vx.
                0x29 => format!("LD F, V{}", x), //  FX29 - LD F, Vx: Set I = location of sprite for digit Vx.
                0x33 => format!("LD B, V{}", x), //  FX33 - LD B, Vx: Store BCD representation of Vx in memory locations I, I+1, and I+2.
                0x55 => format!("LD I, V{}", x), //  FX55 - LD [I], Vx: Store registers V0 through Vx in memory starting at location I.
                0x65 => format!("LD V{}, I", x), //  FX65 - Ld Vx, [I]: Read registers V0 through Vx from memory starting at location I.
                _ => format!("{:x}", opcode),
            },
            _ => format!("Unrecognized opcode {:x}", opcode),
        };
        result
    }
}
