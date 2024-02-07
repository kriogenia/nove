mod processor_status;
mod memory;

use std::fmt::{Debug, Formatter};
use crate::core::memory::Memory;
use crate::core::processor_status::{Flag, ProcessorStatus};
use crate::instruction::{mnemonic::Mnemonic, OPCODES_MAP};
use crate::Rom;
use crate::exception::Exception;
use crate::instruction::addressing_mode::AddressingMode;


#[derive(Default)]
pub struct NoveCore {
    /// Program Counter
    pc: u16,
    /// Accumulator
    a: u8,
    /// Index Register X
    x: u8,
    /// Index Register Y
    y: u8,
    /// Processor Status
    ps: ProcessorStatus,
    /// Memory Map
    memory: Memory,
}

macro_rules! ld {
    ($self:ident, $reg:ident, $code:tt) => {
        {
            let addr = $self.get_addr(&$code.addressing_mode);
            $self.$reg = $self.memory.read(addr);
            $self.update_z_and_n($self.$reg);
            $self.pc += $code.bytes as u16 - 1;
        }
    };
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
            let opcode = OPCODES_MAP.get(&byte).ok_or(Exception::WrongOpCode(byte))?;
            match opcode.mnemonic {
                BRK => break 'game_loop,
                INX => {
                    self.x = self.x.wrapping_add(1);
                    self.update_z_and_n(self.x);
                },
                LDA => ld!(self, a, opcode),
                LDX => ld!(self, x, opcode),
                LDY => ld!(self, y, opcode),
                STA => {
                    let addr = self.get_addr(&opcode.addressing_mode);
                    self.memory.write(addr, self.a);
                    self.pc += opcode.bytes as u16 - 1;
                },
                TAX => {
                    self.x = self.a;
                    self.update_z_and_n(self.x);
                }
            }
        }

        Ok(())
    }

    fn get_addr(&self, mode: &AddressingMode) -> u16 {
        use AddressingMode::*;
        match mode {
            IMM => self.pc,
            ZPG => self.next_byte() as u16,
            ZPX => self.next_byte().wrapping_add(self.x) as u16,
            ZPY => self.next_byte().wrapping_add(self.y) as u16,
            ABS => self.next_word(),
            ABX => self.next_word().wrapping_add(self.x as u16),
            ABY => self.next_word().wrapping_add(self.y as u16),
            IDX => self.memory.read_u16(self.next_byte().wrapping_add(self.x) as u16),
            IDY => self.memory.read_u16(self.next_byte().wrapping_add(self.y) as u16),
            IMP => unreachable!("addressing mode {mode:?} should not access address"),
        }
    }

    fn next_byte(&self) -> u8 {
        self.memory.read(self.pc)
    }

    fn next_word(&self) -> u16 {
        self.memory.read_u16(self.pc)
    }

    #[cfg(test)]
    fn load_and_run(&mut self, rom: Rom) {
        self.load(rom);
        self.reset();
        self.run().expect("error while running the program")
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

    macro_rules! rom {
        (a:$val:literal, run:$($opcode:tt),+) => {
            vec![0xA9, $val, $($opcode),+, 0x00]
        };
        (x:$val:literal, run:$($opcode:tt),+) => {
            vec![0xA2, $val, $($opcode),+, 0x00]
        };
        (a:$acc:literal, x:$x:literal, run:$($opcode:tt),+) => {
            vec![0xA9, $acc, 0xA2, $x, $($opcode),+, 0x00]
        };
    }

    #[test]
    fn inx() {
        let opcode = 0xE8;

        let mut cpu = NoveCore::default();
        cpu.load_and_run(rom!(x:0x05, run:opcode));
        assert_eq!(cpu.x, 0x06);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));

        cpu.load_and_run(rom!(x:0xFF, run:opcode));
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(rom!(x:0xFE, run:opcode));
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

    #[test]
    fn lda_abs() {
        let mut cpu = NoveCore::new();

        cpu.memory.write_u16(0x3412, 0x10);
        cpu.load_and_run(vec![0xAD, 0x12, 0x34, BREAK]);
        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.pc, PC_START + 4);

        cpu.memory.write(0x1234, 0x12);
        cpu.load_and_run(rom!(x:0x02, run:0xBD, 0x32, 0x12));
        assert_eq!(cpu.a, 0x12);
        assert_eq!(cpu.pc, PC_START + 2 + 4);

        // todo aby
    }

    #[test]
    fn lda_id() {
        let mut cpu = NoveCore::new();

        cpu.memory.write_u16(0x05, 0x1234);
        cpu.memory.write(0x1234, 0x12);
        cpu.load_and_run(rom!(x:0x02, run:0xA1, 0x03));
        assert_eq!(cpu.a, 0x12);
        assert_eq!(cpu.pc, PC_START + 2 + 3);

        // todo idy
    }

    #[test]
    fn lda_imm() {
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
    fn lda_zp() {
        let mut cpu = NoveCore::new();

        cpu.memory.write(0x05, 0x10);
        cpu.load_and_run(vec![0xA5, 0x05, BREAK]);
        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.memory.write(0x07, 0x12);
        cpu.load_and_run(rom!(x:0x02, run:0xB5, 0x05));
        assert_eq!(cpu.a, 0x12);
        assert_eq!(cpu.pc, PC_START + 2 + 3);
    }


    #[test]
    fn ldx_abs() {
        let mut cpu = NoveCore::new();

        cpu.memory.write_u16(0x3412, 0x10);
        cpu.load_and_run(vec![0xAE, 0x12, 0x34, BREAK]);
        assert_eq!(cpu.x, 0x10);
        assert_eq!(cpu.pc, PC_START + 4);

        // todo aby
    }

    #[test]
    fn ldx_imm() {
        let mut cpu = NoveCore::new();

        cpu.load_and_run(vec![0xA2, 0x05, BREAK]);
        assert_eq!(cpu.x, 0x05);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.load_and_run(vec![0xA2, 0x00, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(vec![0xA2, 0xFF, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

    #[test]
    fn ldx_zp() {
        let mut cpu = NoveCore::new();

        cpu.memory.write(0x05, 0x10);
        cpu.load_and_run(vec![0xA6, 0x05, BREAK]);
        assert_eq!(cpu.x, 0x10);
        assert_eq!(cpu.pc, PC_START + 3);

        // todo zpy
    }


    #[test]
    fn ldy_abs() {
        let mut cpu = NoveCore::new();

        cpu.memory.write_u16(0x3412, 0x10);
        cpu.load_and_run(vec![0xAC, 0x12, 0x34, BREAK]);
        assert_eq!(cpu.y, 0x10);
        assert_eq!(cpu.pc, PC_START + 4);

        cpu.memory.write_u16(0x3414, 0x10);
        cpu.load_and_run(rom![x:0x02, run:0xAC, 0x12, 0x34]);
        assert_eq!(cpu.y, 0x10);
        assert_eq!(cpu.pc, PC_START + 2 + 4);
    }

    #[test]
    fn ldy_imm() {
        let mut cpu = NoveCore::new();

        cpu.load_and_run(vec![0xA0, 0x05, BREAK]);
        assert_eq!(cpu.y, 0x05);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.load_and_run(vec![0xA0, 0x00, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(vec![0xA0, 0xFF, BREAK]);
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

    #[test]
    fn ldy_zp() {
        let mut cpu = NoveCore::new();

        cpu.memory.write(0x05, 0x10);
        cpu.load_and_run(vec![0xA4, 0x05, BREAK]);
        assert_eq!(cpu.y, 0x10);
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.memory.write(0x08, 0x10);
        cpu.load_and_run(rom![x:0x03, run:0xA4, 0x05]);
        assert_eq!(cpu.y, 0x10);
        assert_eq!(cpu.pc, PC_START + 2 + 3);

        // todo zpy
    }

    #[test]
    fn sta_zp() {
        let mut cpu = NoveCore::new();

        cpu.load_and_run(rom!(a:0x05, run:0x85, 0x15));
        assert_eq!(cpu.memory.read(0x15), 0x05);
        assert_eq!(cpu.pc, PC_START + 3 + 2);

        cpu.load_and_run(rom!(a:0x05, x:0x10, run:0x95, 0x15));
        assert_eq!(cpu.memory.read(0x25), 0x05);
        assert_eq!(cpu.pc, PC_START + 5 + 2);
    }

    #[test]
    fn sta_ab() {
        let mut cpu = NoveCore::new();

        cpu.load_and_run(rom!(a:0x05, run:0x8D, 0x34, 0x12));
        assert_eq!(cpu.memory.read_u16(0x1234), 0x05);
        assert_eq!(cpu.pc, PC_START + 3 + 3);

        cpu.load_and_run(rom!(a:0x05, x:0x10, run:0x9D, 0x11, 0x11));
        assert_eq!(cpu.memory.read(0x1121), 0x05);
        assert_eq!(cpu.pc, PC_START + 5 + 3);
        // todo aby 99
    }

    #[test]
    fn sta_id() {
        let mut cpu = NoveCore::new();

        cpu.memory.write_u16(0x12, 0x1234);
        cpu.load_and_run(rom!(a:0x05, x:0x02, run:0x81, 0x10));
        assert_eq!(cpu.memory.read_u16(0x1234), 0x05);
        assert_eq!(cpu.pc, PC_START + 5 + 2);

        // todo idy
    }

    #[test]
    fn tax() {
        let mut cpu = NoveCore::new();
        let opcode = 0xAA;

        cpu.load_and_run(rom!(a:0x05, run:opcode));
        assert_eq!(cpu.x, 0x05);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));

        cpu.load_and_run(rom!(a:0x00, run:opcode));
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(rom!(a:0xFF, run:opcode));
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

}