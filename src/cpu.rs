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
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;
        let nn = (opcode & 0x00FF) as usize;
        let nnn = (opcode & 0x0FFF) as usize;
        println!("NNN: {}", nnn);
        match nibbles {
            (0x00, 0x00, 0x0E, 0x00) => {}            // 00E0: Clear screen
            (0x00, 0x00, 0x0E, 0x0E) => {}            // 00EE: Return from subroutine
            (0x0A, _, _, _) => self.op_annn(nnn),     // ANNN: Set I to NNN
            (0x02, _, _, _) => self.op_2nnn(nnn),     // 2NNN: Call subroutine at NNN
            (0x08, _, _, 0x04) => self.op_8xy4(x, y), // 8XY4: Add VY to VX
            (0x0F, _, 0x03, 0x03) => self.op_fx33(x), // FX33: Store VX at addresses I, I+1, I+2
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

    fn op_2nnn(&mut self, nnn: usize) {
        self.stack[self.sp] = self.pc + 2;
        self.sp += 1;
        println!("Stack {:?}", self.stack);
        self.pc = nnn;
        println!("PC: {}", self.pc);
    }

    // Add VY to VX
    // If result is great than 255, VF
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let vx = self.v[x] as u16;
        let vy = self.v[y] as u16;
        let result = vx + vy;
        self.v[x] = result as u8;
        self.v[0x0F] = if result > 0xFF { 1 } else { 0 };
        self.pc += 2;
    }

    // Store decimal represnetaiton of VX in addresses I, I+1, I+2
    fn op_fx33(&mut self, x: usize) {
        self.ram[self.i] = self.v[x] / 100;
        self.ram[self.i + 1] = (self.v[x] % 100) / 10;
        self.ram[self.i + 2] = self.v[x] % 10;
        self.pc += 2;
    }
}

#[cfg(test)]
#[path = "./tests/cpu_tests.rs"]
mod cpu_tests;
