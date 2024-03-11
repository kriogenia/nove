use crate::ppu::{TILES_PER_ROW, TILE_HEIGHT, TILE_WIDTH};
use crate::{HEIGHT, WIDTH};

const BUFFER_SIZE: usize = (WIDTH * HEIGHT) as usize;
const TILE_SIZE: usize = (TILE_WIDTH * TILE_HEIGHT) as usize;

/// Stores the index of the color (value between 0 and 51)
pub struct Frame {
    pub buffer: [u8; BUFFER_SIZE],
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            buffer: [0; BUFFER_SIZE],
        }
    }

    pub fn set_tile(&mut self, x: u32, y: u32, values: &[u8]) {
        assert_eq!(values.len(), TILE_SIZE);
        let idx = (y * TILES_PER_ROW + x) as usize * TILE_SIZE;
        self.buffer[idx..idx + TILE_SIZE].copy_from_slice(&values[0..TILE_SIZE]);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, rgb: (u8, u8, u8)) {
        let idx = (y * WIDTH + x) as usize;
        if idx < BUFFER_SIZE {
            self.buffer[idx] = rgb.0;
            self.buffer[idx + 1] = rgb.1;
            self.buffer[idx + 2] = rgb.2;
        }
    }
}
