use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NoveError {
    #[error("iNES is the only supported format")]
    WrongRomFormat,
    #[error("wrong op_code: {0:02x}")]
    WrongOpCode(u8),
}
