mod processor_status;
mod memory;
mod register;

use std::fmt::{Debug, Formatter};
use std::ops::{AddAssign, BitAndAssign, SubAssign};
use crate::core::memory::Memory;
use crate::core::processor_status::{Flag, OVERFLOW_MASK, ProcessorStatus};
use crate::core::register::Register;
use crate::instruction::{mnemonic::Mnemonic, OpCode, OPCODES_MAP};
use crate::Rom;
use crate::exception::Exception;
use crate::instruction::addressing_mode::AddressingMode;


#[derive(Default)]
pub struct NoveCore {
    /// Program Counter
    pc: u16,
    /// Accumulator
    a: Register,
    /// Index Register X
    x: Register,
    /// Index Register Y
    y: Register,
    /// Processor Status
    ps: ProcessorStatus,
    /// Memory Map
    memory: Memory,
}

/// Helper macro for debugging, easies the printing of hex values
#[allow(unused_macros)]
macro_rules! hexprint {
    ($val:expr) => {
        println!("{:x?}", $val);
    };
}

/// Composes an operation over a register and updates zn
macro_rules! op_and_assign {
    ($core:expr, $reg:ident.$op:ident, $val:expr) => {
        {
            $core.$reg.$op($val);
            $core.update_zn($core.$reg.get());

        }
    };
}

