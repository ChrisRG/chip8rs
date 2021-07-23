use crate::display::Display;
use minifb::Key;

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

    pub fn set_key_pressed(&mut self, key: Option<Key>) {
        self.key_pressed = self.decode_key(key);
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        if let Some(key_pressed) = self.key_pressed {
            key == key_pressed
        } else {
            false
        }
    }

    fn decode_key(&self, key: Option<Key>) -> Option<u8> {
        return match key {
            Some(Key::Key1) => Some(0x1),
            Some(Key::Key2) => Some(0x2),
            Some(Key::Key3) => Some(0x3),
            Some(Key::Key4) => Some(0xC),

            Some(Key::Q) => Some(0x4),
            Some(Key::W) => Some(0x5),
            Some(Key::E) => Some(0x6),
            Some(Key::R) => Some(0xD),

            Some(Key::A) => Some(0x7),
            Some(Key::S) => Some(0x8),
            Some(Key::D) => Some(0x9),
            Some(Key::F) => Some(0xE),

            Some(Key::Z) => Some(0xA),
            Some(Key::X) => Some(0x0),
            Some(Key::C) => Some(0xB),
            Some(Key::V) => Some(0xF),
            _ => None,
        };
    }
}
