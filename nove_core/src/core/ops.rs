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
                unimplemented!()
            }
        }
    }
}