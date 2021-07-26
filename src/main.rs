mod bus;
mod chip8;
mod cpu;
mod display;
mod font;
mod ram;
use std::time::{Duration, Instant};

use std::{fs::File, io::Read};

use minifb::{Key, KeyRepeat, Window, WindowOptions};

use crate::chip8::Chip8;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 320;
fn main() {
    println!("Starting CHIP-8 emulator...");
    let mut chip8 = Chip8::new();

    let mut file = File::open("./src/roms/pong.ch8").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");

    // chip8.load_rom(&test::TEST_E.to_vec());
    chip8.load_rom(&data);

    let mut window = Window::new(
        "CHIP8RS",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed: {:?}", e);
    });

    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut last_key_update_time = Instant::now();
    let mut last_instruction_run_time = Instant::now();
    let mut last_display_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
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

        if key.is_some() || Instant::now() - last_key_update_time >= Duration::from_millis(200) {
            last_key_update_time = Instant::now();
            chip8.set_key_pressed(key);
        }

        if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
            chip8.execute_cycle();
            last_instruction_run_time = Instant::now();
        }

        if Instant::now() - last_display_time > Duration::from_millis(10) {
            let chip8_buffer = chip8.get_frame_buffer();
            for y in 0..SCREEN_HEIGHT {
                for x in 0..SCREEN_WIDTH {
                    let index = chip8.get_frame_index(x / 10, y / 10);
                    let pixel = chip8_buffer[index];

                    let color = if pixel == 1 { 0x00ff00 } else { 0x0 };
                    let offset = y * SCREEN_WIDTH + x;
                    buffer[offset] = color;
                }
            }

            window
                .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
                .unwrap();
            last_display_time = Instant::now();
        }
    }
}
