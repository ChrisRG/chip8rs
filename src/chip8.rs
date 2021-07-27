use minifb::Key;

use crate::bus::Bus;
use crate::cpu::Cpu;

pub struct Chip8 {
    bus: Bus,
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        self.cpu.load_rom(rom);
    }

    pub fn execute_cycle(&mut self) {
        self.cpu.execute_cycle(&mut self.bus);
    }

    pub fn get_frame_buffer(&self) -> &[u8] {
        self.bus.display.get_frame_buffer()
    }

    pub fn get_frame_index(&self, x: usize, y: usize) -> usize {
        self.bus.display.get_index(x, y)
    }

    pub fn set_key_pressed(&mut self, key: Option<Key>) {
        self.bus.set_key_pressed(key);
    }

    pub fn should_beep(&self) -> bool {
        self.cpu.should_beep()
    }
}
