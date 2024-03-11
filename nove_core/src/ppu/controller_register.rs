use crate::flag_register::FlagRegister;
use crate::register::RegWrite;
use log::debug;
use std::fmt::{Debug, Formatter};

/*
   7  bit  0
   ---- ----
   VPHB SINN
   |||| ||||
   |||| ||++- Base nametable address
   |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
   |||| |+--- VRAM address increment per CPU read/write of PPUDATA
   |||| |     (0: add 1, going across; 1: add 32, going down)
   |||| +---- Sprite pattern table address for 8x8 sprites
   ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
   |||+------ Background pattern table address (0: $0000; 1: $1000)
   ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
   |+-------- PPU master/slave select
   |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
   +--------- Generate an NMI at the start of the
              vertical blanking interval (0: off; 1: on)
*/
#[derive(Debug, Default)]
pub enum ControlFlags {
    Nametable = 0b0000_0011,
    #[default]
    VramAddrIncrement = 0b0000_0100,
    SpritePatternAddr = 0b0000_1000,
    BGPatternAddr = 0b0001_0000,
    SpriteSize = 0b0010_0000,
    MasterSlave = 0b0100_0000,
    GenerateNMI = 0b1000_0000,
}

impl From<ControlFlags> for u8 {
    fn from(value: ControlFlags) -> Self {
        value as u8
    }
}

pub type ControllerRegister = FlagRegister<ControlFlags>;

impl ControllerRegister {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn vram_add_inc(&mut self) -> u8 {
        if self.is_raised(ControlFlags::VramAddrIncrement) {
            32 // going down
        } else {
            1 // going acrss
        }
    }
}

impl RegWrite for ControllerRegister {
    fn write(&mut self, val: u8) {
        self.set(val);
        debug!("{self:?}");
    }
}

impl Debug for ControllerRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print(f, "ctrl")
    }
}
