mod memory;
mod ops;
mod processor_status;
mod register;
mod stack_pointer;

use crate::core::memory::Memory;
use crate::core::ops::{Direction, Displacement};
use crate::core::processor_status::{Flag, ProcessorStatus, OVERFLOW_MASK};
use crate::core::register::Register;
use crate::core::stack_pointer::StackPointer;
use crate::exception::Exception;
use crate::instruction::addressing_mode::AddressingMode;
use crate::instruction::addressing_mode::AddressingMode::ACC;
use crate::instruction::{mnemonic::Mnemonic, OpCode, OPCODES_MAP};
use crate::Rom;
use std::fmt::{Debug, Formatter};
use std::ops::{AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, SubAssign};

#[derive(Default)]
pub struct NoveCore {
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    sp: StackPointer,
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
        format!("{:#04x}", $val)
    };
}

/// Composes an operation over a register and updates zn
macro_rules! op_and_assign {
    ($core:expr, $reg:ident.$op:ident, $val:expr) => {{
        $core.$reg.$op($val);
        $core.update_zn($core.$reg.get());
    }};
}

macro_rules! compare {
    ($core:expr, $reg:ident, $addr:expr) => {{
        let (result, carry) = $core.$reg.overflowing_sub($core.memory.read($addr));
        $core.ps.set_bit(Flag::Carry, carry);
        $core.update_zn(result);
    }};
}

macro_rules! displace {
    ($core:expr, $displacement:expr, acc) => {{
        let val = $core.a.get();
        let (val, carry) = $displacement.displace(val);
        $core.ps.set_bit(Flag::Carry, carry);
        $core.a.assign(val);
        $core.update_zn(val);
    }};
    ($core:expr, $displacement:expr, mem:$addr:expr) => {{
        let val = $core.memory.read($addr);
        let (val, carry) = $displacement.displace(val);
        $core.ps.set_bit(Flag::Carry, carry);
        $core.memory.write($addr, val);
        $core.update_zn(val);
    }};
}

