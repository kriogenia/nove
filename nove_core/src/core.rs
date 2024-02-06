mod processor_status;
mod memory;

use std::fmt::{Debug, Formatter};
use crate::core::memory::Memory;
use crate::core::processor_status::{Flag, ProcessorStatus};
use crate::instruction::{Mnemonic, OPCODES_MAP};
use crate::Rom;
use crate::exception::Exception;


#[derive(Default)]
pub struct NoveCore {
    /// Program Counter
    pc: u16,
    /// Accumulator
    a: u8,
    /// Index Register X
    x: u8,
    /// Processor Status
    ps: ProcessorStatus,
    /// Memory Map
    memory: Memory,
}

impl NoveCore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.pc = self.memory.read_u16(memory::PC_START_ADDR);
        self.a = 0;
        self.x = 0;
        self.ps = Default::default();
    }

    pub fn load(&mut self, rom: Rom) {
        self.memory.load_rom(rom);
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        'game_loop: loop {
            let byte = self.memory.read(self.pc);
            self.pc += 1;

            use Mnemonic::*;
            let opcode = OPCODES_MAP.get(&byte).ok_or_else(|| Exception::WrongOpCode(byte))?;
            match opcode.mnemonic {
                BRK => break 'game_loop,
                INX => {
                    self.x = self.x.wrapping_add(1);
                    self.update_z_and_n(self.x);
                },
                LDA => {
                    self.a = self.memory.read(self.pc);
                    self.pc += 1;
                    self.update_z_and_n(self.a);
                },
                TAX => {
                    self.x = self.a;
                    self.update_z_and_n(self.x);
                }
            }
        }

        Ok(())
    }

    #[cfg(test)]
    fn load_and_run(&mut self, rom: Rom) {
        self.load(rom);
        self.reset();
        self.run().expect("Error while running the program")
    }

    #[inline]
    fn update_z_and_n(&mut self, value: u8) {
        if value == 0 {
            self.ps.raise(Flag::Zero);
        } else {
            self.ps.low(Flag::Zero);
        }
        if value & 0b1000_0000 != 0 {
            self.ps.raise(Flag::Negative)
        } else {
            self.ps.low(Flag::Negative);
        }
    }

}

impl Debug for NoveCore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NovaCode {{ ")?;
        writeln!(f, "\tpc: {:?}", self.pc)?;
        writeln!(f, "\t a: {:?}", self.a)?;
        writeln!(f, "\t x: {:?}", self.x)?;
        writeln!(f, "\tps: {:?}", self.ps)?;
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const BREAK: u8 = 0x00;
    const PC_START: u16 = memory::PRG_ROM_ADDR as u16;

    macro_rules! loaded_acc {
        ($val:literal, $opcode:tt) => {
            vec![0xA9, $val, $opcode, 0x00]
        };
    }

    macro_rules! loaded_x {
        ($val:literal, $opcode:tt) => {
            vec![0xA9, $val, 0xAA, $opcode, 0x00]
        };
    }

    #[test]
    fn inx() {
        let opcode = 0xE8;

        let mut cpu = NoveCore::default();
        cpu.load_and_run(loaded_x!(0x05, opcode));
        assert_eq!(cpu.x, 0x06);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));

        cpu.load_and_run(loaded_x!(0xFF, opcode));
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(loaded_x!(0xFE, opcode));
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

    #[test]
    fn lda() {
        let mut cpu = NoveCore::new();
        let opcode = 0xA9;

        cpu.load_and_run(vec![opcode, 0x05, BREAK]);
        assert_eq!(cpu.a, 0x05);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.load_and_run(vec![opcode, 0x00, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(vec![opcode, 0xFF, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

    #[test]
    fn tax() {
        let mut cpu = NoveCore::new();
        let opcode = 0xAA;

        cpu.load_and_run(loaded_acc!(0x05, opcode));
        assert_eq!(cpu.x, 0x05);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));

        cpu.load_and_run(loaded_acc!(0x00, opcode));
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(loaded_acc!(0xFF, opcode));
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

}