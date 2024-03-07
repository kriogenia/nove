use crate::cartridge::Mirroring;
use crate::ppu::address_register::AddressRegister;
use crate::ppu::control_register::ControlRegister;
use crate::Program;

mod address_register;
mod control_register;

const PALETTE_SIZE: usize = 32;
const VRAM_SIZE: usize = 2048; // 2 KiB
const OAM_SIZE: usize = 256;
const NAMETABLE_SIZE: u16 = 1024; // 1KiB

const LIMIT_ADDR: u16 = 0x3fff;

const CHROM_START_ADDR: u16 = 0x0000;
const VRAM_START_ADDR: u16 = 0x2000;
const PALETTE_START_ADDR: u16 = 0x3f00;
const CHROM_END_ADDR: u16 = VRAM_START_ADDR - 1;
const VRAM_END_ADDR: u16 = 0x2fff;

pub struct PPU {
    chrom: Program,
    addr: AddressRegister,
    ctrl: ControlRegister,
    palette: [u8; PALETTE_SIZE],
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    mirroring: Mirroring,
    internal_data_buffer: u8,
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
            internal_data_buffer: Default::default(),
        }
    }

    pub fn read_data(&mut self) -> u8 {
        self.ctrl.vram_add_inc();
        let addr = self.addr.get();
        match addr {
            CHROM_START_ADDR..=CHROM_END_ADDR => self.read_and_store(self.chrom[addr as usize]),
            VRAM_START_ADDR..=VRAM_END_ADDR => {
                self.read_and_store(self.vram[self.mirror_vram_addr(addr) as usize])
            }
            PALETTE_START_ADDR..=LIMIT_ADDR => todo!("read from palette"),
            _ => panic!("invalid access to mirrored space: {}", self.addr.get()),
        }
    }

    fn read_and_store(&mut self, val: u8) -> u8 {
        let prev = self.internal_data_buffer;
        self.internal_data_buffer = val;
        prev
    }

    fn write_addr(&mut self, val: u8) {
        self.addr.update(val);
    }

    fn write_ctrl(&mut self, val: u8) {
        self.ctrl.set(val)
    }

    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let vram_addr = (addr & VRAM_END_ADDR) - VRAM_START_ADDR;
        let name_table = vram_addr / NAMETABLE_SIZE;
        vram_addr
            - match (&self.mirroring, name_table) {
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
    use crate::ppu::PPU;

    #[test]
    fn read_chrom() {
        let mut ppu = PPU::new(vec![0, 1, 2, 3], Mirroring::Horizontal);
        assert_read(&mut ppu, 0x00, 0x01, 1);
    }

    #[test]
    fn read_vram_horizontal() {
        let mut ppu = PPU::new(vec![], Mirroring::Horizontal);
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
        let mut ppu = PPU::new(vec![], Mirroring::Vertical);
        ppu.vram[0x0002] = 3;
        ppu.vram[0x0020] = 4;
        ppu.vram[0x0402] = 5;
        ppu.vram[0x0420] = 6;

        assert_read(&mut ppu, 0x20, 0x02, 3);
        assert_read(&mut ppu, 0x24, 0x02, 5);
        assert_read(&mut ppu, 0x28, 0x20, 4);
        assert_read(&mut ppu, 0x2c, 0x20, 6);
    }

    fn assert_read(ppu: &mut PPU, hi: u8, lo: u8, val: u8) {
        ppu.write_addr(hi);
        ppu.write_addr(lo);
        assert_ne!(val, ppu.read_data());
        assert_eq!(val, ppu.read_data());
    }
}