impl NoveCore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.pc = self.memory.read_u16(memory::PC_START_ADDR);
        self.sp = Default::default();
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
                }
                AND => op_and_assign!(self, a.bitand_assign, self.memory.read(addr)),
                ASL if opcode.addressing_mode == ACC => {
                    displace!(self, Displacement::Shift(Direction::Left), acc)
                }
                ASL => displace!(self, Displacement::Shift(Direction::Left), mem:addr),
                BCC => self.branch_if(self.ps.is_lowered(Flag::Carry), addr),
                BCS => self.branch_if(self.ps.is_raised(Flag::Carry), addr),
                BEQ => self.branch_if(self.ps.is_raised(Flag::Zero), addr),
                BIT => self.bit_test(self.memory.read(addr)),
                BMI => self.branch_if(self.ps.is_raised(Flag::Negative), addr),
                BNE => self.branch_if(self.ps.is_lowered(Flag::Zero), addr),
                BPL => self.branch_if(self.ps.is_lowered(Flag::Negative), addr),
                BVC => self.branch_if(self.ps.is_lowered(Flag::Overflow), addr),
                CLC => self.ps.set_bit(Flag::Carry, false),
                CLV => self.ps.set_bit(Flag::Overflow, false),
                CMP => compare!(self, a, addr),
                CPX => compare!(self, x, addr),
                CPY => compare!(self, y, addr),
                DEX => op_and_assign!(self, x.sub_assign, 1),
                EOR => op_and_assign!(self, a.bitxor_assign, self.memory.read(addr)),
                INC => {
                    self.memory.update(addr, |prev| prev.wrapping_add(1));
                    self.update_zn(self.memory.read(addr))
                }
                INX => op_and_assign!(self, x.add_assign, 1),
                JMP => self.pc = addr,
                NOP => {}
                LDA => op_and_assign!(self, a.assign, self.memory.read(addr)),
                LDX => op_and_assign!(self, x.assign, self.memory.read(addr)),
                LDY => op_and_assign!(self, y.assign, self.memory.read(addr)),
                ORA => op_and_assign!(self, a.bitor_assign, self.memory.read(addr)),
                PHA => self.stack_push(self.a.get()),
                PHP => self.stack_push(self.ps.get_for_push()),
                PLA => {
                    let val = self.stack_pull();
                    op_and_assign!(self, a.assign, val)
                }
                PLP => {
                    let val = self.stack_pull();
                    self.ps.set_from_pull(val)
                }
                ROL if opcode.addressing_mode == ACC => displace!(
                    self,
                    Displacement::Rotation(Direction::Left, self.ps.is_raised(Flag::Carry)),
                    acc
                ),
                ROL => {
                    displace!(self, Displacement::Rotation(Direction::Left, self.ps.is_raised(Flag::Carry)), mem:addr)
                }
                ROR if opcode.addressing_mode == ACC => displace!(
                    self,
                    Displacement::Rotation(Direction::Right, self.ps.is_raised(Flag::Carry)),
                    acc
                ),
                ROR => {
                    displace!(self, Displacement::Rotation(Direction::Right, self.ps.is_raised(Flag::Carry)), mem:addr)
                }
                SEC => self.ps.set_bit(Flag::Carry, true),
                SEI => self.ps.set_bit(Flag::Interrupt, true),
                SBC => {
                    let diff = self.sbc(self.memory.read(addr));
                    op_and_assign!(self, a.assign, diff);
                }
                STA => self.memory.write(addr, self.a.get()),
                STX => self.memory.write(addr, self.x.get()),
                STY => self.memory.write(addr, self.y.get()),
                TAX => op_and_assign!(self, x.transfer, &self.a),
                TAY => op_and_assign!(self, y.transfer, &self.a),
                TXA => op_and_assign!(self, a.transfer, &self.x),
                TYA => op_and_assign!(self, a.transfer, &self.y),
            }

            self.update_pc(opcode);
        }

        Ok(())
    }

    fn get_addr(&self, mode: &AddressingMode) -> u16 {
        use AddressingMode::*;
        match mode {
            IMM => self.pc,
            REL => self.pc.wrapping_add(self.next_byte() as u16),
            ZPG => self.next_byte() as u16,
            ZPX => self.next_byte().wrapping_add(self.x.get()) as u16,
            ZPY => self.next_byte().wrapping_add(self.y.get()) as u16,
            ABS => self.next_word(),
            ABX => self.next_word().wrapping_add(self.x.get() as u16),
            ABY => self.next_word().wrapping_add(self.y.get() as u16),
            IND => {
                // todo get_addr_ind
                // 6502 was bugged when reading end-of-page addresses like $03FF, in those cases
                // instead of reading from $03FF and $0400 it took the values from $03FF and $0300
                let address = self.next_word();
                if address & 0x00FF == 0x00FF {
                    let lo = self.memory.read(address);
                    let hi = self.memory.read(address & 0xFF00);
                    u16::from_le_bytes([lo, hi])
                } else {
                    self.memory.read_u16(address)
                }
            }
            IDX => self
                .memory
                .read_u16(self.next_byte().wrapping_add(self.x.get()) as u16),
            IDY => self
                .memory
                .read_u16(self.next_byte().wrapping_add(self.y.get()) as u16),
            IMP | ACC => Default::default(),
        }
    }

    fn next_byte(&self) -> u8 {
        self.memory.read(self.pc)
    }

    fn next_word(&self) -> u16 {
        self.memory.read_u16(self.pc)
    }

    fn stack_push(&mut self, content: u8) {
        self.memory.write(self.sp.get(), content);
        self.sp.next()
    }

    fn stack_pull(&mut self) -> u8 {
        self.sp.prev();
        self.memory.read(self.sp.get())
    }

    fn adc(&mut self, m: u8) -> u8 {
        let a = self.a.get();

        let first = self.ps.get_bit(Flag::Carry).overflowing_add(a);
        let (result, carry) = first.0.overflowing_add(m);

        self.ps.set_bit(Flag::Carry, first.1 || carry);
        self.ps.set_bit(
            Flag::Overflow,
            ((a & m & !result) | (!a & !m & result)) & OVERFLOW_MASK != 0,
        );

        result
    }

    fn bit_test(&mut self, value: u8) {
        self.update_z(&self.a & value);
        self.update_n(value);
        self.update_v(value);
    }

    fn sbc(&mut self, m: u8) -> u8 {
        self.adc(m.wrapping_neg().wrapping_sub(1))
    }

    #[inline]
    fn branch_if(&mut self, condition: bool, addr: u16) {
        if condition {
            self.pc = addr
        }
    }

    #[inline]
    fn update_zn(&mut self, value: u8) {
        self.update_z(value);
        self.update_n(value);
    }

    #[inline]
    fn update_z(&mut self, value: u8) {
        if value == 0 {
            self.ps.raise(Flag::Zero);
        } else {
            self.ps.low(Flag::Zero);
        }
    }

    #[inline]
    fn update_n(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.ps.raise(Flag::Negative)
        } else {
            self.ps.low(Flag::Negative);
        }
    }

    #[inline]
    fn update_v(&mut self, value: u8) {
        if value & 0b0100_0000 != 0 {
            self.ps.raise(Flag::Overflow)
        } else {
            self.ps.low(Flag::Overflow);
        }
    }

    #[inline]
    fn update_pc(&mut self, opcode: &OpCode) {
        self.pc += opcode.bytes as u16 - 1;
    }

    #[cfg(test)]
    fn load_and_run(&mut self, rom: Rom) {
        self.load(rom);
        self.reset();
        self.run().expect("error while running the program")
    }
}

impl Debug for NoveCore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NovaCode {{ ")?;
        writeln!(f, "\tpc: {}", self.pc)?;
        writeln!(f, "\t a: {}", hexprint!(self.a.get()))?;
        writeln!(f, "\t x: {}", hexprint!(self.x.get()))?;
        writeln!(f, "\t y: {}", hexprint!(self.y.get()))?;
        writeln!(f, "\tps: {:?}", self.ps)?;
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const START_ADDR: u16 = memory::PRG_ROM_ADDR as u16;

    const A: u8 = 0xA9;
    const X: u8 = 0xA2;
    const Y: u8 = 0xA0;

    const C: u8 = Flag::Carry as u8;
    const I: u8 = Flag::Interrupt as u8;
    const N: u8 = Flag::Negative as u8;
    const Z: u8 = Flag::Zero as u8;
    const V: u8 = Flag::Overflow as u8;

    const SET_C: u8 = 0x38;

