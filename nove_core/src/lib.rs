pub(crate) mod addresses;
pub mod cartridge;
pub mod core;
mod exception;
// todo move into register
mod flag_register;
mod frame;
pub(crate) mod instruction;
pub mod interrupt;
pub mod memory;
mod ppu;
mod register;

pub type Program = Vec<u8>;

pub const WIDTH: u32 = 256;
pub const HEIGHT: u32 = 240;
