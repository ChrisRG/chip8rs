use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::{fs::File, io::Read};
use std::{thread, time::Duration};
use rodio::{OutputStream, Sink};

use crate::bus::Bus;
use crate::cpu::Cpu;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALAR: usize = 10;
const SCREEN_WIDTH: usize = WIDTH * SCALAR;
const SCREEN_HEIGHT: usize = HEIGHT * SCALAR;

pub struct Chip8 {
    bus: Bus,
    cpu: Cpu,
    canvas: Canvas<Window>,
    sdl_context: sdl2::Sdl,
    frame_buffer: [u8; WIDTH * HEIGHT],
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

        // Set up SDL
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let title = format!("CHIPRS - {}", rom_file);

        // Build window
        let window = if video_subsystem.num_video_displays().unwrap() > 1 {
            // Draw to external monitor if present, otherwise defaults to first
            let display2 = video_subsystem.display_bounds(1).unwrap();
            let x = display2.x + (display2.w/2 - SCREEN_WIDTH as i32 / 2);
            let y = display2.y + (display2.h/2 - SCREEN_HEIGHT as i32 / 2);

            video_subsystem.window(&title, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .position(x, y)
            .opengl()
            .build()
            .expect("Failed to initialize video subsystem")
        } else {
            video_subsystem.window(&title, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .position_centered()
            .opengl()
            .build()
            .expect("Failed to initialize video subsystem")
        };

        // Build canvas
        let mut canvas = window.into_canvas().build().expect("Failed to initialize canvas");
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.present();

        Chip8 {
            bus: Bus::new(),
            cpu: Cpu::new(&rom_buffer),
            canvas,
            sdl_context,
            frame_buffer: [0; WIDTH * HEIGHT],
        }
    }

    // Running Chiprs in the terminal opens an SDL2 window with a canvas
    pub fn run(&mut self) {

         let (_stream, stream_handle) = OutputStream::try_default().unwrap();
         let sink = Sink::try_new(&stream_handle).unwrap();
         let source = rodio::source::SineWave::new(400);
         sink.append(source);
         sink.pause();
        

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let mut clock = 0;

        // Main loop
        'running: loop {
            self.execute_cycle();
            clock += 1;
            // thread::sleep(Duration::new(0, 1_000_000_000u32 / 900));

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                    Event::KeyDown { keycode: key, .. } => self.set_key_pressed(key),
                    _ => {}
                }
            }

            // 55'ish Hz refresh rate
            if clock % 10 == 0 {
                self.cpu.update_timers();
            }


            if self.should_redraw() {
                self.update_framebuffer();
                self.update_display();
            }


            if self.should_beep() {
                sink.play();
            } else {
                sink.pause();
            }

        }
    }

    fn update_display(&mut self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                // Find correct index in one-dimensional frame buffer
                let index = (x/SCALAR) + WIDTH * (y/SCALAR);
                let pixel = self.frame_buffer[index];

                let color = if pixel == 1 { Color::RGB(0,255,0) } else { Color::RGB(0,0,0)};

                self.canvas.set_draw_color(color);
                let _ = self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, 10, 10));
            }
        }
        self.canvas.present();
    }

    fn execute_cycle(&mut self) {
        self.cpu.execute_cycle(&mut self.bus);
    }

    fn update_framebuffer(&mut self) {
        self.frame_buffer = *self.bus.display.get_frame_buffer();
    }

    fn set_key_pressed(&mut self, key: Option<Keycode>) {
        self.bus.set_key_pressed(key);
    }

    fn should_redraw(&self) -> bool {
        self.cpu.should_redraw()
    }

    fn should_beep(&self) -> bool {
        self.cpu.should_beep()
    }
}
