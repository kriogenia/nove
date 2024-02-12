const STACK_PAGE: u16 = 0x0100;
const STACK_START: u8 = 0xff;

pub(crate) struct StackPointer(u8);

impl StackPointer {
    pub fn get(&self) -> u16 {
        STACK_PAGE | (self.0 as u16)
    }

    pub fn next(&mut self) {
        self.0 = self.0.wrapping_sub(1)
    }

    pub fn prev(&mut self) {
        self.0 = self.0.wrapping_add(1)
    }
}

impl Default for StackPointer {
    fn default() -> Self {
        Self(STACK_START)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stack_pointer() {
        let mut pointer = StackPointer::default();
        assert_eq!(pointer.get(), 0x01ff);
        pointer.next();
        assert_eq!(pointer.get(), 0x01fe);
        pointer.prev();
        assert_eq!(pointer.get(), 0x01ff);
    }
}
