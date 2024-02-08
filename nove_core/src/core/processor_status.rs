use std::ops::Not;

pub(super) const OVERFLOW_MASK: u8 = 0b1000_0000;

/// N V _ B D I Z C
#[derive(Default, Debug)]
pub(super) struct ProcessorStatus(pub u8);

pub(super) enum Flag {
    Carry = 0b0000_0001,
    Zero = 0b0000_0010,
    Overflow = 0b0100_0000,
    Negative = 0b1000_0000,
}

impl ProcessorStatus {
    pub fn get_bit(&self, flag: Flag) -> u8 {
        if self.is_raised(flag) { 1 } else { 0 }
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