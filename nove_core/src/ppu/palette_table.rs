use crate::addresses::ppu::PALETTE_START;

const PALETTE_SIZE: usize = 32;

pub const MIRROR_UBG_COLOR: u16 = 0x3f10;
pub const MIRROR_UU_COLOR_1: u16 = 0x3f14;
pub const MIRROR_UU_COLOR_2: u16 = 0x3f18;
pub const MIRROR_UU_COLOR_3: u16 = 0x3f1c;

pub struct PaletteTable(pub(crate) [u8; PALETTE_SIZE]);

impl Default for PaletteTable {
    fn default() -> Self {
        Self([Default::default(); PALETTE_SIZE])
    }
}

impl PaletteTable {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            MIRROR_UBG_COLOR | MIRROR_UU_COLOR_1 | MIRROR_UU_COLOR_2 | MIRROR_UU_COLOR_3 => {
                self.read(addr - 0x10)
            }
            _ => self.0[(addr - PALETTE_START) as usize],
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            MIRROR_UBG_COLOR | MIRROR_UU_COLOR_1 | MIRROR_UU_COLOR_2 | MIRROR_UU_COLOR_3 => {
                self.write(addr - 0x10, val)
            }
            _ => self.0[(addr - PALETTE_START) as usize] = val,
        }
    }
}
