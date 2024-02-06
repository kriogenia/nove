use std::ops::Not;

/// N V _ B D I Z C
#[derive(Default, Debug)]
pub(super) struct ProcessorStatus(u8);

pub(super) enum Flag {
    Zero = 0b0000_0010,
    Negative = 0b1000_0000,
}

impl ProcessorStatus {
    pub fn raise(&mut self, flag: Flag) {
        self.0 |= flag as u8
    }

    pub fn low(&mut self, flag: Flag) {
        self.0 &= (flag as u8).not()
    }

    pub fn is_raised(&self, flag: Flag) -> bool {
        (self.0 & flag as u8) != 0
    }

    pub fn is_lowered(&self, flag: Flag) -> bool {
        (self.0 & flag as u8) == 0
    }
}