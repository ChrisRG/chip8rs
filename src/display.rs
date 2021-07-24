// CHIP-8 graphics:
// 64x32 display, each pixel can be only on or off
// Setting of pixels accomplished through sprites,
// which are always 8 X N (N is pixel height).
// Font set sprites: characters 0-9 and A-F
// to be printed directly within 8x5 grid.

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    frame_buffer: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            frame_buffer: [0; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.frame_buffer.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        x + WIDTH * y
    }

    // Set/unset pixels in display_buffer, return true/false if collision detected
    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        // Sprite height can be between 1 and 15 bytes, sprite width is 8 bits
        // Bit pattern shows which pixel to set/unset using XOR
        // loop by byte/row: 0..sprite.len()
        // loop by bit/col: 0..7
        // val = (row >> 7 - bit) & 0x01
        // 00000000
        // 0
        let mut collision = false;

        return collision;
    }

    pub fn get_frame_buffer(&self) -> &[u8] {
        &self.frame_buffer
    }
}
