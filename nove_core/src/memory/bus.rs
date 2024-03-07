use crate::cartridge::Rom;
use crate::memory::{Memory, PRG_ROM_END_ADDR, PRG_ROM_START_ADDR};
use crate::ppu::PPU;
use crate::Program;
use std::cell::RefCell;

const RAM_START_ADDR: u16 = 0x0000;
const RAM_MIRRORS_END_ADDR: u16 = 0x1fff;
const PPU_CTRL_ADDR: u16 = 0x2000;
const PPU_ADDR_ADDR: u16 = 0x2006;
const PPU_DATA_ADDR: u16 = 0x2007;
const PPU_REGISTERS_START_ADDR: u16 = 0x2008;
const PPU_REGISTERS_MIRRORS_END_ADDR: u16 = 0x3fff;
const VRAM_SIZE: usize = 2048;
const HALF_ROM_SIZE: usize = 0x4000;

pub struct Bus {
    vram: [u8; VRAM_SIZE],
    prg_rom: Program,
    ppu: RefCell<PPU>,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        let ppu = PPU::new(rom.chr_rom, rom.screen_mirroring);
        Self {
            vram: [Default::default(); VRAM_SIZE],
            prg_rom: rom.prg_rom,
            ppu: RefCell::new(ppu),
        }
    }

    pub fn read_rom(&self, addr: u16) -> u8 {
        let mut addr = (addr - PRG_ROM_START_ADDR) as usize;
        if self.prg_rom.len() == HALF_ROM_SIZE && addr >= HALF_ROM_SIZE {
            addr %= 0x4000;
        }
        self.prg_rom[addr]
    }
}

impl Memory for Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            RAM_START_ADDR..=RAM_MIRRORS_END_ADDR => self.vram[addr as usize & 0b00000111_11111111],
            PPU_CTRL_ADDR | 0x2001 | 0x2003 | 0x2005 | PPU_ADDR_ADDR | 0x4014 => {
                panic!("invalid attempt to read from write-only PPU address {addr:x}");
            }
            PPU_DATA_ADDR => self.ppu.borrow_mut().read_data(),
            PPU_REGISTERS_START_ADDR..=PPU_REGISTERS_MIRRORS_END_ADDR => {
                self.read(addr & PPU_DATA_ADDR)
            }
            PRG_ROM_START_ADDR..=PRG_ROM_END_ADDR => self.read_rom(addr),
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            RAM_START_ADDR..=RAM_MIRRORS_END_ADDR => {
                self.vram[addr as usize & 0b11111111111] = value
            }
            PPU_CTRL_ADDR => self.ppu.borrow_mut().write_ctrl(value),
            PPU_ADDR_ADDR => self.ppu.borrow_mut().write_addr(value),
            PPU_DATA_ADDR => todo!("write to ppu data"),
            PPU_REGISTERS_START_ADDR..=PPU_REGISTERS_MIRRORS_END_ADDR => {
                todo!("PPU is not supported yet")
            }
            PRG_ROM_START_ADDR..=PRG_ROM_END_ADDR => {
                panic!("attempt to write to cartridge ROM space")
            }
            _ => {}
        }
    }
}
