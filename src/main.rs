mod bus;
mod cpu;
mod display;
mod ram;
use std::{thread, time};

use std::{fs::File, io::Read};

use bus::Bus;
use cpu::Cpu;

use minifb::{Key, KeyRepeat, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
fn main() {
    println!("Starting CHIP-8 emulator...");
    let mut bus = Bus::new();
    let mut cpu = Cpu::new();
    // Load game into memory
    // let test_rom = vec![0xE0, 0x9E, 0xE1, 0x9E, 0x12, 0x00];
    let mut file = File::open("./src/roms/pong.ch8").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");
    cpu.load_rom(data);

    let mut window = Window::new("CHIP8RS", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("Window creation failed: {:?}", e);
        });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // fun_name(&mut window);

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

        let chip8_buffer = bus.display.get_display_buffer();
        for y in 0..HEIGHT {
            let y_coord = y / 10;
            let offset = y * WIDTH;
            for x in 0..WIDTH {
                let index = bus.display.get_index(x / 10, y_coord);
                let pixel = chip8_buffer[index];
                let color_pixel = match pixel {
                    0 => 0x0,
                    1 => 0xffffff,
                    _ => unreachable!(),
                };
                buffer[offset + x] = color_pixel;
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT);

        cpu.execute_cycle(&bus);
        let millis = time::Duration::from_millis(100);

        thread::sleep(millis);
    }
}

fn fun_name(window: &mut Window) {
    window.update();
}
