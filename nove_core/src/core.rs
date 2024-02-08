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
            //println!("{addr:x?}");
            $self.$reg = $self.memory.read(addr);
            $self.update_z_and_n($self.$reg);
        }
    };
}

/// Helper macro for debugging, easies the printing of hex values
#[allow(unused_macros)]
macro_rules! hexprint {
    ($val:expr) => {
        println!("{:x?}", $val);
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

    const A: u8 = 0xA9;
    const X: u8 = 0xA2;
    const Y: u8 = 0xA0;

    const Z: u8 = Flag::Zero as u8;
    const N: u8 = Flag::Negative as u8;

    macro_rules! rom {
        ($($opcode:expr),+) => {
            vec![$($opcode),+, 0x00]
        };
        ($($ld:expr),+; $($opcode:expr),+) => {
            vec![$($ld),+, $($opcode),+, 0x00]
        };
    }

    /// Runs a tests with the given core and rom checking the list of registers or addresses and the pc addition
    macro_rules! test {
        ($id:expr, $core:expr, $rom:expr, $($reg:ident: $val:literal),*; pc: +$pc:literal $(, ps: $ps:expr)*) => {
            println!($id);
            $core.load_and_run($rom);
            $(assert_eq!($core.$reg, $val);)+
            assert_eq!($core.pc, memory::PRG_ROM_ADDR as u16 + $pc + 7);
            $(assert_eq!($core.ps.0, $ps);)*
        };
        ($id:expr, $core:expr, $rom:expr, $($addr:literal: $val:literal),*; pc: +$pc:literal) => {
            println!($id);
            $core.load_and_run($rom);
            $({
                assert_eq!($core.memory.read_u16($addr), $val);
            })+
            assert_eq!($core.pc, memory::PRG_ROM_ADDR as u16 + $pc + 7);
        };
    }

    #[test]
    fn and() {
        let mut core = preloaded_core(); // 0x0005:0b1010

        test!("imm", &mut core, rom!(A, 0b1010, X, 0x00, Y, 0x00; 0x29, 0b1100), a:0b1000; pc: +2, ps: 0);
        test!("imm_z", &mut core, rom!(A, 0b1010, X, 0x00, Y, 0x00; 0x29, 0b0000), a:0; pc: +2, ps: Z);
        test!("imm_n", &mut core, rom!(A, 0b11111111, X, 0x00, Y, 0x00; 0x29, 0b11110000), a:0b11110000; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0b0110, X, 0x00, Y, 0x00; 0x2D, 0x05, 0x00), a:0b0010; pc: +3);
        test!("abx", &mut core, rom!(A, 0b0110, X, 0x02, Y, 0x00; 0x3D, 0x03, 0x00), a:0b0010; pc: +3);
        test!("aby", &mut core, rom!(A, 0b0110, X, 0x00, Y, 0x01; 0x39, 0x04, 0x00), a:0b0010; pc: +3);
        test!("idx", &mut core, rom!(A, 0b0110, X, 0x20, Y, 0x00; 0x21, 0x30), a:0b0010; pc: +2);
        test!("idy", &mut core, rom!(A, 0b0110, X, 0x00, Y, 0x10; 0x31, 0x40), a:0b0010; pc: +2);
        test!("zpg", &mut core, rom!(A, 0b0110, X, 0x00, Y, 0x00; 0x25, 0x05), a:0b0010; pc: +2);
        test!("zpx", &mut core, rom!(A, 0b0110, X, 0x02, Y, 0x00; 0x35, 0x03), a:0b0010; pc: +2);
    }

    #[test]
    fn inx() {
        let mut core = NoveCore::default();

        test!("inx", &mut core, rom!(A, 0, X, 5, Y, 0; 0xE8), x:6; pc: +1, ps: 0);
        test!("inx_z", &mut core, rom!(A, 0, X, 0xFF, Y, 0; 0xE8), x:0; pc: +1, ps: Z);
        test!("inx_z", &mut core, rom!(A, 0, X, 0xF0, Y, 0; 0xE8), x:0xF1; pc: +1, ps: N);
    }

    #[test]
    fn lda() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0, X, 0, Y, 0; 0xA9, 10), a:10; pc: +2, ps: 0);
        test!("imm_z", &mut core, rom!(A, 12, X, 0, Y, 0; 0xA9, 0b0000), a:0; pc: +2, ps: Z);
        test!("imm_n", &mut core, rom!(A, 1, X, 0, Y, 0; 0xA9, 0xFF), a:0xFF; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0, X, 0x00, Y, 0x00; 0xAD, 0x05, 0x00), a:10; pc: +3);
        test!("abx", &mut core, rom!(A, 0, X, 0x02, Y, 0x00; 0xBD, 0x03, 0x00), a:10; pc: +3);
        test!("aby", &mut core, rom!(A, 0, X, 0x00, Y, 0x01; 0xB9, 0x04, 0x00), a:10; pc: +3);
        test!("idx", &mut core, rom!(A, 0, X, 0x20, Y, 0x00; 0xA1, 0x30), a:10; pc: +2);
        test!("idy", &mut core, rom!(A, 0, X, 0x00, Y, 0x10; 0xB1, 0x40), a:10; pc: +2);
        test!("zpg", &mut core, rom!(A, 0, X, 0x00, Y, 0x00; 0xA5, 0x05), a:10; pc: +2);
        test!("zpx", &mut core, rom!(A, 0, X, 0x02, Y, 0x00; 0xB5, 0x03), a:10; pc: +2);
    }

    #[test]
    fn ldx() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0, X, 0, Y, 0; 0xA2, 10), x:10; pc: +2, ps: 0);
        test!("imm_z", &mut core, rom!(A, 0, X, 12, Y, 0; 0xA2, 0b0000), x:0; pc: +2, ps: Z);
        test!("imm_n", &mut core, rom!(A, 0, X, 1, Y, 0; 0xA2, 0xFF), x:0xFF; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0, X, 0, Y, 0x00; 0xAE, 0x05, 0x00), x:10; pc: +3);
        test!("aby", &mut core, rom!(A, 0, X, 0, Y, 0x01; 0xBE, 0x04, 0x00), x:10; pc: +3);
        test!("zpg", &mut core, rom!(A, 0, X, 0, Y, 0x00; 0xA6, 0x05), x:10; pc: +2);
        test!("zpy", &mut core, rom!(A, 0, X, 0, Y, 0x02; 0xB6, 0x03), x:10; pc: +2);
    }

    #[test]
    fn ldy() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0, X, 0, Y, 0; 0xA0, 10), y:10; pc: +2, ps: 0);
        test!("imm_z", &mut core, rom!(A, 0, X, 0, Y, 1; 0xA0, 0b0000), y:0; pc: +2, ps: Z);
        test!("imm_n", &mut core, rom!(A, 0, X, 0, Y, 2; 0xA0, 0xFF), y:0xFF; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0, X, 0x00, Y, 0; 0xAC, 0x05, 0x00), y:10; pc: +3);
        test!("abx", &mut core, rom!(A, 0, X, 0x01, Y, 0; 0xBC, 0x04, 0x00), y:10; pc: +3);
        test!("zpg", &mut core, rom!(A, 0, X, 0x00, Y, 0; 0xA4, 0x05), y:10; pc: +2);
        test!("zpx", &mut core, rom!(A, 0, X, 0x01, Y, 0; 0xB4, 0x04), y:10; pc: +2);
    }

    #[test]
    fn sta() {
        let mut core = NoveCore::new();
        core.memory.write(0x0050, 0x0005);

        test!("abs", &mut core, rom!(A, 10, X, 0x00, Y, 0x00; 0x8D, 0x05, 0x00), 0x0005:10; pc: +3);
        test!("abx", &mut core, rom!(A, 10, X, 0x02, Y, 0x00; 0x9D, 0x03, 0x00), 0x0005:10; pc: +3);
        test!("aby", &mut core, rom!(A, 10, X, 0x00, Y, 0x01; 0x99, 0x04, 0x00), 0x0005:10; pc: +3);
        test!("idx", &mut core, rom!(A, 10, X, 0x20, Y, 0x00; 0x81, 0x30), 0x0005:10; pc: +2);
        test!("idy", &mut core, rom!(A, 10, X, 0x00, Y, 0x10; 0x91, 0x40), 0x0005:10; pc: +2);
        test!("zpg", &mut core, rom!(A, 10, X, 0x00, Y, 0x00; 0x85, 0x05), 0x0005:10; pc: +2);
        test!("zpx", &mut core, rom!(A, 10, X, 0x02, Y, 0x00; 0x95, 0x03), 0x0005:10; pc: +2);
    }

    #[test]
    fn tax() {
        let mut core = NoveCore::new();

        test!("imm", &mut core, rom!(A, 10, X, 5, Y, 0; 0xAA), x:10; pc: +1, ps: 0);
        test!("imm_z", &mut core, rom!(A, 0, X, 5, Y, 0; 0xAA), x:00; pc: +1, ps: Z);
        test!("imm_n", &mut core, rom!(A, 0xFF, X, 5, Y, 0; 0xAA), x:0xFF; pc: +1, ps: N);
    }

    fn preloaded_core() -> NoveCore {
        let mut core = NoveCore::new();
        core.memory.write(0x0005, 10);
        core.memory.write(0x0050, 0x0005);
        core
    }

}