impl NoveCore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.pc = self.memory.read_u16(memory::PC_START_ADDR);
        self.a = Default::default();
        self.x = Default::default();
        self.y = Default::default();
        self.ps = Default::default();
    }

    pub fn load(&mut self, rom: Rom) {
        self.memory.load_rom(rom);
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        'game_loop: loop {
            let byte = self.memory.read(self.pc);
            self.pc += 1;

            let opcode = OPCODES_MAP.get(&byte).ok_or(Exception::WrongOpCode(byte))?;
            let addr = self.get_addr(&opcode.addressing_mode);

            use Mnemonic::*;
            match opcode.mnemonic {
                BRK => break 'game_loop,
                ADC => {
                    let sum = self.adc(self.memory.read(addr));
                    op_and_assign!(self, a.assign, sum);
                },
                AND => op_and_assign!(self, a.bitand_assign, self.memory.read(addr)),
                CLC => self.ps.set_bit(Flag::Carry, false),
                DEX => op_and_assign!(self, x.sub_assign, 1),
                INX => op_and_assign!(self, x.add_assign, 1),
                LDA => op_and_assign!(self, a.assign, self.memory.read(addr)),
                LDX => op_and_assign!(self, x.assign, self.memory.read(addr)),
                LDY => op_and_assign!(self, y.assign, self.memory.read(addr)),
                STA => self.memory.write(addr, self.a.get()),
                TAX => op_and_assign!(self, x.transfer, &self.a),
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
            ZPX => self.next_byte().wrapping_add(self.x.get()) as u16,
            ZPY => self.next_byte().wrapping_add(self.y.get()) as u16,
            ABS => self.next_word(),
            ABX => self.next_word().wrapping_add(self.x.get() as u16),
            ABY => self.next_word().wrapping_add(self.y.get() as u16),
            IDX => self.memory.read_u16(self.next_byte().wrapping_add(self.x.get()) as u16),
            IDY => self.memory.read_u16(self.next_byte().wrapping_add(self.y.get()) as u16),
            IMP => Default::default(),
        }
    }

    fn next_byte(&self) -> u8 {
        self.memory.read(self.pc)
    }

    fn next_word(&self) -> u16 {
        self.memory.read_u16(self.pc)
    }

    fn adc(&mut self, m: u8) -> u8 {
        let a = self.a.get();

        let first = self.ps.get_bit(Flag::Carry).overflowing_add(a);
        let (result, carry) = first.0.overflowing_add(m);

        self.ps.set_bit(Flag::Carry, first.1 || carry);
        self.ps.set_bit(Flag::Overflow, ((a & m & !result) | (!a & !m & result)) & OVERFLOW_MASK != 0);

        result
    }

    #[cfg(test)]
    fn load_and_run(&mut self, rom: Rom) {
        self.load(rom);
        self.reset();
        self.run().expect("error while running the program")
    }

    #[inline]
    fn update_zn(&mut self, value: u8) {
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
        writeln!(f, "\t a: {:x?}", self.a.get())?;
        writeln!(f, "\t x: {:x?}", self.x)?;
        writeln!(f, "\t y: {:x?}", self.y)?;
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

    const C: u8 = Flag::Carry as u8;
    const Z: u8 = Flag::Zero as u8;
    const VN: u8 = Flag::Overflow as u8 + N;
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
        ($id:expr, $core:expr, $rom:expr, $($reg:ident: $val:literal),+; pc: +$pc:literal $(, ps: $ps:expr)*) => {
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
    fn adc() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0x00, X, 0x00, Y, 0x00; 0x69, 0x10), a:0x10; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0x00, X, 0x00, Y, 0x00; 0x69, 0x00), a:0x00; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0xF0, X, 0x00, Y, 0x00; 0x69, 0x05), a:0xF5; pc: +2, ps: N);
        test!("ovf", &mut core, rom!(A, 0x7F, X, 0x00, Y, 0x00; 0x69, 0x01), a:0x80; pc: +2, ps: VN);
        test!("car", &mut core, rom!(A, 0xFF, X, 0x00, Y, 0x00; 0x69, 0x02), a:0x01; pc: +2, ps: C);
        test!("abs", &mut core, rom!(A, 0x20, X, 0x00, Y, 0x00; 0x6D, 0x05, 0x00), a:0x2a; pc: +3);
        test!("abx", &mut core, rom!(A, 0x20, X, 0x02, Y, 0x00; 0x7D, 0x03, 0x00), a:0x2a; pc: +3);
        test!("aby", &mut core, rom!(A, 0x20, X, 0x00, Y, 0x01; 0x79, 0x04, 0x00), a:0x2a; pc: +3);
        test!("idx", &mut core, rom!(A, 0x20, X, 0x20, Y, 0x00; 0x61, 0x30), a:0x2a; pc: +2);
        test!("idy", &mut core, rom!(A, 0x20, X, 0x00, Y, 0x10; 0x71, 0x40), a:0x2a; pc: +2);
        test!("zpg", &mut core, rom!(A, 0x20, X, 0x00, Y, 0x00; 0x65, 0x05), a:0x2a; pc: +2);
        test!("zpx", &mut core, rom!(A, 0x20, X, 0x02, Y, 0x00; 0x75, 0x03), a:0x2a; pc: +2);
    }

    #[test]
    fn and() {
        let mut core = preloaded_core(); // 0x0005:0b1010

        test!("imm", &mut core, rom!(A, 0b1010, X, 0x00, Y, 0x00; 0x29, 0b1100), a:0b1000; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0b1010, X, 0x00, Y, 0x00; 0x29, 0b0000), a:0b0000; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0b11111111, X, 0x00, Y, 0x00; 0x29, 0b11110000), a:0b11110000; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0b0110, X, 0x00, Y, 0x00; 0x2D, 0x05, 0x00), a:0b0010; pc: +3);
        test!("abx", &mut core, rom!(A, 0b0110, X, 0x02, Y, 0x00; 0x3D, 0x03, 0x00), a:0b0010; pc: +3);
        test!("aby", &mut core, rom!(A, 0b0110, X, 0x00, Y, 0x01; 0x39, 0x04, 0x00), a:0b0010; pc: +3);
        test!("idx", &mut core, rom!(A, 0b0110, X, 0x20, Y, 0x00; 0x21, 0x30), a:0b0010; pc: +2);
        test!("idy", &mut core, rom!(A, 0b0110, X, 0x00, Y, 0x10; 0x31, 0x40), a:0b0010; pc: +2);
        test!("zpg", &mut core, rom!(A, 0b0110, X, 0x00, Y, 0x00; 0x25, 0x05), a:0b0010; pc: +2);
        test!("zpx", &mut core, rom!(A, 0b0110, X, 0x02, Y, 0x00; 0x35, 0x03), a:0b0010; pc: +2);
    }

    #[test]
    fn clc() {
        let mut core = NoveCore::default();
        core.ps.set_bit(Flag::Carry, true);
        test!("clc", &mut core, rom!(A, 1, X, 1, Y, 1, 0x18), a:1; pc: +1, ps:0);
    }

    #[test]
    fn dex() {
        let mut core = NoveCore::default();

        test!("dex", &mut core, rom!(A, 0, X, 5, Y, 0; 0xCA), x:0x04; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0, X, 1, Y, 0; 0xCA), x:0x00; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0, X, 0, Y, 0; 0xCA), x:0xFF; pc: +1, ps: N);
    }

    #[test]
    fn inx() {
        let mut core = NoveCore::default();

        test!("inx", &mut core, rom!(A, 0, X, 0x05, Y, 0; 0xE8), x:0x06; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0, X, 0xFF, Y, 0; 0xE8), x:0x00; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0, X, 0xF0, Y, 0; 0xE8), x:0xF1; pc: +1, ps: N);
    }

    #[test]
    fn lda() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0, X, 0, Y, 0; 0xA9, 0x10), a:0x10; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 2, X, 0, Y, 0; 0xA9, 0x00), a:0x00; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 1, X, 0, Y, 0; 0xA9, 0xFF), a:0xFF; pc: +2, ps: N);
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

        test!("imm", &mut core, rom!(A, 0, X, 0, Y, 0; 0xA2, 0x10), x:0x10; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0, X, 2, Y, 0; 0xA2, 0x00), x:0x00; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0, X, 1, Y, 0; 0xA2, 0xFF), x:0xFF; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0, X, 0, Y, 0x00; 0xAE, 0x05, 0x00), x:10; pc: +3);
        test!("aby", &mut core, rom!(A, 0, X, 0, Y, 0x01; 0xBE, 0x04, 0x00), x:10; pc: +3);
        test!("zpg", &mut core, rom!(A, 0, X, 0, Y, 0x00; 0xA6, 0x05), x:10; pc: +2);
        test!("zpy", &mut core, rom!(A, 0, X, 0, Y, 0x02; 0xB6, 0x03), x:10; pc: +2);
    }

    #[test]
    fn ldy() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0, X, 0, Y, 0; 0xA0, 0x10), y:0x10; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0, X, 0, Y, 1; 0xA0, 0x00), y:0x00; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0, X, 0, Y, 2; 0xA0, 0xFF), y:0xFF; pc: +2, ps: N);
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

        test!("tax", &mut core, rom!(A, 0x10, X, 5, Y, 0; 0xAA), x:0x10; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0x00, X, 5, Y, 0; 0xAA), x:0x00; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0xFF, X, 5, Y, 0; 0xAA), x:0xFF; pc: +1, ps: N);
    }

    #[test]
    fn adc_ops() {
        let mut core = NoveCore::new();

        core.a.assign(0b0000_0000);
        assert_eq!(core.adc(0b0101_1010), 0b0101_1010);
        assert_eq!(core.ps.get_bit(Flag::Carry), 0);
        assert_eq!(core.ps.get_bit(Flag::Overflow), 0);

        core.a.assign(0b0101_1010);
        assert_eq!(core.adc(0b0101_1010), 0b1011_0100);
        assert_eq!(core.ps.get_bit(Flag::Carry), 0);
        assert_eq!(core.ps.get_bit(Flag::Overflow), 1);

        core.a.assign(0b1011_0100);
        assert_eq!(core.adc(0b1011_0100), 0b0110_1000);
        assert_eq!(core.ps.get_bit(Flag::Carry), 1);
        assert_eq!(core.ps.get_bit(Flag::Overflow), 1);

        core.a.assign(0b0111_1000);
        assert_eq!(core.adc(0b1100_0000), 0b0011_1001);
        assert_eq!(core.ps.get_bit(Flag::Carry), 1);
        assert_eq!(core.ps.get_bit(Flag::Overflow), 0);
    }

    fn preloaded_core() -> NoveCore {
        let mut core = NoveCore::new();
        core.memory.write(0x0005, 10);
        core.memory.write(0x0050, 0x0005);
        core
    }

}