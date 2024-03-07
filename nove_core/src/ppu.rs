use crate::cartridge::Mirroring;
use crate::ppu::address_register::AddressRegister;
use crate::ppu::control_register::ControlRegister;
use crate::Program;

mod address_register;
mod control_register;

const PALETTE_SIZE: usize = 32;
const VRAM_SIZE: usize = 2048; // 2 KiB
const OAM_SIZE: usize = 256;

const LIMIT_ADDR: u16 = 0x3fff;

pub struct PPU {
    chrom: Program,
    addr: AddressRegister,
    ctrl: ControlRegister,
    palette: [u8; PALETTE_SIZE],
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    mirroring: Mirroring,
    internal_data_bufffer: u8,
}

impl PPU {
    pub fn new(chrom: Program, mirroring: Mirroring) -> Self {
        Self {
            chrom,
            addr: Default::default(),
            ctrl: Default::default(),
            palette: Default::default(),
            vram: [Default::default(); VRAM_SIZE],
            oam: [Default::default(); OAM_SIZE],
            mirroring,
            internal_data_bufffer: Default::default(),
        }
    }

    pub fn read_data(&mut self) -> u8 {
        self.ctrl.vram_add_inc();
        let addr = self.addr.get();
        match addr {
            0x0000..=0x1fff => self.read_and_store(self.chrom[addr as usize]),
            0x2000..=0x2fff => todo!("read from ram"),
            0x3000..=0x3eff => panic!("invalid access to address {}", self.addr.get()),
            0x3f00..=LIMIT_ADDR => todo!("read from palette"),
            _ => panic!("invalid access to mirrored space: {}", self.addr.get()),
        }
    }

    fn read_and_store(&mut self, val: u8) -> u8 {
        let prev = self.internal_data_bufffer;
        self.internal_data_bufffer = val;
        prev
    }

    fn write_addr(&mut self, val: u8) {
        self.addr.update(val);
    }

    fn write_ctrl(&mut self, val: u8) {
        self.ctrl.set(val)
    }
}

#[cfg(test)]
mod test {
    use crate::cartridge::Mirroring;
    use crate::ppu::PPU;

    #[test]
    fn read_chrom() {
        let mut ppu = PPU::new(vec![0, 1, 2, 3], Mirroring::Horizontal);
        ppu.write_addr(0x00);
        ppu.write_addr(0x01);
        assert_ne!(1, ppu.read_data());
        assert_eq!(1, ppu.read_data());
    }
}
