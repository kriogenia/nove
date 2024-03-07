use crate::register::{RegRead, RegWrite, Register};

const OAM_SIZE: usize = 256;

pub struct Oam {
    pub addr: Register,
    data: [u8; OAM_SIZE],
}

impl RegWrite for Oam {
    fn write(&mut self, val: u8) {
        self.data[self.addr.read() as usize] = val;
        self.addr += 1
    }
}

impl RegRead for Oam {
    fn read(&self) -> u8 {
        self.data[self.addr.get() as usize]
    }
}

impl Default for Oam {
    fn default() -> Self {
        Self {
            addr: Default::default(),
            data: [Default::default(); OAM_SIZE],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ppu::oam::Oam;
    use crate::register::{RegRead, RegWrite};

    #[test]
    fn write_read() {
        let mut oam = Oam::default();
        oam.write(0x22);
        assert_eq!(oam.data[0x00], 0x22);
        assert_eq!(oam.read(), 0x00);
        assert_eq!(oam.addr.get(), 0x01);
        oam.addr.write(0x00);
        assert_eq!(oam.read(), 0x22);
    }
}
