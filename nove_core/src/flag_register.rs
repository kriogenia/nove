use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::Not;

#[derive(Default)]
pub struct FlagRegister<F: Into<u8>>(pub u8, pub PhantomData<F>);

impl<F: Into<u8>> FlagRegister<F> {
    pub fn set(&mut self, val: u8) {
        self.0 = val
    }

    pub fn get_bit(&self, flag: F) -> u8 {
        if self.is_raised(flag) {
            1
        } else {
            0
        }
    }

    pub fn set_bit(&mut self, flag: F, value: bool) {
        if value {
            self.raise(flag)
        } else {
            self.low(flag)
        }
    }

    pub fn raise(&mut self, flag: F) {
        self.0 |= flag.into()
    }

    pub fn low(&mut self, flag: F) {
        self.0 &= flag.into().not()
    }

    pub fn is_raised(&self, flag: F) -> bool {
        (self.0 & flag.into()) != 0
    }

    pub fn is_lowered(&self, flag: F) -> bool {
        !self.is_raised(flag)
    }

    pub(crate) fn print(&self, f: &mut Formatter<'_>, key: &str) -> std::fmt::Result {
        write!(f, "{key}={:#010b}", self.0)
    }
}