    /// Runs a tests with the given core and rom checking the list of registers or addresses and the pc addition
    macro_rules! test {
        ($id:expr, $core:expr, $rom:expr, $($reg:ident: $val:literal),+; pc: +$pc:literal $(, ps: $ps:expr)*) => {
            println!($id);
            $core.load_and_run($rom);
            $(assert_eq!($core.$reg, $val);)+
            assert_eq!($core.pc, memory::PRG_ROM_ADDR as u16 + $pc + 7);
            $(assert_eq!($core.ps.0, $ps);)*
        };
        ($id:expr, $core:expr, $rom:expr, $($addr:literal: $val:literal),*; pc: +$pc:literal $(, ps: $ps:expr)*) => {
            println!($id);
            $core.load_and_run($rom);
            $({
                assert_eq!($core.memory.read_u16($addr), $val);
            })*
            assert_eq!($core.pc, memory::PRG_ROM_ADDR as u16 + $pc + 7);
            $(assert_eq!($core.ps.0, $ps);)*
        };
        ($id:expr, $core:expr, $rom:expr; pc: $pc:expr $(, ps: $ps:expr)*) => {
            println!($id);
            $core.load_and_run($rom);
            assert_eq!($core.pc, $pc);
            $(assert_eq!($core.ps.0, $ps);)*
        };
    }

    macro_rules! rom {
        ($($opcode:expr),+) => {
            vec![$($opcode),+, 0x00]
        };
        ($($ld:expr),+; $($opcode:expr),+) => {
            vec![$($ld),+, $($opcode),+, 0x00]
        };
    }

