pub(crate) enum Direction {
    Left,
    Right,
}

pub(crate) enum Displacement {
    Rotation(Direction, bool),
    Shift(Direction),
}

impl Displacement {
    pub fn displace(&self, val: u8) -> (u8, bool) {
        use Displacement::*;
        match self {
            Rotation(Direction::Left, carry) => {
                let c = val >> 7 == 1;
                let v = (val << 1) | if *carry { 0b0000_0001 } else { 0 };
                (v, c)
            }
            Rotation(Direction::Right, carry) => {
                let c = val & 0b0000_0001 == 1;
                let v = (val >> 1) | if *carry { 0b1000_0000 } else { 0 };
                (v, c)
            }
            Shift(Direction::Left) => (val << 1, val >> 7 == 1),
            Shift(Direction::Right) => (val >> 1, val & 0b0000_0001 == 1),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Direction::*;
    use Displacement::*;

    #[test]
    fn rotate() {
        assert_eq!(
            Rotation(Left, false).displace(0b0101_0101),
            (0b1010_1010, false)
        );
        assert_eq!(
            Rotation(Left, true).displace(0b0101_0101),
            (0b1010_1011, false)
        );
        assert_eq!(
            Rotation(Left, true).displace(0b1010_1010),
            (0b0101_0101, true)
        );
        assert_eq!(
            Rotation(Right, false).displace(0b0101_0101),
            (0b0010_1010, true)
        );
        assert_eq!(
            Rotation(Right, true).displace(0b0101_0100),
            (0b1010_1010, false)
        );
        assert_eq!(
            Rotation(Right, false).displace(0b1010_1010),
            (0b0101_0101, false)
        );
    }

    #[test]
    fn shift() {
        assert_eq!(Shift(Left).displace(0b0101_0101), (0b1010_1010, false));
        assert_eq!(Shift(Left).displace(0b1010_1010), (0b010_10100, true));
    }
}
