pub mod cpu;
mod instruction;
mod exception;

pub(crate) const OP_CODE_SLICE_SIZE: usize = 2;
pub(crate) type OpCodeSlice<'a> = &'a [u8];