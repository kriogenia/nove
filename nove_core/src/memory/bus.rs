use crate::cartridge::Rom;
use crate::memory::{Memory, PRG_ROM_END_ADDR, PRG_ROM_START_ADDR};

const RAM_START_ADDR: u16 = 0x0000;
const RAM_MIRRORS_END_ADDR: u16 = 0x1fff;
const PPU_REGISTERS_START_ADDR: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END_ADDR: u16 = 0x3fff;
const VRAM_SIZE: usize = 2048;
const HALF_ROM_SIZE: usize = 0x4000;

pub struct Bus {
    vram: [u8; VRAM_SIZE],
    rom: Rom,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Self {
            vram: [Default::default(); VRAM_SIZE],
            rom,
        }
    }

    pub fn read_rom(&self, addr: u16) -> u8 {
        let mut addr = (addr - PRG_ROM_START_ADDR) as usize;
        if self.rom.prg_rom.len() == HALF_ROM_SIZE && addr >= HALF_ROM_SIZE {
            addr = addr % 0x4000;
        }
        self.rom.prg_rom[addr]
    }
}

impl Memory for Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            RAM_START_ADDR..=RAM_MIRRORS_END_ADDR => self.vram[addr as usize & 0b00000111_11111111],
            PPU_REGISTERS_START_ADDR..=PPU_REGISTERS_MIRRORS_END_ADDR => {
                todo!("PPU is not supported yet")
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
            PPU_REGISTERS_START_ADDR..=PPU_REGISTERS_MIRRORS_END_ADDR => {
                todo!("PPU is not supported yet")
            }
            PRG_ROM_START_ADDR..=PRG_ROM_END_ADDR => {
                panic!("Attempt to write to Cartridge ROM space")
            }
            _ => {}
        }
    }
}
