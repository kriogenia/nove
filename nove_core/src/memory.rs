pub mod bus;
pub mod cpu_mem;

pub const PRG_ROM_START_ADDR: u16 = 0x8000;
pub const PRG_ROM_END_ADDR: u16 = 0xffff;
pub const MEMORY_SIZE: usize = PRG_ROM_END_ADDR as usize; // 64 KiB
pub const PC_START_ADDR: u16 = 0xfffc;

pub trait Memory {
    fn read(&self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, value: u8);

    fn update(&mut self, addr: u16, update_fn: fn(u8) -> u8) {
        let val = self.read(addr);
        self.write(addr, update_fn(val));
    }

    fn read_u16(&self, addr: u16) -> u16 {
        let lo = self.read(addr);
        let hi = self.read(addr.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    fn write_u16(&mut self, addr: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.write(addr, lo);
        self.write(addr.wrapping_add(1), hi)
    }
}
