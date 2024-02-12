use std::fmt::{Debug, Formatter};
use std::ops::Not;

pub(super) const OVERFLOW_MASK: u8 = 0b1000_0000;

/// N V _ B D I Z C
#[derive(Default)]
pub(super) struct ProcessorStatus(pub u8);

pub(super) enum Flag {
    Carry = 0b0000_0001,
    Zero = 0b0000_0010,
    Break = 0b0001_0000,
    One = 0b0010_0000,
    Overflow = 0b0100_0000,
    Negative = 0b1000_0000,
}

impl ProcessorStatus {
    pub fn get_bit(&self, flag: Flag) -> u8 {
        if self.is_raised(flag) { 1 } else { 0 }
    }

    /// Returns the status of the CPU as a byte with the B flag up.
    /// https://www.nesdev.org/wiki/Status_flags#The_B_flag
    pub fn get_for_push(&self) -> u8 {
        self.0 | Flag::One as u8 | Flag::Break as u8
    }

    pub fn set_bit(&mut self, flag: Flag, value: bool) {
        if value { self.raise(flag) } else { self.low(flag) }
    }

    pub fn raise(&mut self, flag: Flag) {
        self.0 |= flag as u8
    }

    pub fn low(&mut self, flag: Flag) {
        self.0 &= (flag as u8).not()
    }

    pub fn is_raised(&self, flag: Flag) -> bool {
        (self.0 & flag as u8) != 0
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
    use crate::core::processor_status::{Flag, ProcessorStatus};

    #[test]
    fn processor_status() {
        let mut ps = ProcessorStatus::default();
        ps.set_bit(Flag::Carry, true);
        ps.set_bit(Flag::Overflow, true);
        assert_eq!(ps.0, 0b0100_0001);
        assert_eq!(ps.get_for_push(), 0b0111_0001);
    }

}