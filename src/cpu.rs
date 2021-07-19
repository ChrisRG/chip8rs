const RAM: usize = 4096;

pub struct Cpu {
    pc: usize,
    i: usize,
    sp: usize,
    ram: [u8; RAM],
    stack: [usize; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0x200,
            sp: 0,
            i: 0,
            ram: [0u8; RAM],
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}

#[cfg(test)]
#[path = "./tests/cpu_tests.rs"]
mod cpu_tests;
