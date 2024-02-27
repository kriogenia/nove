use crate::Rom;

const MEMORY_SIZE: usize = 0xFFFF; // 64 KiB
pub(crate) const PRG_ROM_ADDR: usize = 0x8000;
pub(crate) const PC_START_ADDR: u16 = 0xFFFC;

#[derive(Debug)]
pub struct Memory(pub [u8; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        Self([0; MEMORY_SIZE])
    }
}

impl Memory {
    pub fn load_rom(&mut self, rom: Rom) {
        let end = PRG_ROM_ADDR + rom.len();
        self.0[PRG_ROM_ADDR..end].copy_from_slice(&rom[..]);
        self.write_u16(PC_START_ADDR, PRG_ROM_ADDR as u16);
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.0[addr as usize] = value
    }

    pub fn update(&mut self, addr: u16, update_fn: fn(u8) -> u8) {
        let val = self.read(addr);
        self.write(addr, update_fn(val));
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let lo = self.read(addr);
        let hi = self.read(addr.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.write(addr, lo);
        self.write(addr.wrapping_add(1), hi)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update() {
        let mut mem = Memory::default();
        mem.write(0, 1);
        mem.update(0, |prev| prev + 1);

        assert_eq!(mem.read(0), 2)
    }

    #[test]
    fn read_little_endian() {
        let mut mem = Memory::default();
        mem.write(0, 0x01);
        mem.write(1, 0x23);

        assert_eq!(mem.read_u16(0), 0x2301)
    }

    #[test]
    fn write_little_endian() {
        let mut mem = Memory::default();
        mem.write_u16(0, 0x0123);

        assert_eq!(mem.read_u16(0), 0x0123)
    }
}
