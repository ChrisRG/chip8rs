mod cpu;
mod display;
mod ram;

use cpu::Cpu;

use minifb::{Key, KeyRepeat, Window, WindowOptions};

fn main() {
    println!("Starting CHIP-8 emulator...");
    // Setup graphics - TODO
    // Setup CPU, initialize
    let mut cpu = Cpu::new();
    // Load game into memory
    let test_rom = vec![0xA2, 0xF0, 0x20, 0x08, 0xFF, 0xFF];
    cpu.load_rom(test_rom);
    // Start emulation loop
    cpu.execute_cycle();

    let width = 640;
    let height = 320;

    let mut window = Window::new("CHIP8RS", width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("Window creation failed: {:?}", e);
        });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }
}
