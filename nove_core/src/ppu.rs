use crate::addresses::ppu::{CHROM_END, CHROM_START, LIMIT, PALETTE_START, VRAM_END, VRAM_START};
use crate::cartridge::Mirroring;
use crate::interrupt::InterruptFlag;
use crate::ppu::address_register::AddressRegister;
use crate::ppu::controller_register::{ControlFlags, ControllerRegister};
use crate::ppu::mask_register::MaskRegister;
use crate::ppu::oam::Oam;
use crate::ppu::palette_table::PaletteTable;
use crate::ppu::scroll_register::ScrollRegister;
use crate::ppu::status_register::{PpuStatusFlag, StatusRegister};
use crate::ppu::tile_reader::TileReader;
use crate::register::{RegRead, RegWrite};
use crate::{Program, HEIGHT, WIDTH};
pub use frame::Frame;
use std::cell::RefCell;
use std::rc::Rc;

mod address_register;
mod controller_register;
mod frame;
mod mask_register;
mod oam;
mod palette_table;
mod scroll_register;
mod status_register;
mod tile_reader;

const VRAM_SIZE: usize = 2048; // 2 KiB
const NAMETABLE_SIZE: u16 = 1024; // 1KiB
const TILE_BANK_SIZE: u16 = 4096; // 4 KiB

const SCANLINE_CYCLES: usize = 341;
const NMI_SCANLINES: u16 = 241;
const SCANLINES_PER_FRAME: u16 = 262;

const TILE_WIDTH: u32 = 8;
const TILE_HEIGHT: u32 = 8;
const TILE_COLOR_SPACE: u32 = 2; // 2 bits to codify color
const TILE_BYTES_SIZE: u32 = (TILE_WIDTH * TILE_HEIGHT * TILE_COLOR_SPACE) / 8; // 128 bits, 16 B
const TILES_PER_ROW: u32 = WIDTH / TILE_WIDTH;
const TILES_PER_FRAME: u32 = TILES_PER_ROW * HEIGHT / TILE_HEIGHT;

pub struct Ppu {
    chr_rom: Program,
    pub ctrl: ControllerRegister, // 0x2000
    pub mask: MaskRegister,       // 0x2001
    pub status: StatusRegister,   // 0x2002
    pub oam: Oam,                 // 0x2003, 0x2004
    pub scroll: ScrollRegister,   // 0x2005
    pub addr: AddressRegister,    // 0x2006
    palette: PaletteTable,        // 0x3f00..0x3fff
    vram: [u8; VRAM_SIZE],
    mirroring: Mirroring,
    internal_data_buffer: u8,
    scanline: u16,
    cycles: usize,
    cpu_interrupt: Rc<RefCell<InterruptFlag>>,
}

