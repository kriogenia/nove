pub mod mnemonic;
pub mod addressing_mode;

use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::instruction::addressing_mode::AddressingMode;
use crate::instruction::addressing_mode::AddressingMode::*;
use crate::instruction::mnemonic::Mnemonic;
use crate::instruction::mnemonic::Mnemonic::*;

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
        OpCode::new(ADC, 0x6d, 3, 4, ABS),
        OpCode::new(ADC, 0x7d, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::new(ADC, 0x79, 3, 4, ABY), // +1 cycle if page crossed
        OpCode::new(ADC, 0x69, 2, 2, IMM),
        OpCode::new(ADC, 0x61, 2, 6, IDX),
        OpCode::new(ADC, 0x71, 2, 5, IDY), // +1 cycle if page crossed
        OpCode::new(ADC, 0x65, 2, 3, ZPG),
        OpCode::new(ADC, 0x75, 2, 4, ZPX),
        OpCode::new(AND, 0x2d, 3, 4, ABS),
        OpCode::new(AND, 0x3d, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::new(AND, 0x39, 3, 4, ABY), // +1 cycle if page crossed
        OpCode::new(AND, 0x29, 2, 2, IMM),
        OpCode::new(AND, 0x21, 2, 6, IDX),
        OpCode::new(AND, 0x31, 2, 5, IDY), // +1 cycle if page crossed
        OpCode::new(AND, 0x25, 2, 3, ZPG),
        OpCode::new(AND, 0x35, 2, 4, ZPX),
        OpCode::new(BRK, 0x00, 1, 7, IMP),
        OpCode::new(CLC, 0x18, 1, 2, IMP),
        OpCode::new(CLV, 0xb8, 1, 2, IMP),
        OpCode::new(CMP, 0xcd, 3, 4, ABS),
        OpCode::new(CMP, 0xdd, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::new(CMP, 0xd9, 3, 4, ABY), // +1 cycle if page crossed
        OpCode::new(CMP, 0xc9, 2, 2, IMM),
        OpCode::new(CMP, 0xc1, 2, 6, IDX),
        OpCode::new(CMP, 0xd1, 2, 5, IDY), // +1 cycle if page crossed
        OpCode::new(CMP, 0xc5, 2, 3, ZPG),
        OpCode::new(CMP, 0xd5, 2, 4, ZPX),
        OpCode::new(CPX, 0xec, 3, 4, ABS),
        OpCode::new(CPX, 0xe0, 2, 2, IMM),
        OpCode::new(CPX, 0xe4, 2, 3, ZPG),
        OpCode::new(CPY, 0xcc, 3, 4, ABS),
        OpCode::new(CPY, 0xc0, 2, 2, IMM),
        OpCode::new(CPY, 0xc4, 2, 3, ZPG),
        OpCode::new(DEX, 0xca, 1, 2, IMP),
        OpCode::new(INX, 0xe8, 1, 2, IMP),
        OpCode::new(LDA, 0xad, 3, 4, ABS),
        OpCode::new(LDA, 0xbd, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::new(LDA, 0xb9, 3, 4, ABY), // +1 cycle if page crossed
        OpCode::new(LDA, 0xa1, 2, 6, IDX),
        OpCode::new(LDA, 0xb1, 2, 5, IDY), // +1 cycle if page crossed
        OpCode::new(LDA, 0xa9, 2, 2, IMM),
        OpCode::new(LDA, 0xa5, 2, 3, ZPG),
        OpCode::new(LDA, 0xb5, 2, 4, ZPX),
        OpCode::new(LDX, 0xae, 3, 4, ABS),
        OpCode::new(LDX, 0xbe, 3, 4, ABY), // +1 cycle if page crossed
        OpCode::new(LDX, 0xa2, 2, 2, IMM),
        OpCode::new(LDX, 0xa6, 2, 3, ZPG),
        OpCode::new(LDX, 0xb6, 2, 4, ZPY),
        OpCode::new(LDY, 0xac, 3, 4, ABS),
        OpCode::new(LDY, 0xbc, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::new(LDY, 0xa0, 2, 2, IMM),
        OpCode::new(LDY, 0xa4, 2, 3, ZPG),
        OpCode::new(LDY, 0xb4, 2, 4, ZPX),
        OpCode::new(STA, 0x8d, 3, 4, ABS),
        OpCode::new(STA, 0x9d, 3, 5, ABX),
        OpCode::new(STA, 0x99, 3, 5, ABY),
        OpCode::new(STA, 0x81, 2, 6, IDX),
        OpCode::new(STA, 0x91, 2, 6, IDY),
        OpCode::new(STA, 0x85, 2, 3, ZPG),
        OpCode::new(STA, 0x95, 2, 4, ZPX),
        OpCode::new(TAX, 0xaa, 1, 2, IMP),
    ];


    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        CPU_OPCODES.iter().map(|c| (c.code, c)).collect()
    };
}
