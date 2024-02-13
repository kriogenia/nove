pub(crate) enum Rotation {
    Left,
    Right
}

impl Rotation {
    pub fn rotate(&self, val: u8, carry: bool) -> (u8, bool) {
        match self {
            Rotation::Left => {
                let c = val >> 7 == 1;
                let v = (val << 1) | if carry { 0b0000_0001 } else { 0 };
                (v, c)
            },
            Rotation::Right => {
                let c = val & 0b0000_0001 == 1;
                let v = (val >> 1) | if carry { 0b1000_0000 } else { 0 };
                (v, c)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rotate() {
        assert_eq!(Rotation::Left.rotate(0b0101_0101, false), (0b1010_1010, false));
        assert_eq!(Rotation::Left.rotate(0b0101_0101, true), (0b1010_1011, false));
        assert_eq!(Rotation::Left.rotate(0b1010_1010, true), (0b0101_0101, true));
        assert_eq!(Rotation::Right.rotate(0b0101_0101, false), (0b0010_1010, true));
        assert_eq!(Rotation::Right.rotate(0b0101_0100, true), (0b1010_1010, false));
        assert_eq!(Rotation::Right.rotate(0b1010_1010, false), (0b0101_0101, false));
    }

}