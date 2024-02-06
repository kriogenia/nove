use crate::exception::Exception;
use crate::OpCodeSlice;

#[allow(clippy::upper_case_acronyms)]
pub enum Instruction {
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
    LDA(u8),
    /// Transfer Accumulator to X
    /// X,Z,N = A
    /// Copies the current contents of the accumulator into the X register.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
    TAX,
}

impl TryFrom<OpCodeSlice<'_>> for Instruction {
    type Error = Exception;

    fn try_from(value: OpCodeSlice) -> Result<Self, Self::Error> {
        use Instruction::*;

        match value {
            [0x00, _] => Ok(BRK),
            [0xAA, _] => Ok(TAX),
            [0xA9, param] => Ok(LDA(*param)),
            [0xE8, _] => Ok(INX),
            _ => Err(Exception::WrongOpCode(value[0])),
        }

    }
}
