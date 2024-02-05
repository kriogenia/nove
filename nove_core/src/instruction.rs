use crate::exception::Exception;
use crate::OpCodeSlice;

#[allow(clippy::upper_case_acronyms)]
pub enum Instruction {
    /// Force Interrupt
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BRK
    BRK,
    /// LoaD Accumulator
    /// Loads a byte of memory into the accumulator setting the Z and N flags as appropriate.
    LDA(u8),
}

impl TryFrom<OpCodeSlice<'_>> for Instruction {
    type Error = Exception;

    fn try_from(value: OpCodeSlice) -> Result<Self, Self::Error> {
        use Instruction::*;

        match value {
            [0x00] => Ok(BRK),
            [0xA9, param] => Ok(LDA(*param)),
            _ => Err(Exception::WrongOpCode(value[0])),
        }

    }
}
