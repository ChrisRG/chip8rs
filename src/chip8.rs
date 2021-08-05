use minifb::Key;
use std::{fs::File, io::Read};

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::disassembler::Disassembler;

pub struct Chip8 {
    bus: Bus,
    cpu: Cpu,
    rom: Vec<u8>,
}

impl Chip8 {
    pub fn new(rom_file: String) -> Chip8 {
        println!("Starting CHIP-8 emulator...");
        let mut rom_buffer = Vec::<u8>::new();
        let mut file = File::open(rom_file).expect("File not found");
        if let Ok(bytes_read) = file.read_to_end(&mut rom_buffer) {
            println!("{} bytes loaded", bytes_read);
        } else {
            println!("Error loading ROM");
        };

        Chip8 {
            bus: Bus::new(),
            cpu: Cpu::new(&rom_buffer),
            rom: rom_buffer,
        }
    }

    pub fn disassemble(&self, rom_name: String) {
        let disassembler = Disassembler::new(&self.rom);
        let result = disassembler.run(rom_name);
        match result {
            Ok(_) => println!("ROM disassembly written to file"),
            Err(_) => println!("Error in disassembling ROM"),
        };
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

    pub fn should_redraw(&self) -> bool {
        self.cpu.should_redraw()
    }
    pub fn should_beep(&self) -> bool {
        self.cpu.should_beep()
    }
}
