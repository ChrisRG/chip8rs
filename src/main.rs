mod bus;
mod chip8;
mod cpu;
mod disassembler;
mod display;
mod font;
mod ram;
use core::time;
use std::{
    thread,
    time::{Duration, Instant},
};

#[macro_use]
extern crate clap;
use clap::App;

use rodio::{OutputStream, Sink};

use minifb::{Key, KeyRepeat, Window, WindowOptions};

use crate::chip8::Chip8;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 320;
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let rom_file = matches.value_of("INPUT").unwrap().to_string();
    let mut chip8 = Chip8::new(rom_file.clone());

    if matches.is_present("disassemble") {
        chip8.disassemble(rom_file);
    }

    let mut window = Window::new(
        "CHIP8RS",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed: {:?}", e);
    });

    // Sound
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = rodio::source::SineWave::new(400);
    sink.append(source);
    sink.pause();

    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut last_key_update_time = Instant::now();
    let sleep_count = time::Duration::from_millis(1);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let key = check_key(window.get_keys_pressed(KeyRepeat::Yes));
        if key.is_some() || Instant::now() - last_key_update_time >= Duration::from_millis(200) {
            last_key_update_time = Instant::now();
            chip8.set_key_pressed(key);
        }

        chip8.execute_cycle();

        if chip8.should_redraw() {
            buffer = update_display(&chip8, &buffer);
            window
                .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
                .unwrap();
        }

        if chip8.should_beep() {
            sink.play();
        } else {
            sink.pause();
        }

        thread::sleep(sleep_count / 2);
    }
}

fn check_key(keys_pressed: Option<Vec<Key>>) -> Option<Key> {
    match keys_pressed {
        Some(keys) => {
            if !keys.is_empty() {
                Some(keys[0])
            } else {
                None
            }
        }
        None => None,
    }
}

fn update_display(chip8: &Chip8, in_buffer: &Vec<u32>) -> Vec<u32> {
    let mut buffer = in_buffer.clone();
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
    buffer
}
