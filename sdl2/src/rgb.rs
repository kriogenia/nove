use crate::RGB_SPACE;
use nove_core::core::Frame;
use nove_core::{HEIGHT, WIDTH};

pub struct RgbFrame {
    pub data: Vec<u8>,
}

impl RgbFrame {
    pub fn new() -> Self {
        RgbFrame {
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

impl From<Frame> for RgbFrame {
    fn from(frame: Frame) -> Self {
        // todo implement into iter for frame to make buffer private
        Self {
            data: frame.buffer.iter().flat_map(to_rgb).collect(),
        }
    }
}

fn to_rgb(val: &u8) -> Vec<u8> {
    match val {
        0 => vec![0x00, 0xff, 0xff],
        1 => vec![0xff, 0xff, 0x00],
        2 => vec![0xff, 0x00, 0xff],
        3 => vec![0xff, 0xff, 0xff],
        _ => panic!("nes only supports for possible colors per pixel"),
    }
}
