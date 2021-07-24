mod bus;
mod cpu;
mod display;
mod font;
mod ram;
mod test;
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

    // let mut file = File::open("./src/roms/pong.ch8").unwrap();
    // let mut data = Vec::<u8>::new();
    // file.read_to_end(&mut data).expect("File not found!");

    cpu.load_rom(test::TEST_E.to_vec());

    let mut window = Window::new("CHIP8RS", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("Window creation failed: {:?}", e);
        });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // window.update();
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

        let chip8_buffer = bus.display.get_frame_buffer();
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
        println!("{:?}", cpu);
        let millis = time::Duration::from_millis(100);

        thread::sleep(millis);
    }
}
