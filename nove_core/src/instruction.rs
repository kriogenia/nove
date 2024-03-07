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
    pub unofficial: bool,
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
            unofficial: false,
        }
    }

    fn unofficial(
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
            unofficial: true,
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

        OpCode::new(BVC, 0x50, 2, 2, REL), // (+1 if branch succeeds, +2 if to a new page)

        OpCode::new(BVS, 0x70, 2, 2, REL), // (+1 if branch succeeds, +2 if to a new page)

        OpCode::new(BRK, 0x00, 1, 7, IMP),

        OpCode::new(CLC, 0x18, 1, 2, IMP),

        OpCode::new(CLD, 0xd8, 1, 2, IMP),

        OpCode::new(CLI, 0x58, 1, 2, IMP),

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

        OpCode::unofficial(DCP, 0xcf, 3, 6, ABS),
        OpCode::unofficial(DCP, 0xdf, 3, 7, ABX),
        OpCode::unofficial(DCP, 0xdb, 3, 7, ABY),
        OpCode::unofficial(DCP, 0xc3, 2, 8, IDX),
        OpCode::unofficial(DCP, 0xd3, 2, 8, IDY),
        OpCode::unofficial(DCP, 0xc7, 2, 5, ZPG),
        OpCode::unofficial(DCP, 0xd7, 2, 6, ZPX),

        OpCode::new(DEC, 0xce, 3, 6, ABS),
        OpCode::new(DEC, 0xde, 3, 7, ABX),
        OpCode::new(DEC, 0xc6, 2, 5, ZPG),
        OpCode::new(DEC, 0xd6, 2, 6, ZPX),

        OpCode::new(DEX, 0xca, 1, 2, IMP),

        OpCode::new(DEY, 0x88, 1, 2, IMP),

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

        OpCode::new(INY, 0xc8, 1, 2, IMP),

        OpCode::unofficial(ISB, 0xef, 3, 6, ABS),
        OpCode::unofficial(ISB, 0xff, 3, 7, ABX),
        OpCode::unofficial(ISB, 0xfb, 3, 7, ABY),
        OpCode::unofficial(ISB, 0xe3, 2, 8, IDX),
        OpCode::unofficial(ISB, 0xf3, 2, 8, IDY),
        OpCode::unofficial(ISB, 0xe7, 2, 5, ZPG),
        OpCode::unofficial(ISB, 0xf7, 2, 6, ZPX),

        OpCode::new(JMP, 0x4c, 1, 3, ABS), // setting 1 byte to evade JMP to advance the pc
        OpCode::new(JMP, 0x6c, 1, 5, IND),

        OpCode::new(JSR, 0x20, 1, 6, ABS), // setting 1 byte to evade JMP to advance the pc

        OpCode::unofficial(LAX, 0xaf, 3, 4, ABS),
        OpCode::unofficial(LAX, 0xbf, 3, 4, ABY), // +1 cycle if page crossed
        OpCode::unofficial(LAX, 0xa3, 2, 6, IDX),
        OpCode::unofficial(LAX, 0xb3, 2, 5, IDY), // +1 cycle if page crossed
        OpCode::unofficial(LAX, 0xa7, 2, 3, ZPG),
        OpCode::unofficial(LAX, 0xb7, 2, 4, ZPY),

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

        OpCode::new(LSR, 0x4a, 1, 2, ACC),
        OpCode::new(LSR, 0x4e, 3, 6, ABS),
        OpCode::new(LSR, 0x5e, 3, 7, ABX),
        OpCode::new(LSR, 0x46, 2, 5, ZPG),
        OpCode::new(LSR, 0x56, 2, 6, ZPX),

        OpCode::new(NOP, 0xea, 1, 2, IMP),
        OpCode::unofficial(NOP, 0x1a, 1, 2, IMP),
        OpCode::unofficial(NOP, 0x3a, 1, 2, IMP),
        OpCode::unofficial(NOP, 0x5a, 1, 2, IMP),
        OpCode::unofficial(NOP, 0x7a, 1, 2, IMP),
        OpCode::unofficial(NOP, 0xda, 1, 2, IMP),
        OpCode::unofficial(NOP, 0xfa, 1, 2, IMP),
        // DOP
        OpCode::unofficial(NOP, 0x80, 2, 2, IMM),
        OpCode::unofficial(NOP, 0x04, 2, 3, ZPG),
        OpCode::unofficial(NOP, 0x44, 2, 3, ZPG),
        OpCode::unofficial(NOP, 0x64, 2, 3, ZPG),
        OpCode::unofficial(NOP, 0x14, 2, 4, ZPX),
        OpCode::unofficial(NOP, 0x34, 2, 4, ZPX),
        OpCode::unofficial(NOP, 0x54, 2, 4, ZPX),
        OpCode::unofficial(NOP, 0x74, 2, 4, ZPX),
        OpCode::unofficial(NOP, 0xd4, 2, 4, ZPX),
        OpCode::unofficial(NOP, 0xf4, 2, 4, ZPX),
        // TOP
        OpCode::unofficial(NOP, 0x0c, 3, 4, ABS),
        OpCode::unofficial(NOP, 0x1c, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::unofficial(NOP, 0x3c, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::unofficial(NOP, 0x5c, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::unofficial(NOP, 0x7c, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::unofficial(NOP, 0xdc, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::unofficial(NOP, 0xfc, 3, 4, ABX), // +1 cycle if page crossed

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

        OpCode::new(RTI, 0x40, 1, 6, IMP),

        OpCode::new(RTS, 0x60, 1, 6, IMP),

        OpCode::unofficial(SAX, 0x8f, 3, 4, ABS),
        OpCode::unofficial(SAX, 0x83, 2, 6, IDX),
        OpCode::unofficial(SAX, 0x87, 2, 3, ZPG),
        OpCode::unofficial(SAX, 0x97, 2, 4, ZPY),

        OpCode::new(SBC, 0xed, 3, 4, ABS),
        OpCode::new(SBC, 0xfd, 3, 4, ABX), // +1 cycle if page crossed
        OpCode::new(SBC, 0xf9, 3, 4, ABY), // +1 cycle if page crossed
        OpCode::new(SBC, 0xe9, 2, 2, IMM),
        OpCode::new(SBC, 0xe1, 2, 6, IDX),
        OpCode::new(SBC, 0xf1, 2, 5, IDY), // +1 cycle if page crossed
        OpCode::new(SBC, 0xe5, 2, 3, ZPG),
        OpCode::new(SBC, 0xf5, 2, 4, ZPX),
        OpCode::unofficial(SBC, 0xeb, 2, 2, IMM),

        OpCode::new(SEC, 0x38, 1, 2, IMP),

        OpCode::new(SED, 0xf8, 1, 2, IMP),

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

        OpCode::new(TSX, 0xba, 1, 2, IMP),

        OpCode::new(TXA, 0x8a, 1, 2, IMP),

        OpCode::new(TXS, 0x9a, 1, 2, IMP),

        OpCode::new(TYA, 0x98, 1, 2, IMP),
    ];


    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        CPU_OPCODES.iter().map(|c| (c.code, c)).collect()
    };
}
