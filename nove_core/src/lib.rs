pub mod addresses;
pub mod cartridge;
pub mod core;
mod exception;
mod flag_register;
pub mod instruction;
pub mod memory;
mod ppu;
mod register;

pub type Program = Vec<u8>;
