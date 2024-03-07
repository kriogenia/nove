use crate::addresses::rom;
use crate::memory::{Memory, MEMORY_SIZE};
use crate::Program;

#[derive(Debug)]
pub struct CpuMem(pub [u8; MEMORY_SIZE]);

impl Default for CpuMem {
    fn default() -> Self {
        Self([0; MEMORY_SIZE])
    }
}

impl CpuMem {
    pub fn load_rom(&mut self, rom: Program) {
        use crate::addresses::PC_START;
        let end = rom::PRG_ROM_START as usize + rom.len();
        self.0[rom::PRG_ROM_START as usize..end].copy_from_slice(&rom[..]);
        self.write_u16(PC_START, rom::PRG_ROM_START);
    }
}

impl Memory for CpuMem {
    fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.0[addr as usize] = value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update() {
        let mut mem = CpuMem::default();
        mem.write(0, 1);
        mem.update(0, |prev| prev + 1);

        assert_eq!(mem.read(0), 2)
    }

    #[test]
    fn read_little_endian() {
        let mut mem = CpuMem::default();
        mem.write(0, 0x01);
        mem.write(1, 0x23);

        assert_eq!(mem.read_u16(0), 0x2301)
    }

    #[test]
    fn write_little_endian() {
        let mut mem = CpuMem::default();
        mem.write_u16(0, 0x0123);

        assert_eq!(mem.read_u16(0), 0x0123)
    }
}
