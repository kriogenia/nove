#[derive(Debug, Default, PartialEq)]
pub enum InterruptFlag {
    #[default]
    None,
    NMI,
}
