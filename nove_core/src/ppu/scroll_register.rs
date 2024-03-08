use crate::register::RegWrite;

#[derive(Debug, Default)]
pub struct ScrollRegister {
    x: u8,
    y: u8,
    ptr: Axis,
}

#[derive(Debug, Default)]
enum Axis {
    #[default]
    X,
    Y,
}

impl RegWrite for ScrollRegister {
    fn write(&mut self, val: u8) {
        self.ptr = match self.ptr {
            Axis::X => {
                self.x = val;
                Axis::Y
            }
            Axis::Y => {
                self.y = val;
                Axis::X
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ppu::scroll_register::ScrollRegister;
    use crate::register::RegWrite;

    #[test]
    fn write() {
        let mut scroll = ScrollRegister::default();
        scroll.write(0x22);
        scroll.write(0x11);
        assert_eq!(scroll.x, 0x22);
        assert_eq!(scroll.y, 0x11);
    }
}
