pub mod mnemomic;
mod addressing_mode;

use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::instruction::addressing_mode::AddressingMode;
use crate::instruction::addressing_mode::AddressingMode::*;
use crate::instruction::mnemomic::Mnemonic;
use crate::instruction::mnemomic::Mnemonic::*;

pub struct OpCode {
    pub mnemonic: Mnemonic,
    pub code: u8,
    pub bytes: u8,
    pub cycles: u8,
    pub addressing_mode: AddressingMode,
}

impl OpCode {
    fn new(mnemonic: Mnemonic, code: u8, bytes: u8, cycles: u8, addressing_mode: AddressingMode) -> Self {
        Self { mnemonic, code, bytes, cycles, addressing_mode }
    }
}

lazy_static! {
    pub static ref CPU_OPCODES: Vec<OpCode> = vec![
        OpCode::new(BRK, 0x00, 1, 7, IMP),
        OpCode::new(TAX, 0xAA, 1, 2, IMP),
        OpCode::new(LDA, 0xA9, 2, 2, IMM),
        OpCode::new(INX, 0xE8, 1, 2, IMP),
    ];


    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        CPU_OPCODES.iter().map(|c| (c.code, c)).collect()
    };
}
