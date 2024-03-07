use crate::addresses::*;
use crate::cartridge::Rom;
use crate::memory::Memory;
use crate::ppu::PPU;
use crate::Program;
use std::cell::RefCell;

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
        let mut addr = (addr - rom::PRG_ROM_START) as usize;
        if self.prg_rom.len() == HALF_ROM_SIZE && addr >= HALF_ROM_SIZE {
            addr %= 0x4000;
        }
        self.prg_rom[addr]
    }
}

impl Memory for Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            ram::START..=ram::MIRRORS_END => self.vram[addr as usize & 0b00000111_11111111],
            ppu::CTRL | 0x2001 | 0x2003 | 0x2005 | ppu::ADDR | 0x4014 => {
                panic!("invalid attempt to read from write-only PPU address {addr:x}");
            }
            ppu::DATA => self.ppu.borrow_mut().read_data(),
            ppu::REGISTERS_START..=ppu::REGISTERS_MIRRORS_END => self.read(addr & ppu::DATA),
            rom::PRG_ROM_START..=rom::PRG_ROM_END => self.read_rom(addr),
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            ram::START..=ram::MIRRORS_END => self.vram[addr as usize & 0b11111111111] = value,
            ppu::CTRL => self.ppu.borrow_mut().write_ctrl(value),
            ppu::ADDR => self.ppu.borrow_mut().write_addr(value),
            ppu::DATA => todo!("write to ppu data"),
            ppu::REGISTERS_START..=ppu::REGISTERS_MIRRORS_END => {
                todo!("PPU is not supported yet")
            }
            rom::PRG_ROM_START..=rom::PRG_ROM_END => {
                panic!("attempt to write to cartridge ROM space")
            }
            _ => {}
        }
    }
}
