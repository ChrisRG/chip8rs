// use core::time;
use std::{fs::File, io::Read};
use std::{thread, time::Duration};

use rodio::{OutputStream, Sink};

use minifb::{Key, KeyRepeat, Window, WindowOptions};

use crate::bus::Bus;
use crate::cpu::Cpu;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 320;

pub struct Chip8 {
    bus: Bus,
    cpu: Cpu,
}

impl Chip8 {
    pub fn new(rom_file: String) -> Chip8 {
        let mut rom_buffer = Vec::<u8>::new();
        let mut file = File::open(&rom_file).expect("File not found");

        if let Ok(bytes_read) = file.read_to_end(&mut rom_buffer) {
            println!("{} bytes loaded", bytes_read);
        } else {
            println!("Error loading ROM");
        };

        Chip8 {
            bus: Bus::new(),
            cpu: Cpu::new(&rom_buffer),
        }
    }

    pub fn run(&mut self) {
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

        let mut clock = 0;

        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.execute_cycle();

            clock += 1;

            // 60Hz refresh rate
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));

            let key = self.check_key(window.get_keys_pressed(KeyRepeat::Yes));
            if key.is_some() {
                self.set_key_pressed(key);
            }

            if clock % 10 == 0 {
                self.cpu.update_timers();
            }
            if self.should_redraw() {
                buffer = self.update_display(&buffer);
                window
                    .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
                    .unwrap();
            }
            if self.should_beep() {
                sink.play();
            } else {
                sink.pause();
            }
        }
    }

    fn check_key(&self, keys_pressed: Option<Vec<Key>>) -> Option<Key> {
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

    fn update_display(&self, in_buffer: &Vec<u32>) -> Vec<u32> {
        let mut buffer = in_buffer.clone();
        let chip8_buffer = self.get_frame_buffer();
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = self.get_frame_index(x / 10, y / 10);
                let pixel = chip8_buffer[index];

                let color = if pixel == 1 { 0x00ff00 } else { 0x0 };
                let offset = y * SCREEN_WIDTH + x;
                buffer[offset] = color;
            }
        }
        buffer
    }

    fn execute_cycle(&mut self) {
        self.cpu.execute_cycle(&mut self.bus);
    }

    fn get_frame_buffer(&self) -> &[u8] {
        self.bus.display.get_frame_buffer()
    }

    fn get_frame_index(&self, x: usize, y: usize) -> usize {
        self.bus.display.get_index(x, y)
    }

    fn set_key_pressed(&mut self, key: Option<Key>) {
        self.bus.set_key_pressed(key);
    }

    fn should_redraw(&self) -> bool {
        self.cpu.should_redraw()
    }

    fn should_beep(&self) -> bool {
        self.cpu.should_beep()
    }
}
