use crate::cartridge::Mirroring;
use crate::ppu::address_register::AddressRegister;
use crate::ppu::control_register::ControlRegister;
use crate::ppu::mask_register::MaskRegister;
use crate::Program;

mod address_register;
mod control_register;
mod mask_register;

const PALETTE_SIZE: usize = 32;
const VRAM_SIZE: usize = 2048; // 2 KiB
const OAM_SIZE: usize = 256;
const NAMETABLE_SIZE: u16 = 1024; // 1KiB

pub struct Ppu {
    chrom: Program,
    pub ctrl: ControlRegister, // 0x2000
    pub mask: MaskRegister,    // 0x2001
    pub addr: AddressRegister, // 0x2006
    palette: [u8; PALETTE_SIZE],
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    mirroring: Mirroring,
    internal_data_buffer: u8,
}

impl Ppu {
    pub fn new(chrom: Program, mirroring: Mirroring) -> Self {
        Self {
            chrom,
            addr: Default::default(),
            mask: Default::default(),
            ctrl: Default::default(),
            palette: Default::default(),
            vram: [Default::default(); VRAM_SIZE],
            oam: [Default::default(); OAM_SIZE],
            mirroring,
            internal_data_buffer: Default::default(),
        }
    }

    pub fn read_data(&mut self) -> u8 {
        self.ctrl.vram_add_inc();
        let addr = self.addr.get();
        use crate::addresses::ppu::*;
        match addr {
            CHROM_START..=CHROM_END => self.read_and_store(self.chrom[addr as usize]),
            VRAM_START..=VRAM_END => {
                self.read_and_store(self.vram[self.mirror_vram(addr) as usize])
            }
            PALETTE_START..=LIMIT => todo!("read from palette"),
            _ => panic!("invalid access to mirrored space: {}", self.addr.get()),
        }
    }

    fn read_and_store(&mut self, val: u8) -> u8 {
        let prev = self.internal_data_buffer;
        self.internal_data_buffer = val;
        prev
    }

    fn mirror_vram(&self, addr: u16) -> u16 {
        use crate::addresses::ppu::{VRAM_END, VRAM_START};
        let vram = (addr & VRAM_END) - VRAM_START;
        let name_table = vram / NAMETABLE_SIZE;
        vram - match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => 2 * NAMETABLE_SIZE,
            (Mirroring::Horizontal, 1) | (Mirroring::Horizontal, 2) => NAMETABLE_SIZE,
            (Mirroring::Horizontal, 3) => 2 * NAMETABLE_SIZE,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cartridge::Mirroring;
    use crate::ppu::Ppu;
    use crate::RegWrite;

    #[test]
    fn read_chrom() {
        let mut ppu = Ppu::new(vec![0, 1, 2, 3], Mirroring::Horizontal);
        assert_read(&mut ppu, 0x00, 0x01, 1);
    }

    #[test]
    fn read_vram_horizontal() {
        let mut ppu = Ppu::new(vec![], Mirroring::Horizontal);
        ppu.vram[0x0002] = 3;
        ppu.vram[0x0020] = 4;
        ppu.vram[0x0402] = 5;
        ppu.vram[0x0420] = 6;

        assert_read(&mut ppu, 0x20, 0x02, 3);
        assert_read(&mut ppu, 0x24, 0x20, 4);
        assert_read(&mut ppu, 0x28, 0x02, 5);
        assert_read(&mut ppu, 0x2c, 0x20, 6);
    }

    #[test]
    fn read_vram_vertical() {
        let mut ppu = Ppu::new(vec![], Mirroring::Vertical);
        ppu.vram[0x0002] = 3;
        ppu.vram[0x0020] = 4;
        ppu.vram[0x0402] = 5;
        ppu.vram[0x0420] = 6;

        assert_read(&mut ppu, 0x20, 0x02, 3);
        assert_read(&mut ppu, 0x24, 0x02, 5);
        assert_read(&mut ppu, 0x28, 0x20, 4);
        assert_read(&mut ppu, 0x2c, 0x20, 6);
    }

    fn assert_read(ppu: &mut Ppu, hi: u8, lo: u8, val: u8) {
        ppu.addr.write(hi);
        ppu.addr.write(lo);
        assert_ne!(val, ppu.read_data());
        assert_eq!(val, ppu.read_data());
    }
}
