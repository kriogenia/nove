use crate::flag_register::FlagRegister;

#[derive(Debug, Default, PartialEq)]
pub enum ControlFlags {
    #[default]
    Increment = 0b0000_0010,
}

impl From<ControlFlags> for u8 {
    fn from(value: ControlFlags) -> Self {
        value as u8
    }
}

pub type ControlRegister = FlagRegister<ControlFlags>;

impl ControlRegister {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn vram_add_inc(&mut self) -> u8 {
        if self.is_lowered(ControlFlags::Increment) {
            0x01
        } else {
            0x20
        }
    }
}
