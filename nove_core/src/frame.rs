use crate::{HEIGHT, WIDTH};

const BUFFER_SIZE: usize = (WIDTH * HEIGHT) as usize;

pub struct Frame {
    pub buffer: [u8; BUFFER_SIZE],
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            buffer: [0; BUFFER_SIZE],
        }
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
