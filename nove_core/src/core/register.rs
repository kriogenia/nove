use std::ops::{AddAssign, BitAndAssign, SubAssign};

#[derive(Debug, Default)]
pub(super) struct Register(u8);

impl Register {
    #[inline]
    pub fn assign(&mut self, val: u8) {
        self.0 = val
    }

    #[inline]
    pub fn get(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn transfer(&mut self, other: &Register) {
        self.0 = other.0
    }
}

impl AddAssign<u8> for Register {
    fn add_assign(&mut self, rhs: u8) {
        self.0 = self.0.wrapping_add(rhs)
    }
}

impl SubAssign<u8> for Register {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 = self.0.wrapping_sub(rhs)
    }
}

impl BitAndAssign<u8> for Register {
    fn bitand_assign(&mut self, rhs: u8) {
        self.0 &= rhs
    }
}

impl PartialEq<u8> for Register {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}