    #[test]
    fn adc() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0x00, X, 0x00, Y, 0x00; 0x69, 0x10), a:0x10; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0x00, X, 0x00, Y, 0x00; 0x69, 0x00), a:0x00; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0xf0, X, 0x00, Y, 0x00; 0x69, 0x05), a:0xF5; pc: +2, ps: N);
        test!("ovf", &mut core, rom!(A, 0x7f, X, 0x00, Y, 0x00; 0x69, 0x01), a:0x80; pc: +2, ps: V+N);
        test!("car", &mut core, rom!(A, 0xff, X, 0x00, Y, 0x00; 0x69, 0x02), a:0x01; pc: +2, ps: C);
        test!("abs", &mut core, rom!(A, 0x20, X, 0x00, Y, 0x00; 0x6d, 0x05, 0x00), a:0x2a; pc: +3);
        test!("abx", &mut core, rom!(A, 0x20, X, 0x02, Y, 0x00; 0x7d, 0x03, 0x00), a:0x2a; pc: +3);
        test!("aby", &mut core, rom!(A, 0x20, X, 0x00, Y, 0x01; 0x79, 0x04, 0x00), a:0x2a; pc: +3);
        test!("idx", &mut core, rom!(A, 0x20, X, 0x20, Y, 0x00; 0x61, 0x30), a:0x2a; pc: +2);
        test!("idy", &mut core, rom!(A, 0x20, X, 0x00, Y, 0x10; 0x71, 0x40), a:0x2a; pc: +2);
        test!("zpg", &mut core, rom!(A, 0x20, X, 0x00, Y, 0x00; 0x65, 0x05), a:0x2a; pc: +2);
        test!("zpx", &mut core, rom!(A, 0x20, X, 0x02, Y, 0x00; 0x75, 0x03), a:0x2a; pc: +2);
    }

    #[test]
    fn and() {
        let mut core = preloaded_core(); // 0x0005:0b1010

        test!("imm", &mut core, rom!(A, 0b00001010, X, 0x00, Y, 0x00; 0x29, 0b1100), a:0b1000; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0b00001010, X, 0x00, Y, 0x00; 0x29, 0b0000), a:0b0000; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0b11111111, X, 0x00, Y, 0x00; 0x29, 0b11110000), a:0b11110000; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x00; 0x2d, 0x05, 0x00), a:0b0010; pc: +3);
        test!("abx", &mut core, rom!(A, 0b00000110, X, 0x02, Y, 0x00; 0x3d, 0x03, 0x00), a:0b0010; pc: +3);
        test!("aby", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x01; 0x39, 0x04, 0x00), a:0b0010; pc: +3);
        test!("idx", &mut core, rom!(A, 0b00000110, X, 0x20, Y, 0x00; 0x21, 0x30), a:0b0010; pc: +2);
        test!("idy", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x10; 0x31, 0x40), a:0b0010; pc: +2);
        test!("zpg", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x00; 0x25, 0x05), a:0b0010; pc: +2);
        test!("zpx", &mut core, rom!(A, 0b00000110, X, 0x02, Y, 0x00; 0x35, 0x03), a:0b0010; pc: +2);
    }

    #[test]
    fn asl() {
        let mut core = preloaded_core();

        test!("acc", &mut core, rom!(A, 0b0001_0101, X, 0, Y, 0; 0x0a), a:0b0010_1010; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0b0000_0000, X, 0, Y, 0; 0x0a), a:0b0000_0000; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0b0101_1001, X, 0, Y, 0; 0x0a), a:0b1011_0010; pc: +1, ps: N);
        test!("car", &mut core, rom!(A, 0b1001_0101, X, 0, Y, 0, 0x0a), a:0b0010_1010; pc: +1, ps: C);
        test!("abs", &mut core, rom!(A, 0, X, 0, Y, 0; 0x0e, 0x05, 0x00), 0x0005:0b0001_0100; pc: +3);
        test!("abx", &mut core, rom!(A, 0, X, 2, Y, 0; 0x1e, 0x03, 0x00), 0x0005:0b0010_1000; pc: +3);
        test!("zpg", &mut core, rom!(A, 0, X, 0, Y, 0; 0x06, 0x05), 0x0005:0b0101_0000; pc: +2);
        test!("zpx", &mut core, rom!(A, 0, X, 2, Y, 0; 0x16, 0x03), 0x0005:0b1010_0000; pc: +2);
    }

    #[test]
    fn bcc() {
        test_branch(rom!(0x90, 0x03), 0x03);
    }

    #[test]
    fn bcs() {
        test_branch(rom!(SET_C; 0xb0, 0x03), 0x03 + 1);
    }

    #[test]
    fn beq() {
        test_branch(rom!(A, 0; 0xf0, 0x03), 0x03 + 2);
    }

    #[test]
    fn bit() {
        let mut core = preloaded_core();
        core.memory.write(0x0010, 0b0100_0000);
        core.memory.write(0x0020, 0b1000_0000);
        core.memory.write(0x0030, 0b1111_0000);

        test!("abs", &mut core, rom!(A, 0b0000_0011, X, 0, Y, 0; 0x2c, 0x05, 0x00),; pc: +3, ps:0);
        test!("zpg", &mut core, rom!(A, 0b0000_1100, X, 0, Y, 0; 0x24, 0x05),; pc: +2, ps:0);
        test!("zer", &mut core, rom!(A, 0b0000_0101, X, 0, Y, 0; 0x24, 0x05),; pc: +2, ps:Z);
        test!("ovf", &mut core, rom!(A, 0b1111_1111, X, 0, Y, 0; 0x24, 0x10),; pc: +2, ps:V);
        test!("neg", &mut core, rom!(A, 0b1111_1111, X, 0, Y, 0; 0x24, 0x20),; pc: +2, ps:N);
        test!("zvn", &mut core, rom!(A, 0b0000_1111, X, 0, Y, 0; 0x24, 0x30),; pc: +2, ps:Z+V+N);
    }

    #[test]
    fn bmi() {
        test_branch(rom!(A, 0_u8.wrapping_sub(2); 0x30, 0x03), 0x03 + 2);
    }

    #[test]
    fn bne() {
        test_branch(rom!(0xd0, 0x03), 0x03);
    }

    #[test]
    fn bpl() {
        test_branch(rom!(0x10, 0x03), 0x03);
    }

    #[test]
    fn bvc() {
        test_branch(rom!(0x50, 0x03), 0x03);
    }

    #[test]
    fn clc() {
        let mut core = NoveCore::default();
        core.ps.set_bit(Flag::Carry, true);
        test!("clc", &mut core, rom!(A, 1, X, 1, Y, 1, 0x18), a:1; pc: +1, ps:0);
    }

    #[test]
    fn clv() {
        let mut core = NoveCore::default();
        core.ps.set_bit(Flag::Overflow, true);
        test!("clc", &mut core, rom!(A, 1, X, 1, Y, 1, 0xB8), a:1; pc: +1, ps:0);
    }

    #[test]
    fn cmp() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0x20, X, 0x00, Y, 0x00; 0xc9, 0x10), a:0x20; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0x20, X, 0x00, Y, 0x00; 0xc9, 0x20), a:0x20; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0xff, X, 0x00, Y, 0x00; 0xc9, 0x0f), a:0xff; pc: +2, ps: N);
        test!("car", &mut core, rom!(A, 0x00, X, 0x00, Y, 0x00; 0xc9, 0xff), a:0x00; pc: +2, ps: C);
        test!("abs", &mut core, rom!(A, 0x0a, X, 0x00, Y, 0x00; 0xcd, 0x05, 0x00), a:0x0a; pc: +3, ps: Z);
        test!("abx", &mut core, rom!(A, 0x0a, X, 0x02, Y, 0x00; 0xdd, 0x03, 0x00), a:0x0a; pc: +3, ps: Z);
        test!("aby", &mut core, rom!(A, 0x0a, X, 0x00, Y, 0x01; 0xd9, 0x04, 0x00), a:0x0a; pc: +3, ps: Z);
        test!("idx", &mut core, rom!(A, 0x0a, X, 0x20, Y, 0x00; 0xc1, 0x30), a:0x0a; pc: +2, ps: Z);
        test!("idy", &mut core, rom!(A, 0x0a, X, 0x00, Y, 0x10; 0xd1, 0x40), a:0x0a; pc: +2, ps: Z);
        test!("zpg", &mut core, rom!(A, 0x0a, X, 0x00, Y, 0x00; 0xc5, 0x05), a:0x0a; pc: +2, ps: Z);
        test!("zpx", &mut core, rom!(A, 0x0a, X, 0x02, Y, 0x00; 0xd5, 0x03), a:0x0a; pc: +2, ps: Z);
    }

    #[test]
    fn cpx() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0x20, X, 0x20, Y, 0x00; 0xe0, 0x10), a:0x20; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0x20, X, 0x20, Y, 0x00; 0xe0, 0x20), a:0x20; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0xff, X, 0xff, Y, 0x00; 0xe0, 0x0f), a:0xff; pc: +2, ps: N);
        test!("car", &mut core, rom!(A, 0x00, X, 0x00, Y, 0x00; 0xe0, 0xff), a:0x00; pc: +2, ps: C);
        test!("abs", &mut core, rom!(A, 0x0a, X, 0x0a, Y, 0x00; 0xec, 0x05, 0x00), a:0x0a; pc: +3, ps: Z);
        test!("zpg", &mut core, rom!(A, 0x0a, X, 0x0a, Y, 0x00; 0xe4, 0x05), a:0x0a; pc: +2, ps: Z);
    }

    #[test]
    fn cpy() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0x20, X, 0x20, Y, 0x20; 0xc0, 0x10), a:0x20; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0x20, X, 0x20, Y, 0x20; 0xc0, 0x20), a:0x20; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0xff, X, 0xff, Y, 0xff; 0xc0, 0x0f), a:0xff; pc: +2, ps: N);
        test!("car", &mut core, rom!(A, 0x00, X, 0x00, Y, 0x00; 0xc0, 0xff), a:0x00; pc: +2, ps: C);
        test!("abs", &mut core, rom!(A, 0x0a, X, 0x0a, Y, 0x0a; 0xcc, 0x05, 0x00), a:0x0a; pc: +3, ps: Z);
        test!("zpg", &mut core, rom!(A, 0x0a, X, 0x0a, Y, 0x0a; 0xc4, 0x05), a:0x0a; pc: +2, ps: Z);
    }

    #[test]
    fn dex() {
        let mut core = NoveCore::default();

        test!("dex", &mut core, rom!(A, 0, X, 5, Y, 0; 0xca), x:0x04; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0, X, 1, Y, 0; 0xca), x:0x00; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0, X, 0, Y, 0; 0xca), x:0xff; pc: +1, ps: N);
    }

    #[test]
    fn eor() {
        let mut core = preloaded_core(); // 0x0005:0b1010

        test!("imm", &mut core, rom!(A, 0b00001010, X, 0x00, Y, 0x00; 0x49, 0b1100), a:0b0110; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0b00001010, X, 0x00, Y, 0x00; 0x49, 0b1010), a:0b0000; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0b11110000, X, 0x00, Y, 0x00; 0x49, 0b00101010), a:0b11011010; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x00; 0x4d, 0x05, 0x00), a:0b1100; pc: +3);
        test!("abx", &mut core, rom!(A, 0b00000110, X, 0x02, Y, 0x00; 0x5d, 0x03, 0x00), a:0b1100; pc: +3);
        test!("aby", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x01; 0x59, 0x04, 0x00), a:0b1100; pc: +3);
        test!("idx", &mut core, rom!(A, 0b00000110, X, 0x20, Y, 0x00; 0x41, 0x30), a:0b1100; pc: +2);
        test!("idy", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x10; 0x51, 0x40), a:0b1100; pc: +2);
        test!("zpg", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x00; 0x45, 0x05), a:0b1100; pc: +2);
        test!("zpx", &mut core, rom!(A, 0b00000110, X, 0x02, Y, 0x00; 0x55, 0x03), a:0b1100; pc: +2);
    }

    #[test]
    fn inc() {
        let mut core = preloaded_core();

        test!("abs", &mut core, rom!(A, 0, X, 0x00, Y, 0; 0xee, 0x05, 0x00), 0x0005:11; pc: +3);
        test!("abx", &mut core, rom!(A, 0, X, 0x02, Y, 0; 0xfe, 0x03, 0x00), 0x0005:12; pc: +3);
        test!("zpg", &mut core, rom!(A, 0, X, 0x00, Y, 0; 0xe6, 0x05, 0x00), 0x0005:13; pc: +2);
        test!("zpx", &mut core, rom!(A, 0, X, 0x02, Y, 0; 0xf6, 0x03, 0x00), 0x0005:14; pc: +2);

        core.memory.write(0x10, 0xfe);
        test!("neg", &mut core, rom!(A, 0, X, 0x00, Y, 0; 0xee, 0x10, 0x00), 0x0010:0xff; pc: +3, ps: N);
        test!("zer", &mut core, rom!(A, 0, X, 0x00, Y, 0; 0xee, 0x10, 0x00), 0x0010:0x00; pc: +3, ps: Z);
    }

    #[test]
    fn inx() {
        let mut core = NoveCore::default();

        test!("inx", &mut core, rom!(A, 0, X, 0x05, Y, 0; 0xe8), x:0x06; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0, X, 0xff, Y, 0; 0xe8), x:0x00; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0, X, 0xf0, Y, 0; 0xe8), x:0xF1; pc: +1, ps: N);
    }

    #[test]
    fn jmp() {
        let mut core = NoveCore::default();
        core.memory.write_u16(0x0050, 0x0100);

        test!("abs", &mut core, rom!(0x4c, 0x05, 0x00); pc: 0x0006);
        test!("ind", &mut core, rom!(0x6c, 0x50, 0x00); pc: 0x0101);
    }

    #[test]
    fn lda() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0, X, 0, Y, 0; 0xa9, 0x10), a:0x10; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 2, X, 0, Y, 0; 0xa9, 0x00), a:0x00; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 1, X, 0, Y, 0; 0xa9, 0xff), a:0xff; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0, X, 0x00, Y, 0x00; 0xad, 0x05, 0x00), a:10; pc: +3);
        test!("abx", &mut core, rom!(A, 0, X, 0x02, Y, 0x00; 0xbd, 0x03, 0x00), a:10; pc: +3);
        test!("aby", &mut core, rom!(A, 0, X, 0x00, Y, 0x01; 0xb9, 0x04, 0x00), a:10; pc: +3);
        test!("idx", &mut core, rom!(A, 0, X, 0x20, Y, 0x00; 0xa1, 0x30), a:10; pc: +2);
        test!("idy", &mut core, rom!(A, 0, X, 0x00, Y, 0x10; 0xb1, 0x40), a:10; pc: +2);
        test!("zpg", &mut core, rom!(A, 0, X, 0x00, Y, 0x00; 0xa5, 0x05), a:10; pc: +2);
        test!("zpx", &mut core, rom!(A, 0, X, 0x02, Y, 0x00; 0xb5, 0x03), a:10; pc: +2);
    }

    #[test]
    fn ldx() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0, X, 0, Y, 0; 0xa2, 0x10), x:0x10; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0, X, 2, Y, 0; 0xa2, 0x00), x:0x00; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0, X, 1, Y, 0; 0xa2, 0xff), x:0xff; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0, X, 0, Y, 0x00; 0xae, 0x05, 0x00), x:10; pc: +3);
        test!("aby", &mut core, rom!(A, 0, X, 0, Y, 0x01; 0xbe, 0x04, 0x00), x:10; pc: +3);
        test!("zpg", &mut core, rom!(A, 0, X, 0, Y, 0x00; 0xa6, 0x05), x:10; pc: +2);
        test!("zpy", &mut core, rom!(A, 0, X, 0, Y, 0x02; 0xb6, 0x03), x:10; pc: +2);
    }

    #[test]
    fn ldy() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0, X, 0, Y, 0; 0xa0, 0x10), y:0x10; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0, X, 0, Y, 1; 0xa0, 0x00), y:0x00; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0, X, 0, Y, 2; 0xa0, 0xff), y:0xff; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0, X, 0x00, Y, 0; 0xac, 0x05, 0x00), y:10; pc: +3);
        test!("abx", &mut core, rom!(A, 0, X, 0x01, Y, 0; 0xbc, 0x04, 0x00), y:10; pc: +3);
        test!("zpg", &mut core, rom!(A, 0, X, 0x00, Y, 0; 0xa4, 0x05), y:10; pc: +2);
        test!("zpx", &mut core, rom!(A, 0, X, 0x01, Y, 0; 0xb4, 0x04), y:10; pc: +2);
    }

    #[test]
    fn nop() {
        let mut core = NoveCore::new();

        test!("imp", &mut core, rom!(A, 0, X, 0, Y, 0; 0xea),; pc: +1);
    }

    #[test]
    fn ora() {
        let mut core = preloaded_core(); // 0x0005:0b1010

        test!("imm", &mut core, rom!(A, 0b00001010, X, 0x00, Y, 0x00; 0x09, 0b1100), a:0b1110; pc: +2, ps: 0);
        test!("zer", &mut core, rom!(A, 0b00000000, X, 0x00, Y, 0x00; 0x09, 0b0000), a:0b0000; pc: +2, ps: Z);
        test!("neg", &mut core, rom!(A, 0b11111001, X, 0x00, Y, 0x00; 0x09, 0b11110000), a:0b11111001; pc: +2, ps: N);
        test!("abs", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x00; 0x0d, 0x05, 0x00), a:0b1110; pc: +3);
        test!("abx", &mut core, rom!(A, 0b00000110, X, 0x02, Y, 0x00; 0x1d, 0x03, 0x00), a:0b1110; pc: +3);
        test!("aby", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x01; 0x19, 0x04, 0x00), a:0b1110; pc: +3);
        test!("idx", &mut core, rom!(A, 0b00000110, X, 0x20, Y, 0x00; 0x01, 0x30), a:0b1110; pc: +2);
        test!("idy", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x10; 0x11, 0x40), a:0b1110; pc: +2);
        test!("zpg", &mut core, rom!(A, 0b00000110, X, 0x00, Y, 0x00; 0x05, 0x05), a:0b1110; pc: +2);
        test!("zpx", &mut core, rom!(A, 0b00000110, X, 0x02, Y, 0x00; 0x15, 0x03), a:0b1110; pc: +2);
    }

    #[test]
    fn pha() {
        let mut core = NoveCore::new();
        test!("imp", &mut core, rom!(A, 0x12, X, 0, Y, 0; 0x48), 0x01ff:0x12; pc: +1);
    }

    #[test]
    fn phs() {
        let mut core = NoveCore::new();
        test!("imp", &mut core, rom!(A, 0, X, 0, Y, 0; 0x08), 0x01ff:0b0011_0010; pc: +1);
    }

    #[test]
    fn pla() {
        let mut core = NoveCore::default();
        test!("imp", &mut core, rom!(A, 2, X, 0, Y, 0, 0x08; 0x68), a:0b0011_0010; pc: +2);
    }

    #[test]
    fn plp() {
        let mut core = NoveCore::default();
        test!("imp", &mut core, rom!(A, 0b1011_0000, X, 0, Y, 0, 0x48; 0x28),; pc: +2, ps: N);
    }

    #[test]
    fn rol() {
        let mut core = preloaded_core();

        test!("acc", &mut core, rom!(A, 0b0001_0101, X, 0, Y, 0; 0x2a), a:0b0010_1010; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0b0000_0000, X, 0, Y, 0; 0x2a), a:0b0000_0000; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0b0101_1001, X, 0, Y, 0; 0x2a), a:0b1011_0010; pc: +1, ps: N);
        test!("car", &mut core, rom!(A, 0b1001_0101, X, 0, Y, 0, SET_C; 0x2a), a:0b0010_1011; pc: +2, ps: C);
        test!("abs", &mut core, rom!(A, 0, X, 0, Y, 0; 0x2e, 0x05, 0x00), 0x0005:0b0001_0100; pc: +3);
        test!("abx", &mut core, rom!(A, 0, X, 2, Y, 0; 0x3e, 0x03, 0x00), 0x0005:0b0010_1000; pc: +3);
        test!("zpg", &mut core, rom!(A, 0, X, 0, Y, 0; 0x26, 0x05), 0x0005:0b0101_0000; pc: +2);
        test!("zpx", &mut core, rom!(A, 0, X, 2, Y, 0; 0x36, 0x03), 0x0005:0b1010_0000; pc: +2);
    }

    #[test]
    fn ror() {
        let mut core = preloaded_core();

        test!("acc", &mut core, rom!(A, 0b0001_0100, X, 0, Y, 0; 0x6a), a:0b0000_1010; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0b0000_0000, X, 0, Y, 0; 0x6a), a:0b0000_0000; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0b0101_1000, X, 0, Y, 0, SET_C; 0x6a), a:0b1010_1100; pc: +2, ps: N);
        test!("car", &mut core, rom!(A, 0b1001_0101, X, 0, Y, 0, 0x6a), a:0b0100_1010; pc: +1, ps: C);
        test!("abs", &mut core, rom!(A, 0, X, 0, Y, 0; 0x6e, 0x05, 0x00), 0x0005:0b0000_0101; pc: +3);
        test!("abx", &mut core, rom!(A, 0, X, 2, Y, 0; 0x7e, 0x03, 0x00), 0x0005:0b0000_0010; pc: +3);
        test!("zpg", &mut core, rom!(A, 0, X, 0, Y, 0; 0x66, 0x05), 0x0005:0b0000_0001; pc: +2);
        test!("zpx", &mut core, rom!(A, 0, X, 2, Y, 0; 0x76, 0x03), 0x0005:0b0000_0000; pc: +2);
    }

    #[test]
    fn sbc() {
        let mut core = preloaded_core();

        test!("imm", &mut core, rom!(A, 0x08, X, 0x00, Y, 0x00; 0xe9, 0x06), a:0x01; pc: +2, ps: C);
        test!("zer", &mut core, rom!(A, 0x08, X, 0x00, Y, 0x00; 0xe9, 0x07), a:0x00; pc: +2, ps: Z+C);
        test!("neg", &mut core, rom!(A, 0x06, X, 0x00, Y, 0x00; 0xe9, 0x06), a:0xff; pc: +2, ps: N);
        test!("ovf", &mut core, rom!(A, 0x40, X, 0x00, Y, 0x00; 0xe9, 0x80), a:0xbf; pc: +2, ps: V+N);
        test!("abs", &mut core, rom!(A, 0x1b, X, 0x00, Y, 0x00; 0xed, 0x05, 0x00), a:0x10; pc: +3);
        test!("abx", &mut core, rom!(A, 0x1b, X, 0x02, Y, 0x00; 0xfd, 0x03, 0x00), a:0x10; pc: +3);
        test!("aby", &mut core, rom!(A, 0x1b, X, 0x00, Y, 0x01; 0xf9, 0x04, 0x00), a:0x10; pc: +3);
        test!("idx", &mut core, rom!(A, 0x1b, X, 0x20, Y, 0x00; 0xe1, 0x30), a:0x10; pc: +2);
        test!("idy", &mut core, rom!(A, 0x1b, X, 0x00, Y, 0x10; 0xf1, 0x40), a:0x10; pc: +2);
        test!("zpg", &mut core, rom!(A, 0x1b, X, 0x00, Y, 0x00; 0xe5, 0x05), a:0x10; pc: +2);
        test!("zpx", &mut core, rom!(A, 0x1b, X, 0x02, Y, 0x00; 0xf5, 0x03), a:0x10; pc: +2);
    }

    #[test]
    fn sec() {
        let mut core = NoveCore::new();
        test!("imp", &mut core, rom!(A, 1, X, 1, Y, 1; 0x38),; pc: +1, ps: C);
    }

    #[test]
    fn sei() {
        let mut core = NoveCore::new();
        test!("imp", &mut core, rom!(A, 1, X, 1, Y, 1; 0x78),; pc: +1, ps: I);
    }

    #[test]
    fn sta() {
        let mut core = preloaded_core();

        test!("abs", &mut core, rom!(A, 10, X, 0x00, Y, 0x00; 0x8d, 0x05, 0x00), 0x0005:10; pc: +3);
        test!("abx", &mut core, rom!(A, 11, X, 0x02, Y, 0x00; 0x9d, 0x03, 0x00), 0x0005:11; pc: +3);
        test!("aby", &mut core, rom!(A, 12, X, 0x00, Y, 0x01; 0x99, 0x04, 0x00), 0x0005:12; pc: +3);
        test!("idx", &mut core, rom!(A, 13, X, 0x20, Y, 0x00; 0x81, 0x30), 0x0005:13; pc: +2);
        test!("idy", &mut core, rom!(A, 14, X, 0x00, Y, 0x10; 0x91, 0x40), 0x0005:14; pc: +2);
        test!("zpg", &mut core, rom!(A, 15, X, 0x00, Y, 0x00; 0x85, 0x05), 0x0005:15; pc: +2);
        test!("zpx", &mut core, rom!(A, 16, X, 0x02, Y, 0x00; 0x95, 0x03), 0x0005:16; pc: +2);
    }

    #[test]
    fn stx() {
        let mut core = NoveCore::new();

        test!("abs", &mut core, rom!(A, 0, X, 10, Y, 0; 0x8e, 0x05, 0x00), 0x0005:10; pc: +3);
        test!("zpg", &mut core, rom!(A, 0, X, 11, Y, 0; 0x86, 0x05), 0x0005:11; pc: +2);
        test!("zpy", &mut core, rom!(A, 0, X, 12, Y, 2; 0x96, 0x03), 0x0005:12; pc: +2);
    }

    #[test]
    fn sty() {
        let mut core = NoveCore::new();

        test!("abs", &mut core, rom!(A, 0, X, 0, Y, 10; 0x8c, 0x05, 0x00), 0x0005:10; pc: +3);
        test!("zpg", &mut core, rom!(A, 0, X, 0, Y, 11; 0x84, 0x05), 0x0005:11; pc: +2);
        test!("zpx", &mut core, rom!(A, 0, X, 2, Y, 12; 0x94, 0x03), 0x0005:12; pc: +2);
    }

    #[test]
    fn tax() {
        let mut core = NoveCore::new();

        test!("tax", &mut core, rom!(A, 0x10, X, 5, Y, 0; 0xaa), x:0x10; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0x00, X, 5, Y, 0; 0xaa), x:0x00; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0xff, X, 5, Y, 0; 0xaa), x:0xff; pc: +1, ps: N);
    }

    #[test]
    fn tay() {
        let mut core = NoveCore::new();

        test!("tax", &mut core, rom!(A, 0x10, X, 0, Y, 5; 0xa8), y:0x10; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0x00, X, 0, Y, 5; 0xa8), y:0x00; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0xff, X, 0, Y, 5; 0xa8), y:0xff; pc: +1, ps: N);
    }

    #[test]
    fn txa() {
        let mut core = NoveCore::new();

        test!("tax", &mut core, rom!(A, 0, X, 0x05, Y, 0; 0x8a), a:0x05; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0, X, 0x00, Y, 0; 0x8a), a:0x00; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0, X, 0xff, Y, 0; 0x8a), a:0xff; pc: +1, ps: N);
    }

    #[test]
    fn tya() {
        let mut core = NoveCore::new();

        test!("tax", &mut core, rom!(A, 0, X, 0, Y, 0x05; 0x98), a:0x05; pc: +1, ps: 0);
        test!("zer", &mut core, rom!(A, 0, X, 0, Y, 0x00; 0x98), a:0x00; pc: +1, ps: Z);
        test!("neg", &mut core, rom!(A, 0, X, 0, Y, 0xff; 0x98), a:0xff; pc: +1, ps: N);
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

    #[test]
    fn sbc_ops() {
        let mut core = NoveCore::new();

        core.a.assign(8); // 8 - 6 = 2
        core.ps.set_bit(Flag::Carry, true);
        assert_eq!(core.sbc(6), 2);
        assert_eq!(core.ps.get_bit(Flag::Carry), 1);
        assert_eq!(core.ps.get_bit(Flag::Overflow), 0);

        core.a.assign(8); // 8 - 10 = -2
        core.ps.set_bit(Flag::Carry, true);
        assert_eq!(core.sbc(10), 0_u8.wrapping_sub(2));
        assert_eq!(core.ps.get_bit(Flag::Carry), 0);
        assert_eq!(core.ps.get_bit(Flag::Overflow), 0);

        core.a.assign(64); // 64 - (-128) = -64 (OV)
        core.ps.set_bit(Flag::Carry, true);
        assert_eq!(core.sbc(0_u8.wrapping_sub(128)), 0_u8.wrapping_sub(64));
        assert_eq!(core.ps.get_bit(Flag::Carry), 0);
        assert_eq!(core.ps.get_bit(Flag::Overflow), 1);

        core.a.assign(8); // 8 - 6 - C = 2
        core.ps.set_bit(Flag::Carry, false);
        assert_eq!(core.sbc(6), 1);
        assert_eq!(core.ps.get_bit(Flag::Carry), 1);
        assert_eq!(core.ps.get_bit(Flag::Overflow), 0);
    }

    #[test]
    fn addressing_mode() {
        let mut core = NoveCore::new();
        core.pc = 0x0105;
        core.x.assign(0x04);
        core.y.assign(0x10);
        core.memory.write_u16(0x0105, 0x0a01);
        core.memory.write_u16(0x0a01, 0x0b00);
        core.memory.write_u16(0x0005, 0x0c00);
        core.memory.write_u16(0x0011, 0x0d00);

        assert_eq!(core.get_addr(&AddressingMode::IMM), 0x0105);
        assert_eq!(core.get_addr(&AddressingMode::REL), 0x0106);
        assert_eq!(core.get_addr(&AddressingMode::ZPG), 0x0001);
        assert_eq!(core.get_addr(&AddressingMode::ZPX), 0x0005);
        assert_eq!(core.get_addr(&AddressingMode::ZPY), 0x0011);
        assert_eq!(core.get_addr(&AddressingMode::ABS), 0x0a01);
        assert_eq!(core.get_addr(&AddressingMode::ABX), 0x0a05);
        assert_eq!(core.get_addr(&AddressingMode::ABY), 0x0a11);
        assert_eq!(core.get_addr(&AddressingMode::IND), 0x0b00);
        assert_eq!(core.get_addr(&AddressingMode::IDX), 0x0c00);
        assert_eq!(core.get_addr(&AddressingMode::IDY), 0x0d00);

        // IND bug
        core.pc = 0x0200;
        core.memory.write_u16(0x200, 0x03ff);
        core.memory.write(0x0300, 0x12);
        core.memory.write(0x03ff, 0x34);
        core.memory.write(0x0400, 0x56);
        assert_eq!(core.get_addr(&AddressingMode::IND), 0x1234);
    }

    fn test_branch(rom: Rom, jmp: u16) {
        let mut core = NoveCore::default();
        test!("rel", &mut core, rom; pc: START_ADDR + 1 + jmp + 1 + 1);
    }

    fn preloaded_core() -> NoveCore {
        let mut core = NoveCore::new();
        core.memory.write(0x0005, 0x000a);
        core.memory.write(0x0050, 0x0005);
        core
    }
}
