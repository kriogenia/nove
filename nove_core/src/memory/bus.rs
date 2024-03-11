use crate::addresses::*;
use crate::cartridge::Rom;
use crate::interrupt::InterruptFlag;
use crate::memory::Memory;
use crate::ppu::Ppu;
use crate::register::{RegRead, RegWrite};
use crate::Program;
use std::cell::RefCell;
use std::rc::Rc;

const VRAM_SIZE: usize = 2048;
const HALF_ROM_SIZE: usize = 0x4000;

const PPU_CYCLES_PER_CPU: u8 = 3;

pub struct Bus {
    vram: [u8; VRAM_SIZE],
    prg_rom: Program,
    pub(crate) ppu: RefCell<Ppu>,
}

impl Bus {
    pub fn new(rom: Rom, cpu_interrupt: Rc<RefCell<InterruptFlag>>) -> Self {
        let ppu = Ppu::new(rom.chr_rom, rom.screen_mirroring, cpu_interrupt);
        Self {
            vram: [Default::default(); VRAM_SIZE],
            prg_rom: rom.prg_rom,
            ppu: RefCell::new(ppu),
        }
    }

    pub fn read_rom(&self, addr: u16) -> u8 {
        let mut addr = (addr - rom::PRG_ROM_START) as usize;
        if self.prg_rom.len() == HALF_ROM_SIZE && addr >= HALF_ROM_SIZE {
            addr %= HALF_ROM_SIZE;
        }
        self.prg_rom[addr]
    }
}

impl Memory for Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            ram::START..=ram::MIRRORS_END => self.vram[addr as usize & 0b00000111_11111111],
            ppu::STATUS => self.ppu.borrow_mut().read_status(),
            ppu::OAM_DATA => self.ppu.borrow().oam.read(),
            ppu::DATA => self.ppu.borrow_mut().read_data(),
            ppu::REGISTERS_START..=ppu::REGISTERS_MIRRORS_END => self.read(addr & ppu::DATA),
            rom::PRG_ROM_START..=rom::PRG_ROM_END => self.read_rom(addr),
            ppu::CTRL | ppu::MASK | ppu::OAM_ADDR | ppu::SCROLL | ppu::ADDR | ppu::OAM_DMA => {
                panic!("invalid attempt to read from write-only PPU address {addr:x}");
            }
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            ram::START..=ram::MIRRORS_END => self.vram[addr as usize & 0b11111111111] = value,
            ppu::CTRL => self.ppu.borrow_mut().write_to_ctrl(value),
            ppu::MASK => self.ppu.borrow_mut().mask.write(value),
            ppu::OAM_ADDR => self.ppu.borrow_mut().oam.addr.write(value),
            ppu::OAM_DATA => self.ppu.borrow_mut().oam.write(value),
            ppu::SCROLL => self.ppu.borrow_mut().scroll.write(value),
            ppu::ADDR => self.ppu.borrow_mut().addr.write(value),
            ppu::DATA => self.ppu.borrow_mut().write_to_data(value),
            ppu::REGISTERS_START..=ppu::REGISTERS_MIRRORS_END => {
                self.write(addr & ppu::DATA, value)
            }
            ppu::STATUS | rom::PRG_ROM_START..=rom::PRG_ROM_END => {
                panic!("invalid attempt to write to read-only PPU address {addr:x}");
            }
            _ => {}
        }
    }

    fn tick(&mut self, cpu_cycles: u8) {
        for _ in 0..(cpu_cycles * PPU_CYCLES_PER_CPU) {
            self.ppu.borrow_mut().tick();
        }
    }
}
