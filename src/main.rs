mod bus;
mod cpu;
mod display;
mod ram;

use bus::Bus;
use cpu::Cpu;

use minifb::{Key, KeyRepeat, Window, WindowOptions};

fn main() {
    println!("Starting CHIP-8 emulator...");
    let mut bus = Bus::new();
    let mut cpu = Cpu::new();
    // Load game into memory
    let test_rom = vec![0xE0, 0x9E, 0xE1, 0x9E, 0x12, 0x00];
    cpu.load_rom(test_rom);

    // Start emulation loop

    let width = 640;
    let height = 320;

    let mut window = Window::new("CHIP8RS", width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("Window creation failed: {:?}", e);
        });

    let mut buffer: Vec<u32> = vec![0; width * height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();

        let keys_pressed = window.get_keys_pressed(KeyRepeat::Yes);
        let key = match keys_pressed {
            Some(keys) => {
                if !keys.is_empty() {
                    Some(keys[0])
                } else {
                    None
                }
            }
            None => None,
        };
        bus.set_key_pressed(key);
        // window.update_with_buffer(&buffer);
        cpu.execute_cycle(&bus);
    }
}
