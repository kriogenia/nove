use std::ops::{AddAssign, BitAnd, BitAndAssign, BitOrAssign, BitXorAssign, SubAssign};

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

    #[inline]
    pub fn overflowing_sub(&self, rhs: u8) -> (u8, bool) {
        self.0.overflowing_sub(rhs)
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

impl BitOrAssign<u8> for Register {
    fn bitor_assign(&mut self, rhs: u8) {
        self.0 |= rhs
    }
}

impl BitXorAssign<u8> for Register {
    fn bitxor_assign(&mut self, rhs: u8) {
        self.0 ^= rhs
    }
}

impl PartialEq<u8> for Register {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl BitAnd<u8> for &Register {
    type Output = u8;

    fn bitand(self, rhs: u8) -> Self::Output {
        self.0 & rhs
    }
}