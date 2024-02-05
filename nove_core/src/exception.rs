use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Exception {
    #[error("wrong op_code: {0:02x}")]
    WrongOpCode(u8),
}
