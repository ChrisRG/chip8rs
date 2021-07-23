// CHIP-8 graphics:
// 64x32 display, each pixel can be only on or off
// Setting of pixels accomplished through sprites,
// which are always 8 X N (N is pixel height).

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.screen.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn get_index(&self, coord_x: usize, coord_y: usize) -> usize {
        coord_y * WIDTH + coord_x
    }
    pub fn draw(&mut self, x: usize, y: usize, byte: u8) {
        let mut coord_x = x as usize;
        let mut coord_y = y as usize;
        let mut b = byte;

        for _ in 0..8 {
            coord_x %= WIDTH;
            coord_y %= HEIGHT;
            let index = self.get_index(coord_x, coord_y);
            let bit = (b & 0b1000_0000) >> 7;
            self.screen[index] ^= bit;

            coord_x += 1;
            b <<= 1;
        }
    }

    pub fn get_display_buffer(&self) -> &[u8] {
        &self.screen
    }
}
