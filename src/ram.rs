use crate::font;

const RAM_SIZE: usize = 4096;
const START_ROM: usize = 512; // 0x200

pub struct Ram {
    pub memory: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new(rom_buffer: &[u8]) -> Self {
        let mut memory = [0; RAM_SIZE];

        // Load the font set into the first 80 bytes
        for (idx, byte) in font::FONT_SET.iter().flatten().enumerate() {
            memory[idx] = *byte;
        }

        // Load ROM into memory starting at 0x200
        let buffer_size = rom_buffer.len();
        for i in 0..buffer_size {
            memory[i + START_ROM] = rom_buffer[i];
        }

        Self { memory }
    }

    pub fn write_byte(&mut self, index: usize, byte: u8) {
        self.memory[index] = byte;
    }

    pub fn read_byte(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn read_bytes(&self, start_idx: usize, stop_idx: usize) -> &[u8] {
        &self.memory[start_idx..stop_idx]
    }
}
