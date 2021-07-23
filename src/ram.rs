use crate::font;

const RAM_SIZE: usize = 4096;
const START_ROM: usize = 512; // 0x200

pub struct Ram {
    memory: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        let mut memory = [0; RAM_SIZE];

        // Load the font set into the first 80 bytes
        let font_bytes = font::FONT_SET.iter().flatten().collect::<Vec<_>>();
        for (idx, byte) in font_bytes.into_iter().enumerate() {
            memory[idx] = *byte;
        }

        Self { memory }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let buffer_size = rom.len();
        for i in 0..buffer_size {
            self.write_byte(i + START_ROM, rom[i]);
        }
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
