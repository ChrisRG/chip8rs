use crate::display::Display;
use sdl2::keyboard::Keycode;

pub struct Bus {
    pub display: Display,
    pub key_pressed: Option<u8>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            display: Display::new(),
            key_pressed: None,
        }
    }

    pub fn set_key_pressed(&mut self, key: Option<Keycode>) {
        self.key_pressed = self.decode_key(key);
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        if let Some(key_pressed) = self.key_pressed {
            key == key_pressed
        } else {
            false
        }
    }

    fn decode_key(&self, key: Option<Keycode>) -> Option<u8> {
        match key {
            Some(Keycode::Num1) => Some(0x1),
            Some(Keycode::Num2) => Some(0x2),
            Some(Keycode::Num3) => Some(0x3),
            Some(Keycode::Num4) => Some(0xC),

            Some(Keycode::Q) => Some(0x4),
            Some(Keycode::W) => Some(0x5),
            Some(Keycode::E) => Some(0x6),
            Some(Keycode::R) => Some(0xD),

            Some(Keycode::A) => Some(0x7),
            Some(Keycode::S) => Some(0x8),
            Some(Keycode::D) => Some(0x9),
            Some(Keycode::F) => Some(0xE),

            Some(Keycode::Z) => Some(0xA),
            Some(Keycode::X) => Some(0x0),
            Some(Keycode::C) => Some(0xB),
            Some(Keycode::V) => Some(0xF),
            _ => None,
        }
    }
}