impl Ppu {
    pub fn new(
        chr_rom: Program,
        mirroring: Mirroring,
        cpu_interrupt: Rc<RefCell<InterruptFlag>>,
    ) -> Self {
        Self {
            chr_rom,
            ctrl: Default::default(),
            mask: Default::default(),
            status: Default::default(),
            oam: Default::default(),
            scroll: Default::default(),
            addr: Default::default(),
            palette: Default::default(),
            vram: [Default::default(); VRAM_SIZE],
            mirroring,
            internal_data_buffer: Default::default(),
            scanline: Default::default(),
            cycles: Default::default(),
            cpu_interrupt,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.cycles += 1;
        if self.cycles == SCANLINE_CYCLES {
            self.cycles = 0;
            self.scanline += 1;

            if self.scanline == NMI_SCANLINES {
                // todo move to function (trigger_scanline)
                self.status.raise(PpuStatusFlag::VerticalBlankStarted);
                self.status.low(PpuStatusFlag::Sprite0Hit);
                if self.ctrl.is_raised(ControlFlags::GenerateNMI) {
                    self.nmi_interruption(true);
                }
                return false;
            }

            if self.scanline == SCANLINES_PER_FRAME {
                // todo move to function (attempt_close_scanline)
                self.scanline = 0;
                self.nmi_interruption(false);
                self.status.low(PpuStatusFlag::Sprite0Hit);
                self.status.low(PpuStatusFlag::VerticalBlankStarted);
                return true;
            }
        }
        return false;
    }

    pub fn render(&self) -> Frame {
        let mut frame = Frame::new();
        let bank_addr = self.ctrl.get_bit(ControlFlags::BGPatternAddr) as u16 * TILE_BANK_SIZE;

        for i in 0..TILES_PER_FRAME {
            let tile_idx = self.vram[i as usize] as u16;
            let tile_addr = (bank_addr + tile_idx * TILE_BYTES_SIZE as u16) as usize;
            let tile = &self.chr_rom[tile_addr..tile_addr + TILE_BYTES_SIZE as usize];

            let tile_values: Vec<u8> = TileReader::new(tile).collect();
            frame.set_tile(i % TILES_PER_ROW, i / TILES_PER_ROW, &tile_values);
        }
        frame
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.inc_vram_addr();
        use crate::addresses::ppu::*;
        match addr {
            CHROM_START..=CHROM_END => self.read_and_store(self.chr_rom[addr as usize]),
            VRAM_START..=VRAM_END => {
                self.read_and_store(self.vram[self.mirror_vram(addr) as usize])
            }
            PALETTE_START..=LIMIT => self.palette.read(addr),
            _ => panic!("invalid PPU read access to {}", self.addr.get()),
        }
    }

    pub fn read_status(&mut self) -> u8 {
        let val = self.status.read();
        self.status.low(PpuStatusFlag::VerticalBlankStarted);
        self.addr.reset();
        self.scroll.reset();
        val
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        let prev_gen_nmi = self.ctrl.is_raised(ControlFlags::GenerateNMI);
        self.ctrl.write(value);
        if !prev_gen_nmi & self.ctrl.is_raised(ControlFlags::GenerateNMI)
            && self.status.is_raised(PpuStatusFlag::VerticalBlankStarted)
        {
            self.nmi_interruption(true);
        }
    }

    pub fn write_to_data(&mut self, value: u8) {
        let addr = self.addr.get();
        match addr {
            CHROM_START..=CHROM_END => { /* ignore attempt to right on CHR ROM space */ }
            VRAM_START..=VRAM_END => self.vram[self.mirror_vram(addr) as usize] = value,
            PALETTE_START..=LIMIT => self.palette.write(addr, value),
            _ => panic!("invalid PPU write access to {}", addr),
        }
        self.inc_vram_addr();
    }

    fn read_and_store(&mut self, val: u8) -> u8 {
        let prev = self.internal_data_buffer;
        self.internal_data_buffer = val;
        prev
    }

    fn mirror_vram(&self, addr: u16) -> u16 {
        let vram = (addr & VRAM_END) - VRAM_START;
        let name_table = vram / NAMETABLE_SIZE;
        vram - match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => 2 * NAMETABLE_SIZE,
            (Mirroring::Horizontal, 1) | (Mirroring::Horizontal, 2) => NAMETABLE_SIZE,
            (Mirroring::Horizontal, 3) => 2 * NAMETABLE_SIZE,
            _ => 0,
        }
    }

    fn nmi_interruption(&mut self, trigger: bool) {
        let flag = if trigger {
            InterruptFlag::NMI
        } else {
            InterruptFlag::None
        };
        self.cpu_interrupt.replace(flag);
    }

    fn inc_vram_addr(&mut self) {
        self.addr.inc(self.ctrl.vram_add_inc());
    }

    #[cfg(test)]
    fn set_addr(&mut self, hi: u8, lo: u8) {
        self.addr.write(hi);
        self.addr.write(lo);
    }
}

#[cfg(test)]
mod test {
    use crate::cartridge::Mirroring;
    use crate::interrupt::InterruptFlag;
    use crate::ppu::controller_register::ControlFlags;
    use crate::ppu::{Ppu, NMI_SCANLINES, SCANLINE_CYCLES};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn read_chrom() {
        let mut ppu = Ppu::new(vec![0, 1, 2, 3], Mirroring::Horizontal, Default::default());
        assert_read(&mut ppu, 0x00, 0x01, 1);
    }

