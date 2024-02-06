mod processor_status;

use crate::core::processor_status::{Flag, ProcessorStatus};
use crate::instruction::{Mnemonic, OPCODES_MAP};
use crate::Program;
use crate::exception::Exception;

const MEMORY_SIZE: usize = 0xFFFF;  // 64 KiB
const PRG_ROM_ADDR: usize = 0x8000;

#[derive(Debug)]
pub struct NoveCore {
    /// Program Counter
    pc: u16,
    /// Accumulator
    a: u8,
    /// Index Register X
    x: u8,
    /// Processor Status
    ps: ProcessorStatus,
    // map to a new struct?
    /// Memory Map
    memory: [u8; MEMORY_SIZE],
}

impl NoveCore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self, program: Program) {
        self.memory[PRG_ROM_ADDR .. (PRG_ROM_ADDR + program.len())].copy_from_slice(&program[..]);
        self.pc = PRG_ROM_ADDR as u16;
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        'game_loop: loop {
            let byte = self.mem_read(self.pc);
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
                    self.a = self.mem_read(self.pc);
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
    fn load_and_run(&mut self, program: Program) {
        self.load(program);
        self.run().expect("Error while running the program")
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
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

impl Default for NoveCore {
    fn default() -> Self {
        Self {
            pc: Default::default(),
            a: Default::default(),
            x: Default::default(),
            ps: Default::default(),
            memory: [0; MEMORY_SIZE],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const BREAK: u8 = 0x00;

    const ZERO: u8 = 0;
    const NEG: u8 = 0xFF;

    const PC_START: u16 = PRG_ROM_ADDR as u16;

    #[test]
    fn inx() {
        let opcode = 0xe8;

        let mut cpu = NoveCore { x: 0x05, ..Default::default() };
        cpu.load_and_run(vec![opcode, BREAK]);
        assert_eq!(cpu.x, 0x06);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));
        assert_eq!(cpu.pc, PC_START+ 2);

        cpu = NoveCore { x: NEG, ..Default::default() };
        cpu.load_and_run(vec![opcode, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Zero));

        let mut cpu = NoveCore { x: NEG - 1, ..Default::default() };
        cpu.load_and_run(vec![opcode, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

    #[test]
    fn lda() {
        let opcode = 0xa9;
        let mut cpu = NoveCore::new();
        cpu.load_and_run(vec![opcode, 0x05, BREAK]);
        assert_eq!(cpu.a, 0x05);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.load_and_run(vec![opcode, ZERO, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(vec![opcode, NEG, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

    #[test]
    fn tax() {
        let opcode = 0xaa;
        let mut cpu = NoveCore { a: 0x05, ..Default::default() };
        cpu.load_and_run(vec![0xaa, BREAK]);
        assert_eq!(cpu.x, 0x05);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));
        assert_eq!(cpu.pc, PC_START + 2);

        cpu = NoveCore { a: ZERO, ..Default::default() };
        cpu.load_and_run(vec![opcode, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Zero));

        let mut cpu = NoveCore { a: NEG, ..Default::default() };
        cpu.load_and_run(vec![opcode, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

}