use crate::ppu::{TILE_HEIGHT, TILE_WIDTH};

/// Handles the reading of a tile to extract the color value from up to bottom and left to right
pub struct TileReader<'a> {
    tile: &'a [u8],
    x: usize,
    y: usize,
    upper: u8,
    lower: u8,
}

impl<'a> TileReader<'a> {
    pub fn new(tile: &'a [u8]) -> Self {
        Self {
            tile,
            x: 0,
            y: 0,
            upper: tile[0],
            lower: tile[TILE_HEIGHT as usize],
        }
    }
}

impl<'a> Iterator for TileReader<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == TILE_WIDTH as usize {
            self.x = 0;
            self.y += 1;

            if self.y == TILE_HEIGHT as usize {
                return None;
            }

            self.upper = self.tile[self.y];
            self.lower = self.tile[self.y + TILE_HEIGHT as usize];
        }

        let val = (self.upper & 0b1000_0000) >> 6 | (self.lower & 0b1000_0000) >> 7;

        self.x += 1;
        self.upper <<= 1;
        self.lower <<= 1;
        Some(val)
    }
}

#[cfg(test)]
mod test {
    use crate::ppu::tile_reader::TileReader;

    #[test]
    fn tile_reader() {
        let tile = vec![
            // upper
            0b0000_0000,
            0b0101_0101,
            0b0100_1001,
            0b0001_0001,
            0b0010_0001,
            0b0100_0001,
            0b1000_0001,
            0b1111_1111,
            // lower
            0b1010_1010,
            0b0010_0100,
            0b1000_1000,
            0b0001_0000,
            0b0010_0000,
            0b0100_0000,
            0b1000_0000,
            0b0000_1111,
        ];

        let expected: Vec<u8> = vec![
            0b01, 0b00, 0b01, 0b00, 0b01, 0b00, 0b01, 0b00, //
            0b00, 0b10, 0b01, 0b10, 0b00, 0b11, 0b00, 0b10, //
            0b01, 0b10, 0b00, 0b00, 0b11, 0b00, 0b00, 0b10, //
            0b00, 0b00, 0b00, 0b11, 0b00, 0b00, 0b00, 0b10, //
            0b00, 0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b10, //
            0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b00, 0b10, //
            0b11, 0b00, 0b00, 0b00, 0b00, 0b00, 0b00, 0b10, //
            0b10, 0b10, 0b10, 0b10, 0b11, 0b11, 0b11, 0b11, //
        ];

        let mut tile_reader = TileReader::new(&tile);
        for expected in expected.into_iter() {
            assert_eq!(tile_reader.next().unwrap(), expected);
        }
    }
}
