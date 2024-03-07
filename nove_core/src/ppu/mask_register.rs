use crate::flag_register::FlagRegister;
use crate::ppu::ppu_register::RegWrite;
use std::fmt::{Debug, Formatter};

pub type MaskRegister = FlagRegister<MaskFlag>;

/*
   7  bit  0
   ---- ----
   BGRs bMmG
   |||| ||||
   |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
   |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
   |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
   |||| +---- 1: Show background
   |||+------ 1: Show sprites
   ||+------- Emphasize red (green on PAL/Dendy)
   |+-------- Emphasize green (red on PAL/Dendy)
   +--------- Emphasize blue
*/
#[derive(Default, Debug)]
pub enum MaskFlag {
    #[default]
    Greyscale = 0b0000_0001,
    ShowBGLeft = 0b0000_0010,
    ShowSpritesLeft = 0b0000_0100,
    ShowBG = 0b0000_1000,
    ShowSprites = 0b0001_0000,
    EmphasizeRed = 0b0010_0000,
    EmphasizeGreen = 0b0100_0000,
    EmphasizeBlue = 0b1000_0000,
}

impl From<MaskFlag> for u8 {
    fn from(value: MaskFlag) -> Self {
        value as u8
    }
}

impl RegWrite for MaskRegister {
    fn write(&mut self, val: u8) {
        self.set(val)
    }
}

impl Debug for MaskRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print(f, "BGRsbMmG")
    }
}
