const RAM_SIZE: usize = 4096;
const START_ROM: usize = 512; // 0x200

pub struct Cpu {
    ram: [u8; RAM_SIZE],
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
            ram: [0u8; RAM_SIZE],
            pc: 0x200,
            v: [0x00; 16],
            sp: 0x0000,
            i: 0x0000,
            stack: [0x0000; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let buffer_size = rom.len();
        for i in 0..buffer_size {
            self.ram[i + START_ROM] = rom[i];
        }
    }
    pub fn cycle(&mut self) {
        // Fetch opcode from program counter
        println!("PC: {:x}", self.pc);
        let opcode = self.fetch_op();
        // Decode and execute opcode
        self.decode_op(opcode);
        // Update delay and sound timers
        self.update_timers();
        println!("PC: {:x}", self.pc);
    }
    fn fetch_op(&mut self) -> u16 {
        // Load from self.pc
        // Since opcode is 2 bytes, fetch two successive bytes and merge
        let opcode = (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc + 1] as u16);
        println!("Opcode: {:x}", opcode);
        opcode
    }
    fn decode_op(&mut self, opcode: u16) {
        // Split up the 2 byte opcode so that we can pattern match opcodes
        // Use bitwise & to mask each position, then shift right to fit into a byte
        let nibbles = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        );
        // NNN: might need to use last 12 bits, use bitwise & to mask again
        let nnn = (opcode & 0x0FFF) as usize;
        match nibbles {
            (0x00, 0x00, 0x0E, 0x00) => {}        // 00E0: Clear screen
            (0x00, 0x00, 0x0E, 0x0E) => {}        // 00EE: Return from subroutine
            (0x0A, _, _, _) => self.op_annn(nnn), // ANNN: Set I to NNN
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
    // ANNN => sets I to last 12 bits of opcode
    fn op_annn(&mut self, nnn: usize) {
        self.i = nnn;
        println!("I: {:x}", nnn);
        self.pc += 2;
    }
}

#[cfg(test)]
#[path = "./tests/cpu_tests.rs"]
mod cpu_tests;
