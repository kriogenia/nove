pub trait RegWrite {
    fn write(&mut self, val: u8);
}

pub trait RegRead {
    fn read(&self) -> u8;
}
