#[derive(Debug, Default)]
pub struct AddressRegister {
    hi: u8,
    lo: u8,
    ptr: BytePointer,
}

impl AddressRegister {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set(&mut self, val: u16) {
        let [hi, lo] = u16::to_be_bytes(val);
        self.hi = hi;
        self.lo = lo;
    }

    pub fn get(&self) -> u16 {
        u16::from_be_bytes([self.hi, self.lo])
    }

    pub fn update(&mut self, val: u8) {
        match self.ptr {
            BytePointer::Hi => self.hi = val,
            BytePointer::Lo => self.lo = val,
        }
        self.mirror_down();
        self.swap();
    }

    pub fn inc(&mut self, val: u8) {
        self.lo = match self.lo.overflowing_add(val) {
            (res, true) => {
                self.hi = self.hi.wrapping_add(1);
                res
            }
            (res, false) => res,
        };
        self.mirror_down();
    }

    pub fn reset(&mut self) {
        self.ptr = BytePointer::Hi;
    }

    fn swap(&mut self) {
        self.ptr = match self.ptr {
            BytePointer::Hi => BytePointer::Lo,
            BytePointer::Lo => BytePointer::Hi,
        }
    }

    fn mirror_down(&mut self) {
        if self.get() > crate::addresses::ppu::LIMIT {
            self.set(self.get() & crate::addresses::ppu::LIMIT);
        }
    }
}

#[derive(Debug, Default, PartialEq)]
enum BytePointer {
    #[default]
    Hi,
    Lo,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::addresses::ppu;

    #[test]
    fn set() {
        let mut reg = AddressRegister::new();
        reg.set(0x1234);
        assert_eq!(0x12, reg.hi);
        assert_eq!(0x34, reg.lo);
    }

    #[test]
    fn get() {
        let reg = AddressRegister {
            hi: 0x12,
            lo: 0x34,
            ptr: BytePointer::Hi,
        };
        assert_eq!(0x1234, reg.get());
    }

    #[test]
    fn update() {
        let mut reg = AddressRegister::new();
        reg.update(0x12);
        assert_eq!(0x1200, reg.get());
        reg.update(0x34);
        assert_eq!(0x1234, reg.get());
        reg.update(0x40);
        assert_eq!(0x0034, reg.get());
    }

    #[test]
    fn inc() {
        let mut reg = AddressRegister::new();
        reg.set(0x1234);
        reg.inc(0x0c);
        assert_eq!(0x1240, reg.get());
        reg.inc(0xc0);
        assert_eq!(0x1300, reg.get());
        reg.set(ppu::LIMIT);
        reg.inc(3);
        assert_eq!(0x0002, reg.get());
    }

    #[test]
    fn reset() {
        let mut reg = AddressRegister::new();
        reg.update(0x12);
        assert_eq!(BytePointer::Lo, reg.ptr);
        reg.reset();
        assert_eq!(BytePointer::Hi, reg.ptr);
    }
}