    #[test]
    fn read_vram_horizontal() {
        let mut ppu = preloaded_ppu(Mirroring::Horizontal);
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
        let mut ppu = preloaded_ppu(Mirroring::Vertical);
        assert_read(&mut ppu, 0x20, 0x02, 3);
        assert_read(&mut ppu, 0x24, 0x02, 5);
        assert_read(&mut ppu, 0x28, 0x20, 4);
        assert_read(&mut ppu, 0x2c, 0x20, 6);
    }

    #[test]
    fn read_palette() {
        let mut ppu = Ppu::new(vec![], Mirroring::Horizontal, Default::default());
        ppu.palette.0[0x12] = 0x34;
        ppu.palette.0[0x04] = 0x56;

        ppu.set_addr(0x3f, 0x12);
        assert_eq!(ppu.read_data(), 0x34);

        ppu.set_addr(0x3f, 0x14);
        assert_eq!(ppu.read_data(), 0x56);
    }

    #[test]
    fn write_vram_horizontal() {
        let mut ppu = preloaded_ppu(Mirroring::Horizontal);
        assert_write(&mut ppu, 0x20, 0x02, 1);
        assert_write(&mut ppu, 0x24, 0x20, 2);
        assert_write(&mut ppu, 0x28, 0x02, 3);
        assert_write(&mut ppu, 0x2c, 0x20, 4);
    }

    #[test]
    fn write_vram_vertical() {
        let mut ppu = preloaded_ppu(Mirroring::Vertical);
        assert_write(&mut ppu, 0x20, 0x02, 1);
        assert_write(&mut ppu, 0x24, 0x02, 2);
        assert_write(&mut ppu, 0x28, 0x20, 3);
        assert_write(&mut ppu, 0x2c, 0x20, 4);
    }

    #[test]
    fn write_palette() {
        let mut ppu = Ppu::new(vec![], Mirroring::Horizontal, Default::default());
        ppu.set_addr(0x3f, 0x13);
        ppu.write_to_data(0x12);
        assert_eq!(ppu.palette.0[0x13], 0x12);
        ppu.set_addr(0x3f, 0x18);
        ppu.write_to_data(0x89);
        assert_eq!(ppu.palette.0[0x08], 0x89);
        assert_ne!(ppu.palette.0[0x18], 0x89);
    }

    #[test]
    fn nmi_interrupt() {
        let interrupt: Rc<RefCell<InterruptFlag>> = Default::default();
        let mut ppu = Ppu::new(vec![], Mirroring::Horizontal, interrupt.clone());
        ppu.ctrl.raise(ControlFlags::GenerateNMI);

        assert_eq!(*interrupt.borrow(), InterruptFlag::None);
        for _ in 0..(SCANLINE_CYCLES * NMI_SCANLINES as usize) {
            ppu.tick();
        }
        assert_eq!(*interrupt.borrow(), InterruptFlag::NMI);
    }

    fn assert_read(ppu: &mut Ppu, hi: u8, lo: u8, val: u8) {
        ppu.set_addr(hi, lo);
        assert_ne!(val, ppu.read_data());
        assert_eq!(val, ppu.read_data());
    }

    fn assert_write(ppu: &mut Ppu, hi: u8, lo: u8, val: u8) {
        ppu.set_addr(hi, lo);
        ppu.write_to_data(val);
        ppu.set_addr(hi, lo);
        assert_ne!(ppu.read_data(), val);
        assert_eq!(ppu.read_data(), val);
    }

    fn preloaded_ppu(mirroring: Mirroring) -> Ppu {
        let mut ppu = Ppu::new(vec![], mirroring, Default::default());
        ppu.vram[0x0002] = 3;
        ppu.vram[0x0020] = 4;
        ppu.vram[0x0402] = 5;
        ppu.vram[0x0420] = 6;
        ppu
    }
}
