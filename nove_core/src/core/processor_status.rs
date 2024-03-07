use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use crate::flag_register::FlagRegister;

const INIT: u8 = 0b100100;
pub(super) const OVERFLOW_MASK: u8 = 0b1000_0000;

/// N V _ B D I Z C
pub type ProcessorStatus = FlagRegister<StatusFlag>;

pub enum StatusFlag {
    Carry = 0b0000_0001,
    Zero = 0b0000_0010,
    Interrupt = 0b0000_0100,
    Decimal = 0b0000_1000,
    Break = 0b0001_0000,
    One = 0b0010_0000,
    Overflow = 0b0100_0000,
    Negative = 0b1000_0000,
}

impl From<StatusFlag> for u8 {
    fn from(value: StatusFlag) -> Self {
        value as u8
    }
}

impl ProcessorStatus {
    pub fn new() -> Self {
        Self(INIT, PhantomData)
    }

    /// Returns the status of the CPU as a byte with the B flag up.
    /// https://www.nesdev.org/wiki/Status_flags#The_B_flag
    pub fn get_for_push(&self) -> u8 {
        self.0 | StatusFlag::One as u8 | StatusFlag::Break as u8
    }

    pub fn set_from_pull(&mut self, val: u8) {
        self.0 = val;
        self.raise(StatusFlag::One);
        self.low(StatusFlag::Break);
    }
}

impl Default for ProcessorStatus {
    fn default() -> Self {
        Self(0, PhantomData)
    }
}

impl Debug for ProcessorStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n\t\t  NV BDIZC")?;
        writeln!(f, "\t\t{:#010b}", self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::core::processor_status::{ProcessorStatus, StatusFlag};

    #[test]
    fn processor_status() {
        let mut ps = ProcessorStatus::default();
        ps.set_bit(StatusFlag::Carry, true);
        ps.set_bit(StatusFlag::Overflow, true);
        assert_eq!(ps.0, 0b0100_0001);
        assert_eq!(ps.get_for_push(), 0b0111_0001);
        ps.set_from_pull(0b0011_0001);
        assert_eq!(ps.0, 0b0010_0001);
    }
}
