mod processor_status;
mod memory;

use std::fmt::{Debug, Formatter};
use crate::core::memory::Memory;
use crate::core::processor_status::{Flag, ProcessorStatus};
use crate::instruction::{mnemonic::Mnemonic, OpCode, OPCODES_MAP};
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
    ($self:ident, $reg:ident, $code:expr) => {
        {
            let addr = $self.get_addr(&$code.addressing_mode);
            $self.$reg = $self.memory.read(addr);
            $self.update_z_and_n($self.$reg);
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
                AND => {
                    let addr = self.get_addr(&opcode.addressing_mode);
                    self.a &= self.memory.read(addr);
                    self.update_z_and_n(self.a);
                }
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
                },
                TAX => {
                    self.x = self.a;
                    self.update_z_and_n(self.x);
                }
            }

            self.update_pc(opcode);
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

    fn update_pc(&mut self, opcode: &OpCode) {
        self.pc += opcode.bytes as u16 - 1;
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

    const PC_START: u16 = memory::PRG_ROM_ADDR as u16;

    const A: u8 = 0xA9;
    const X: u8 = 0xA2;
    const Y: u8 = 0xA0;

    macro_rules! rom {
        ($($opcode:expr),+) => {
            vec![$($opcode),+, 0x00]
        };
        ($($ld:expr),+; $($opcode:expr),+) => {
            vec![$($ld),+, $($opcode),+, 0x00]
        };
    }

    #[test]
    fn and_abs() {
        let mut cpu = NoveCore::default();

        cpu.memory.write(0x0011, 0b110);
        cpu.load_and_run(rom!(A, 0b011; 0x2D, 0x11, 0x00));
        assert_eq!(cpu.a, 0b010);

        cpu.memory.write(0x0015, 0b111);
        cpu.load_and_run(rom!(A, 0b011, X, 0x04; 0x3D, 0x11, 0x00));
        assert_eq!(cpu.a, 0b011);

        cpu.memory.write(0x0019, 0b010);
        cpu.load_and_run(rom!(A, 0b011, Y, 0x08; 0x39, 0x11, 0x00));
        assert_eq!(cpu.a, 0b010);
    }

    #[test]
    fn and_imm() {
        let mut cpu = NoveCore::default();

        cpu.memory.write(0x0011, 0b110);
        cpu.load_and_run(rom!(A, 0b011; 0x2D, 0x11, 0x00));
        assert_eq!(cpu.a, 0b010);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));

        // todo test Z and N

        // test!(&mut cpu, rom, a:0b010, x:0, y:0, pc: +2)
    }

    #[test]
    fn inx() {
        let mut cpu = NoveCore::default();

        cpu.load_and_run(rom!(X, 0x05; 0xE8));
        assert_eq!(cpu.x, 0x06);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));

        cpu.load_and_run(rom!(X, 0xFF; 0xE8));
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(rom!(X, 0xFE; 0xE8));
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

    #[test]
    fn lda_abs() {
        let mut cpu = NoveCore::new();

        cpu.memory.write_u16(0x3412, 0x10);
        cpu.load_and_run(rom![0xAD, 0x12, 0x34]);
        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.pc, PC_START + 4);

        cpu.memory.write(0x1234, 0x12);
        cpu.load_and_run(rom!(X, 0x02; 0xBD, 0x32, 0x12));
        assert_eq!(cpu.a, 0x12);
        assert_eq!(cpu.pc, PC_START + 2 + 4);

        cpu.memory.write(0x1242, 0x13);
        cpu.load_and_run(rom!(Y, 0x10; 0xB9, 0x32, 0x12));
        assert_eq!(cpu.a, 0x13);
        assert_eq!(cpu.pc, PC_START + 2 + 4);
    }

    #[test]
    fn lda_id() {
        let mut cpu = NoveCore::new();

        cpu.memory.write_u16(0x05, 0x1234);
        cpu.memory.write(0x1234, 0x12);
        cpu.load_and_run(rom!(X, 0x02; 0xA1, 0x03));
        assert_eq!(cpu.a, 0x12);
        assert_eq!(cpu.pc, PC_START + 2 + 3);

        cpu.memory.write_u16(0x07, 0x1111);
        cpu.memory.write(0x1111, 0x13);
        cpu.load_and_run(rom!(Y, 0x04; 0xB1, 0x03));
        assert_eq!(cpu.a, 0x13);
        assert_eq!(cpu.pc, PC_START + 2 + 3);
    }

    #[test]
    fn lda_imm() {
        let mut cpu = NoveCore::new();

        cpu.load_and_run(rom![0xA9, 0x05]);
        assert_eq!(cpu.a, 0x05);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.load_and_run(rom![0xA9, 0x00]);
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(rom![0xA9, 0xFF]);
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

    #[test]
    fn lda_zp() {
        let mut cpu = NoveCore::new();

        cpu.memory.write(0x05, 0x10);
        cpu.load_and_run(rom![0xA5, 0x05]);
        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.memory.write(0x07, 0x12);
        cpu.load_and_run(rom!(X, 0x02; 0xB5, 0x05));
        assert_eq!(cpu.a, 0x12);
        assert_eq!(cpu.pc, PC_START + 2 + 3);
    }


    #[test]
    fn ldx_abs() {
        let mut cpu = NoveCore::new();

        cpu.memory.write_u16(0x3412, 0x10);
        cpu.load_and_run(rom![0xAE, 0x12, 0x34]);
        assert_eq!(cpu.x, 0x10);
        assert_eq!(cpu.pc, PC_START + 4);

        cpu.memory.write_u16(0x341a, 0x20);
        cpu.load_and_run(rom![Y, 0x08; 0xBE, 0x12, 0x34]);
        assert_eq!(cpu.x, 0x20);
        assert_eq!(cpu.pc, PC_START + 2 + 4);
    }

    #[test]
    fn ldx_imm() {
        let mut cpu = NoveCore::new();

        cpu.load_and_run(rom![0xA2, 0x05]);
        assert_eq!(cpu.x, 0x05);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.load_and_run(rom![0xA2, 0x00]);
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(rom![0xA2, 0xFF]);
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

    #[test]
    fn ldx_zp() {
        let mut cpu = NoveCore::new();

        cpu.memory.write(0x05, 0x10);
        cpu.load_and_run(rom![0xA6, 0x05]);
        assert_eq!(cpu.x, 0x10);
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.memory.write(0x07, 0x11);
        cpu.load_and_run(rom![Y, 0x02, 0xB6, 0x05]);
        assert_eq!(cpu.x, 0x11);
        assert_eq!(cpu.pc, PC_START + 2 + 3);
    }


    #[test]
    fn ldy_abs() {
        let mut cpu = NoveCore::new();

        cpu.memory.write_u16(0x3412, 0x10);
        cpu.load_and_run(rom![0xAC, 0x12, 0x34]);
        assert_eq!(cpu.y, 0x10);
        assert_eq!(cpu.pc, PC_START + 4);

        cpu.memory.write_u16(0x3414, 0x10);
        cpu.load_and_run(rom![X, 0x02; 0xAC, 0x12, 0x34]);
        assert_eq!(cpu.y, 0x10);
        assert_eq!(cpu.pc, PC_START + 2 + 4);
    }

    #[test]
    fn ldy_imm() {
        let mut cpu = NoveCore::new();

        cpu.load_and_run(rom![0xA0, 0x05]);
        assert_eq!(cpu.y, 0x05);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.load_and_run(rom![0xA0, 0x00]);
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(rom![0xA0, 0xFF]);
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

    #[test]
    fn ldy_zp() {
        let mut cpu = NoveCore::new();

        cpu.memory.write(0x05, 0x10);
        cpu.load_and_run(rom![0xA4, 0x05]);
        assert_eq!(cpu.y, 0x10);
        assert_eq!(cpu.pc, PC_START + 3);

        cpu.memory.write(0x08, 0x10);
        cpu.load_and_run(rom![X, 0x03; 0xA4, 0x05]);
        assert_eq!(cpu.y, 0x10);
        assert_eq!(cpu.pc, PC_START + 2 + 3);
    }

    #[test]
    fn sta_zp() {
        let mut cpu = NoveCore::new();

        cpu.load_and_run(rom!(A, 0x05; 0x85, 0x15));
        assert_eq!(cpu.memory.read(0x15), 0x05);
        assert_eq!(cpu.pc, PC_START + 3 + 2);

        cpu.load_and_run(rom!(A, 0x05, X, 0x10; 0x95, 0x15));
        assert_eq!(cpu.memory.read(0x25), 0x05);
        assert_eq!(cpu.pc, PC_START + 5 + 2);
    }

    #[test]
    fn sta_abs() {
        let mut cpu = NoveCore::new();

        cpu.load_and_run(rom!(A, 0x05; 0x8D, 0x34, 0x12));
        assert_eq!(cpu.memory.read_u16(0x1234), 0x05);
        assert_eq!(cpu.pc, PC_START + 3 + 3);

        cpu.load_and_run(rom!(A, 0x05, X, 0x10; 0x9D, 0x11, 0x11));
        assert_eq!(cpu.memory.read(0x1121), 0x05);
        assert_eq!(cpu.pc, PC_START + 5 + 3);

        cpu.load_and_run(rom!(A, 0x05, Y, 0x20; 0x99, 0x11, 0x11));
        assert_eq!(cpu.memory.read(0x1131), 0x05);
        assert_eq!(cpu.pc, PC_START + 5 + 3);
    }

    #[test]
    fn sta_id() {
        let mut cpu = NoveCore::new();

        cpu.memory.write_u16(0x12, 0x1234);
        cpu.load_and_run(rom!(A, 0x05, X, 0x02; 0x81, 0x10));
        assert_eq!(cpu.memory.read_u16(0x1234), 0x05);
        assert_eq!(cpu.pc, PC_START + 5 + 2);

        cpu.memory.write_u16(0x16, 0x1234);
        cpu.load_and_run(rom!(A, 0x05, Y, 0x06; 0x81, 0x10));
        assert_eq!(cpu.memory.read_u16(0x1234), 0x05);
        assert_eq!(cpu.pc, PC_START + 5 + 2);
    }

    #[test]
    fn tax() {
        let mut cpu = NoveCore::new();

        cpu.load_and_run(rom!(A, 0x05; 0xAA));
        assert_eq!(cpu.x, 0x05);
        assert!(cpu.ps.is_lowered(Flag::Zero));
        assert!(cpu.ps.is_lowered(Flag::Negative));

        cpu.load_and_run(rom!(A, 0x00; 0xAA));
        assert!(cpu.ps.is_raised(Flag::Zero));

        cpu.load_and_run(rom!(A, 0xFF; 0xAA));
        assert!(cpu.ps.is_raised(Flag::Negative));
    }

}