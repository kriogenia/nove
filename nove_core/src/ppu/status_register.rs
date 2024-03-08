use crate::flag_register::FlagRegister;
use crate::register::RegRead;
use std::fmt::{Debug, Formatter};

pub type StatusRegister = FlagRegister<PpuStatusFlag>;

/*
   7  bit  0
   ---- ----
   VSO. ....
   |||| ||||
   |||+-++++- PPU open bus. Returns stale PPU bus contents.
   ||+------- Sprite overflow. The intent was for this flag to be set
   ||         whenever more than eight sprites appear on a scanline, but a
   ||         hardware bug causes the actual behavior to be more complicated
   ||         and generate false positives as well as false negatives; see
   ||         PPU sprite evaluation. This flag is set during sprite
   ||         evaluation and cleared at dot 1 (the second dot) of the
   ||         pre-render line.
   |+-------- Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
   |          a nonzero background pixel; cleared at dot 1 of the pre-render
   |          line.  Used for raster timing.
   +--------- Vertical blank has started (0: not in vblank; 1: in vblank).
              Set at dot 1 of line 241 (the line *after* the post-render
              line); cleared after reading $2002 and at dot 1 of the
              pre-render line.
*/
#[derive(Default)]
pub enum PpuStatusFlag {
    OpenBus = 0b0001_1111,
    #[default]
    SpriteOV = 0b0010_0000,
    Sprite0Hit = 0b0100_0000,
    VerticalBlankStarted = 0b1000_0000,
}

impl From<PpuStatusFlag> for u8 {
    fn from(value: PpuStatusFlag) -> Self {
        value as u8
    }
}

impl RegRead for StatusRegister {
    fn read(&self) -> u8 {
        self.0
    }
}

impl Debug for StatusRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print(f, "VSO_____")
    }
}
