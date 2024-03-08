pub mod ppu {
    pub const LIMIT: u16 = 0x3fff;

    pub const CTRL: u16 = 0x2000;
    pub const MASK: u16 = 0x2001;
    pub const STATUS: u16 = 0x2002;
    pub const OAM_ADDR: u16 = 0x2003;
    pub const OAM_DATA: u16 = 0x2004;
    pub const SCROLL: u16 = 0x2005;
    pub const ADDR: u16 = 0x2006;
    pub const DATA: u16 = 0x2007;

    pub const REGISTERS_START: u16 = 0x2008;
    pub const REGISTERS_MIRRORS_END: u16 = 0x3fff;

    pub const CHROM_START: u16 = 0x0000;
    pub const VRAM_START: u16 = 0x2000;
    pub const PALETTE_START: u16 = 0x3f00;
    pub const CHROM_END: u16 = VRAM_START - 1;
    pub const VRAM_END: u16 = 0x2fff;
}

pub mod rom {
    pub const PRG_ROM_START: u16 = 0x8000;
    pub const PRG_ROM_END: u16 = 0xffff;
}

pub mod ram {
    pub const START: u16 = 0x0000;
    pub const MIRRORS_END: u16 = 0x1fff;
}

pub const PC_START: u16 = 0xfffc;
