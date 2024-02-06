use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::instruction::Mnemonic::{BRK, INX, LDA, TAX};

pub struct OpCode {
    pub mnemonic: Mnemonic,
    pub code: u8,
}

impl OpCode {
    fn new(mnemonic: Mnemonic, code: u8) -> Self {
        Self { mnemonic, code }
    }
}

lazy_static! {
    pub static ref CPU_OPCODES: Vec<OpCode> = vec![
        OpCode::new(BRK, 0x00),
        OpCode::new(TAX, 0xAA),
        OpCode::new(LDA, 0xA9),
        OpCode::new(INX, 0xE8),
    ];


    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        CPU_OPCODES.iter().map(|c| (c.code, c)).collect()
    };
}


#[allow(clippy::upper_case_acronyms)]
pub enum Mnemonic {
    /// Force Interrupt
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BRK
    BRK,
    /// Increment X Register
    /// X,Z,N = X+1
    /// Adds one to the X register.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#INX
    INX,
    /// LoaD Accumulator
    /// A,Z,N = M
    /// Loads a byte of memory into the accumulator.
    /// Flags: N Z
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
    LDA,
    /// Transfer Accumulator to X
    /// X,Z,N = A
    /// Copies the current contents of the accumulator into the X register.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
    TAX,
}

