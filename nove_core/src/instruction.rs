pub mod addressing_mode;
pub mod mnemonic;

use crate::instruction::addressing_mode::AddressingMode;
use crate::instruction::addressing_mode::AddressingMode::*;
use crate::instruction::mnemonic::Mnemonic;
use crate::instruction::mnemonic::Mnemonic::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct OpCode {
    pub mnemonic: Mnemonic,
    pub code: u8,
    pub bytes: u8,
    pub cycles: u8,
    pub addressing_mode: AddressingMode,
}

impl OpCode {
    fn new(
        mnemonic: Mnemonic,
        code: u8,
        bytes: u8,
        cycles: u8,
        addressing_mode: AddressingMode,
    ) -> Self {
        Self {
            mnemonic,
            code,
            bytes,
            cycles,
            addressing_mode,
        }
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

        OpCode::new(ASL, 0x0a, 1, 2, ACC),
        OpCode::new(ASL, 0x0e, 3, 6, ABS),
        OpCode::new(ASL, 0x1e, 3, 7, ABX),
        OpCode::new(ASL, 0x06, 2, 5, ZPG),
        OpCode::new(ASL, 0x16, 2, 6, ZPX),

        OpCode::new(BCC, 0x90, 2, 2, REL), // (+1 if branch succeeds, +2 if to a new page)

        OpCode::new(BCS, 0xb0, 2, 2, REL), // (+1 if branch succeeds, +2 if to a new page)

        OpCode::new(BEQ, 0xf0, 2, 2, REL), // (+1 if branch succeeds, +2 if to a new page)

        OpCode::new(BIT, 0x2c, 3, 4, ABS),
        OpCode::new(BIT, 0x24, 2, 3, ZPG),

        OpCode::new(BMI, 0x30, 2, 2, REL), // (+1 if branch succeeds, +2 if to a new page)

        OpCode::new(BNE, 0xd0, 2, 2, REL), // (+1 if branch succeeds, +2 if to a new page)

        OpCode::new(BPL, 0x10, 2, 2, REL), // (+1 if branch succeeds, +2 if to a new page)

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

        OpCode::new(EOR, 0x4d, 3, 4, ABS),
        OpCode::new(EOR, 0x5d, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::new(EOR, 0x59, 3, 4, ABY), // +1 cycle if page crossed
        OpCode::new(EOR, 0x49, 2, 2, IMM),
        OpCode::new(EOR, 0x41, 2, 6, IDX),
        OpCode::new(EOR, 0x51, 2, 5, IDY), // +1 cycle if page crossed
        OpCode::new(EOR, 0x45, 2, 3, ZPG),
        OpCode::new(EOR, 0x55, 2, 4, ZPX),

        OpCode::new(INC, 0xee, 3, 6, ABS),
        OpCode::new(INC, 0xfe, 3, 7, ABX),
        OpCode::new(INC, 0xe6, 2, 5, ZPG),
        OpCode::new(INC, 0xf6, 2, 6, ZPX),

        OpCode::new(INX, 0xe8, 1, 2, IMP),

        OpCode::new(JMP, 0x4c, 1, 3, ABS), // setting 1 byte to evade JMP to advance the pc
        OpCode::new(JMP, 0x6c, 1, 5, IND),

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

        OpCode::new(NOP, 0xea, 1, 2, IMP),

        OpCode::new(ORA, 0x0d, 3, 4, ABS),
        OpCode::new(ORA, 0x1d, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::new(ORA, 0x19, 3, 4, ABY), // +1 cycle if page crossed
        OpCode::new(ORA, 0x09, 2, 2, IMM),
        OpCode::new(ORA, 0x01, 2, 6, IDX),
        OpCode::new(ORA, 0x11, 2, 5, IDY), // +1 cycle if page crossed
        OpCode::new(ORA, 0x05, 2, 3, ZPG),
        OpCode::new(ORA, 0x15, 2, 4, ZPX),

        OpCode::new(PHA, 0x48, 1, 3, IMP),

        OpCode::new(PHP, 0x08, 1, 3, IMP),

        OpCode::new(PLA, 0x68, 1, 4, IMP),

        OpCode::new(PLP, 0x28, 1, 4, IMP),

        OpCode::new(ROL, 0x2a, 1, 2, ACC),
        OpCode::new(ROL, 0x2e, 3, 6, ABS),
        OpCode::new(ROL, 0x3e, 3, 7, ABX),
        OpCode::new(ROL, 0x26, 2, 5, ZPG),
        OpCode::new(ROL, 0x36, 2, 6, ZPX),

        OpCode::new(ROR, 0x6a, 1, 2, ACC),
        OpCode::new(ROR, 0x6e, 3, 6, ABS),
        OpCode::new(ROR, 0x7e, 3, 7, ABX),
        OpCode::new(ROR, 0x66, 2, 5, ZPG),
        OpCode::new(ROR, 0x76, 2, 6, ZPX),

        OpCode::new(SBC, 0xed, 3, 4, ABS),
        OpCode::new(SBC, 0xfd, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::new(SBC, 0xf9, 3, 4, ABY), // +1 cycle if page crossed
        OpCode::new(SBC, 0xe9, 2, 2, IMM),
        OpCode::new(SBC, 0xe1, 2, 6, IDX),
        OpCode::new(SBC, 0xf1, 2, 5, IDY), // +1 cycle if page crossed
        OpCode::new(SBC, 0xe5, 2, 3, ZPG),
        OpCode::new(SBC, 0xf5, 2, 4, ZPX),

        OpCode::new(SEC, 0x38, 1, 2, IMP),

        OpCode::new(SEI, 0x78, 1, 2, IMP),

        OpCode::new(STA, 0x8d, 3, 4, ABS),
        OpCode::new(STA, 0x9d, 3, 5, ABX),
        OpCode::new(STA, 0x99, 3, 5, ABY),
        OpCode::new(STA, 0x81, 2, 6, IDX),
        OpCode::new(STA, 0x91, 2, 6, IDY),
        OpCode::new(STA, 0x85, 2, 3, ZPG),
        OpCode::new(STA, 0x95, 2, 4, ZPX),

        OpCode::new(STX, 0x8e, 3, 4, ABS),
        OpCode::new(STX, 0x86, 2, 3, ZPG),
        OpCode::new(STX, 0x96, 2, 4, ZPY),

        OpCode::new(STY, 0x8c, 3, 4, ABS),
        OpCode::new(STY, 0x84, 2, 3, ZPG),
        OpCode::new(STY, 0x94, 2, 4, ZPX),

        OpCode::new(TAX, 0xaa, 1, 2, IMP),

        OpCode::new(TAY, 0xa8, 1, 2, IMP),

        OpCode::new(TXA, 0x8a, 1, 2, IMP),

        OpCode::new(TYA, 0x98, 1, 2, IMP),
    ];


    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        CPU_OPCODES.iter().map(|c| (c.code, c)).collect()
    };
}
