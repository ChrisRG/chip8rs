const RAM_SIZE: usize = 4096;

pub struct Ram {
    memory: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            memory: [0; RAM_SIZE],
        }
    }

    pub fn write_byte(&mut self, index: usize, byte: u8) {
        self.memory[index] = byte;
    }

    pub fn read_byte(&self, index: usize) -> u8 {
        self.memory[index]
    }
}
