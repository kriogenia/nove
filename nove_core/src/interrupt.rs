use crate::core::processor_status::StatusFlag;

#[derive(Debug, Default, PartialEq)]
pub enum InterruptFlag {
    #[default]
    None,
    NMI,
    BRK, // todo handle
}

impl InterruptFlag {
    pub fn cycles(&self) -> u8 {
        match self {
            InterruptFlag::NMI => 2,
            _ => panic!("requesting cycles of no flag"),
        }
    }

    pub fn addr(&self) -> u16 {
        match self {
            InterruptFlag::NMI => 0xfffa,
            _ => panic!("requesting address of no flag"),
        }
    }

    pub fn mask(&self, val: u8) -> u8 {
        let brk: u8 = StatusFlag::Break.into();
        let one: u8 = StatusFlag::One.into();
        match self {
            InterruptFlag::NMI => val & !brk | one,
            _ => panic!("requesting mask of no flag"),
        }
    }
}
