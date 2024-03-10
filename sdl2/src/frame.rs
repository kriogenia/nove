use crate::RGB_SPACE;
use nove_core::{HEIGHT, WIDTH};

pub struct Frame {
    pub data: Vec<u8>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            data: vec![0; (WIDTH * HEIGHT * RGB_SPACE) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, rgb: (u8, u8, u8)) {
        let idx = (y * RGB_SPACE * WIDTH + x * RGB_SPACE) as usize;
        if idx + 2 < self.data.len() {
            self.data[idx] = rgb.0;
            self.data[idx + 1] = rgb.1;
            self.data[idx + 2] = rgb.2;
        }
    }
}
