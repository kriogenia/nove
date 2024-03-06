mod ops;
mod processor_status;
mod register;
mod stack_pointer;

use crate::cartridge::Rom;
use crate::core::ops::{Direction, Displacement};
use crate::core::processor_status::{Flag, ProcessorStatus, OVERFLOW_MASK};
use crate::core::register::Register;
use crate::core::stack_pointer::StackPointer;
use crate::exception::NoveError;
use crate::instruction::addressing_mode::AddressingMode;
use crate::instruction::{mnemonic::Mnemonic, OpCode, OPCODES_MAP};
use crate::memory::bus::Bus;
use crate::memory::cpu_mem::CpuMem;
use crate::memory::{Memory, PC_START_ADDR};
use crate::Program;
use std::fmt::{Debug, Formatter};
use std::ops::{AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, SubAssign};

pub type Core6502 = NoveCore<CpuMem>;
pub type NesNoveCore = NoveCore<Bus>;

#[derive(Default)]
pub struct NoveCore<M> {
    /// Program Counter
    pub pc: u16,
    /// Stack Pointer
    pub sp: StackPointer,
    /// Accumulator
    pub a: Register,
    /// Index Register X
    pub x: Register,
    /// Index Register Y
    pub y: Register,
    /// Processor Status
    pub ps: ProcessorStatus,
    /// Memory Map
    pub memory: M,
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
        let val = $core.memory.read($addr);
        $core.ps.set_bit(Flag::Carry, val <= $core.$reg.get());
        $core.update_zn($core.$reg.get().wrapping_sub(val));
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

macro_rules! update_mem {
    ($core:expr, $addr:expr, $op:ident) => {{
        $core.memory.update($addr, |prev| prev.$op(1));
        $core.update_zn($core.memory.read($addr))
    }};
}

impl<M: Memory> NoveCore<M> {
    pub fn reset(&mut self) {
        self.pc = self.memory.read_u16(PC_START_ADDR);
        self.sp = Default::default();
        self.a = Default::default();
        self.x = Default::default();
        self.y = Default::default();
        self.ps = ProcessorStatus::new();
    }

    #[cfg(not(test))]
    pub fn run<F>(&mut self, mut callback: F) -> Result<(), NoveError>
    where
        F: FnMut(&mut Self),
    {
        while self.tick()? {
            callback(self);
        }
        Ok(())
    }

    pub fn tick(&mut self) -> Result<bool, NoveError> {
        let byte = self.memory.read(self.pc);
        self.pc += 1;

        let opcode = OPCODES_MAP.get(&byte).ok_or(NoveError::WrongOpCode(byte))?;
        let addr = self.get_addr(&opcode.addressing_mode);

        use Mnemonic::*;
        match opcode.mnemonic {
            BRK => return Ok(false),
            ADC => {
                let sum = self.adc(self.memory.read(addr));
                op_and_assign!(self, a.assign, sum);
            }
            AND => op_and_assign!(self, a.bitand_assign, self.memory.read(addr)),
            ASL if opcode.addressing_mode == AddressingMode::ACC => {
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
            BVS => self.branch_if(self.ps.is_raised(Flag::Overflow), addr),
            CLC => self.ps.set_bit(Flag::Carry, false),
            CLD => self.ps.set_bit(Flag::Decimal, false),
            CLI => self.ps.set_bit(Flag::Interrupt, false),
            CLV => self.ps.set_bit(Flag::Overflow, false),
            CMP => compare!(self, a, addr),
            CPX => compare!(self, x, addr),
            CPY => compare!(self, y, addr),
            DEC => update_mem!(self, addr, wrapping_sub),
            DEX => op_and_assign!(self, x.sub_assign, 1),
            DEY => op_and_assign!(self, y.sub_assign, 1),
            EOR => op_and_assign!(self, a.bitxor_assign, self.memory.read(addr)),
            INC => update_mem!(self, addr, wrapping_add),
            INX => op_and_assign!(self, x.add_assign, 1),
            INY => op_and_assign!(self, y.add_assign, 1),
            JMP => self.pc = addr,
            JSR => {
                self.stack_push_u16(self.pc.wrapping_add(1));
                self.pc = addr;
            }
            NOP => {}
            LAX => {
                op_and_assign!(self, a.assign, self.memory.read(addr));
                op_and_assign!(self, x.assign, self.a.get());
            }
            LDA => op_and_assign!(self, a.assign, self.memory.read(addr)),
            LDX => op_and_assign!(self, x.assign, self.memory.read(addr)),
            LDY => op_and_assign!(self, y.assign, self.memory.read(addr)),
            LSR if opcode.addressing_mode == AddressingMode::ACC => {
                displace!(self, Displacement::Shift(Direction::Right), acc)
            }
            LSR => displace!(self, Displacement::Shift(Direction::Right), mem:addr),
            ORA => op_and_assign!(self, a.bitor_assign, self.memory.read(addr)),
            PHA => self.stack_push(self.a.get()),
            PHP => self.stack_push(self.ps.get_for_push()),
            PLA => {
                let val = self.stack_pull();
                op_and_assign!(self, a.assign, val)
            }
            PLP => self.pull_ps(),
            ROL if opcode.addressing_mode == AddressingMode::ACC => displace!(
                self,
                Displacement::Rotation(Direction::Left, self.ps.is_raised(Flag::Carry)),
                acc
            ),
            ROL => {
                displace!(self, Displacement::Rotation(Direction::Left, self.ps.is_raised(Flag::Carry)), mem:addr)
            }
            ROR if opcode.addressing_mode == AddressingMode::ACC => displace!(
                self,
                Displacement::Rotation(Direction::Right, self.ps.is_raised(Flag::Carry)),
                acc
            ),
            ROR => {
                displace!(self, Displacement::Rotation(Direction::Right, self.ps.is_raised(Flag::Carry)), mem:addr)
            }
            RTI => {
                self.pull_ps();
                let val = self.stack_pull_u16();
                self.pc = val;
            }
            RTS => self.pc = self.stack_pull_u16() + 1,
            SAX => self.memory.write(addr, self.a.get() & self.x.get()),
            SBC => {
                let diff = self.sbc(self.memory.read(addr));
                op_and_assign!(self, a.assign, diff);
            }
            SEC => self.ps.set_bit(Flag::Carry, true),
            SED => self.ps.set_bit(Flag::Decimal, true),
            SEI => self.ps.set_bit(Flag::Interrupt, true),
            STA => self.memory.write(addr, self.a.get()),
            STX => self.memory.write(addr, self.x.get()),
            STY => self.memory.write(addr, self.y.get()),
            TAX => op_and_assign!(self, x.transfer, &self.a),
            TAY => op_and_assign!(self, y.transfer, &self.a),
            TSX => op_and_assign!(self, x.assign, self.sp.0),
            TXA => op_and_assign!(self, a.transfer, &self.x),
            TXS => self.sp.0 = self.x.get(),
            TYA => op_and_assign!(self, a.transfer, &self.y),
        }

        self.update_pc(opcode);
        Ok(true)
    }

    pub fn get_addr(&self, mode: &AddressingMode) -> u16 {
        use AddressingMode::*;
        match mode {
            IMM => self.pc,
            REL => self.pc.wrapping_add(self.next_byte() as i8 as u16),
            ZPG => self.next_byte() as u16,
            ZPX => self.next_byte().wrapping_add(self.x.get()) as u16,
            ZPY => self.next_byte().wrapping_add(self.y.get()) as u16,
            ABS => self.next_word(),
            ABX => self.next_word().wrapping_add(self.x.get() as u16),
            ABY => self.next_word().wrapping_add(self.y.get() as u16),
            IDX => {
                let addr = self.next_byte().wrapping_add(self.x.get());
                let lo = self.memory.read(addr as u16);
                let hi = self.memory.read(addr.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi])
            }
            IDY => {
                let addr = self.next_byte();
                let lo = self.memory.read(addr as u16);
                let hi = self.memory.read(addr.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi]).wrapping_add(self.y.get() as u16)
            }
            IMP | ACC => Default::default(),
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
        }
    }

    pub fn next_byte(&self) -> u8 {
        self.memory.read(self.pc)
    }

    pub fn next_word(&self) -> u16 {
        self.memory.read_u16(self.pc)
    }

    fn stack_push(&mut self, content: u8) {
        self.memory.write(self.sp.get(), content);
        self.sp.next()
    }

    fn stack_push_u16(&mut self, content: u16) {
        let [lo, hi] = content.to_le_bytes();
        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn stack_pull(&mut self) -> u8 {
        self.sp.prev();
        self.memory.read(self.sp.get())
    }

    fn stack_pull_u16(&mut self) -> u16 {
        self.sp.prev();
        self.sp.prev();
        self.memory.read_u16(self.sp.get().wrapping_sub(1))
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

    fn pull_ps(&mut self) {
        let val = self.stack_pull();
        self.ps.set_from_pull(val);
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
    fn stack_peek_u16(&self) -> u16 {
        self.memory.read_u16(self.sp.get() + 1)
    }
}

impl NesNoveCore {
    pub fn new(rom: Rom) -> Self {
        Self {
            pc: Default::default(),
            sp: Default::default(),
            a: Default::default(),
            x: Default::default(),
            y: Default::default(),
            ps: Default::default(),
            memory: Bus::new(rom),
        }
    }
}

impl Core6502 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self, rom: Program) {
        self.memory.load_rom(rom);
    }

    pub fn snake_load(&mut self, rom: Program) {
        self.memory.0[0x0600..(0x0600 + rom.len())].copy_from_slice(&rom[..]);
        self.memory.write_u16(0xFFFC, 0x0600);
    }

    #[cfg(test)]
    fn run(&mut self) -> Result<(), NoveError> {
        while let Ok(true) = self.tick() {}
        Ok(())
    }

    #[cfg(test)]
    fn load_and_run(&mut self, rom: Program) {
        self.load(rom);
        self.reset();
        self.ps = Default::default();
        self.run().expect("error while running the program")
    }
}

impl<M> Debug for NoveCore<M> {
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
    use crate::memory;

    const START_ADDR: u16 = memory::PRG_ROM_START_ADDR;

    const A: u8 = 0xA9;
    const X: u8 = 0xA2;
    const Y: u8 = 0xA0;

    const C: u8 = Flag::Carry as u8;
    const I: u8 = Flag::Interrupt as u8;
    const D: u8 = Flag::Decimal as u8;
    const N: u8 = Flag::Negative as u8;
    const O: u8 = Flag::One as u8;
    const Z: u8 = Flag::Zero as u8;
    const V: u8 = Flag::Overflow as u8;

    const SET_C: u8 = 0x38;
    const PUSH_A: u8 = 0x48;
    const PUSH_PS: u8 = 0x08;

    /// Runs a tests with the given core and rom checking the list of registers or addresses and the pc addition
    macro_rules! test {
        ($core:expr, $rom:expr, $($reg:ident: $val:literal),+; pc: +$pc:literal $(, ps: $ps:expr)*) => {
            $core.load_and_run($rom);
            $({
                assert_eq!($core.$reg, $val);
            })+
            assert_eq!($core.pc, memory::PRG_ROM_START_ADDR as u16 + $pc + 7);
            $(assert_eq!($core.ps.0, $ps);)*
        };
        ($core:expr, $rom:expr, $($addr:literal: $val:literal),*; pc: +$pc:literal $(, ps: $ps:expr)*) => {
            $core.load_and_run($rom);
            $({
                assert_eq!($core.memory.read_u16($addr), $val);
            })*
            assert_eq!($core.pc, memory::PRG_ROM_START_ADDR as u16 + $pc + 7);
            $(assert_eq!($core.ps.0, $ps);)*
        };
        ($core:expr, $rom:expr; pc: $pc:expr $(, ps: $ps:expr)*) => {
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
        test!(&mut core, rom!(A, 0x00, X, 0x00, Y, 0x00; 0x69, 0x10), a:0x10; pc: +2, ps: 0);
        test!(&mut core, rom!(A, 0x7f, X, 0x00, Y, 0x00; 0x69, 0x01), a:0x80; pc: +2, ps: V+N);
        test!(&mut core, rom!(A, 0xff, X, 0x00, Y, 0x00; 0x69, 0x02), a:0x01; pc: +2, ps: C);
    }

    #[test]
    fn and() {
        let mut core = preloaded_core(); // 0x0005:0b1010
        test!(&mut core, rom!(A, 0b00001010, X, 0x00, Y, 0x00; 0x29, 0b1100), a:0b1000; pc: +2, ps: 0);
    }

    #[test]
    fn asl() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0b0001_0101, X, 0, Y, 0; 0x0a), a:0b0010_1010; pc: +1, ps: 0);
        test!(&mut core, rom!(A, 0b1001_0101, X, 0, Y, 0, 0x0a), a:0b0010_1010; pc: +1, ps: C);
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

        test!(&mut core, rom!(A, 0b0000_0011, X, 0, Y, 0; 0x2c, 0x05, 0x00),; pc: +3, ps:0);
        test!(&mut core, rom!(A, 0b0000_1111, X, 0, Y, 0; 0x24, 0x30),; pc: +2, ps:Z+V+N);
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
    fn bvs() {
        test_branch(rom!(A, 64, 0x69, 64; 0x70, 0x03), 0x03 + 4);
    }

    #[test]
    fn clc() {
        let mut core = NoveCore::default();
        core.ps.set_bit(Flag::Carry, true);
        test!(&mut core, rom!(A, 1, X, 1, Y, 1, 0x18), a:1; pc: +1, ps:0);
    }

    #[test]
    fn cld() {
        let mut core = NoveCore::default();
        core.ps.set_bit(Flag::Decimal, true);
        test!(&mut core, rom!(A, 1, X, 1, Y, 1, 0xd8), a:1; pc: +1, ps:0);
    }

    #[test]
    fn cli() {
        let mut core = NoveCore::default();
        core.ps.set_bit(Flag::Interrupt, true);
        test!(&mut core, rom!(A, 1, X, 1, Y, 1, 0x58), a:1; pc: +1, ps:0);
    }

    #[test]
    fn clv() {
        let mut core = NoveCore::default();
        core.ps.set_bit(Flag::Overflow, true);
        test!(&mut core, rom!(A, 1, X, 1, Y, 1, 0xb8), a:1; pc: +1, ps:0);
    }

    #[test]
    fn cmp() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0x00, X, 0x00, Y, 0x00; 0xc9, 0xff), a:0x00; pc: +2, ps: 0);
        test!(&mut core, rom!(A, 0x20, X, 0x00, Y, 0x00; 0xc9, 0x10), a:0x20; pc: +2, ps: C);
    }

    #[test]
    fn cpx() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0x00, X, 0x00, Y, 0x00; 0xe0, 0xff), a:0x00; pc: +2, ps: 0);
        test!(&mut core, rom!(A, 0x20, X, 0x20, Y, 0x00; 0xe0, 0x10), a:0x20; pc: +2, ps: C);
    }

    #[test]
    fn cpy() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0x00, X, 0x00, Y, 0x00; 0xc0, 0xff), a:0x00; pc: +2, ps: 0);
        test!(&mut core, rom!(A, 0x20, X, 0x20, Y, 0x20; 0xc0, 0x10), a:0x20; pc: +2, ps: C);
    }

    #[test]
    fn dec() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0, X, 0x00, Y, 0; 0xce, 0x05, 0x00), 0x0005:9; pc: +3);
    }

    #[test]
    fn dex() {
        let mut core = NoveCore::default();
        test!(&mut core, rom!(A, 0, X, 5, Y, 0; 0xca), x:0x04; pc: +1, ps: 0);
    }

    #[test]
    fn dey() {
        let mut core = NoveCore::default();
        test!(&mut core, rom!(A, 0, X, 0, Y, 5; 0x88), y:0x04; pc: +1, ps: 0);
        test!(&mut core, rom!(A, 0, X, 0, Y, 1; 0x88), y:0x00; pc: +1, ps: Z);
        test!(&mut core, rom!(A, 0, X, 0, Y, 0; 0x88), y:0xff; pc: +1, ps: N);
    }

    #[test]
    fn eor() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0b00001010, X, 0x00, Y, 0x00; 0x49, 0b1100), a:0b0110; pc: +2, ps: 0);
    }

    #[test]
    fn inc() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0, X, 0x00, Y, 0; 0xee, 0x05, 0x00), 0x0005:11; pc: +3);
    }

    #[test]
    fn inx() {
        let mut core = NoveCore::default();
        test!(&mut core, rom!(A, 0, X, 0x05, Y, 0; 0xe8), x:0x06; pc: +1, ps: 0);
    }

    #[test]
    fn iny() {
        let mut core = NoveCore::default();
        test!(&mut core, rom!(A, 0, X, 0, Y, 0x05; 0xc8), y:0x06; pc: +1, ps: 0);
    }

    #[test]
    fn jmp() {
        let mut core = NoveCore::<CpuMem>::default();
        core.memory.write_u16(0x0050, 0x0100);
        test!(&mut core, rom!(0x4c, 0x05, 0x00); pc: 0x0006);
        test!(&mut core, rom!(0x6c, 0x50, 0x00); pc: 0x0101);
    }

    #[test]
    fn jsr() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(0x20, 0x05, 0x00); pc: 0x0007);
        assert_eq!(core.stack_peek_u16(), START_ADDR + 2);
    }

    #[test]
    fn lax() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0, X, 0x00, Y, 0x00; 0xaf, 0x05, 0x00), a:10, x:10; pc: +3);
    }

    #[test]
    fn lda() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0, X, 0, Y, 0; 0xa9, 0x10), a:0x10; pc: +2, ps: 0);
    }

    #[test]
    fn ldx() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0, X, 0, Y, 0; 0xa2, 0x10), x:0x10; pc: +2, ps: 0);
    }

    #[test]
    fn ldy() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0, X, 0, Y, 0; 0xa0, 0x10), y:0x10; pc: +2, ps: 0);
    }

    #[test]
    fn lsr() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0b0101_0100, X, 0, Y, 0; 0x4a), a:0b0010_1010; pc: +1, ps: 0);
        test!(&mut core, rom!(A, 0b1001_0101, X, 0, Y, 0, 0x4a), a:0b0100_1010; pc: +1, ps: C);
    }

    #[test]
    fn nop() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 0, X, 0, Y, 0; 0xea),; pc: +1);
    }

    #[test]
    fn ora() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0b00001010, X, 0x00, Y, 0x00; 0x09, 0b1100), a:0b1110; pc: +2, ps: 0);
    }

    #[test]
    fn pha() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 0x12, X, 0, Y, 0; 0x48), 0x01fd:0x12; pc: +1);
    }

    #[test]
    fn phs() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 0, X, 0, Y, 0; 0x08), 0x01fd:0b0011_0010; pc: +1);
    }

    #[test]
    fn pla() {
        let mut core = Core6502::default();
        test!(&mut core, rom!(A, 2, X, 0, Y, 0, 0x08; 0x68), a:0b0011_0010; pc: +2);
    }

    #[test]
    fn plp() {
        let mut core = Core6502::default();
        test!(&mut core, rom!(A, 0b1011_0000, X, 0, Y, 0, 0x48; 0x28),; pc: +2, ps: N+O);
    }

    #[test]
    fn rol() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0b0001_0101, X, 0, Y, 0; 0x2a), a:0b0010_1010; pc: +1, ps: 0);
        test!(&mut core, rom!(A, 0b1001_0101, X, 0, Y, 0, SET_C; 0x2a), a:0b0010_1011; pc: +2, ps: C);
    }

    #[test]
    fn ror() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0b0001_0100, X, 0, Y, 0; 0x6a), a:0b0000_1010; pc: +1, ps: 0);
        test!(&mut core, rom!(A, 0b1001_0101, X, 0, Y, 0, 0x6a), a:0b0100_1010; pc: +1, ps: C);
    }

    #[test]
    fn rti() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0x12, PUSH_A, A, 0x00, PUSH_A, PUSH_PS; 0x40); pc: 0x1200 + 1, ps: Z+O);
    }

    #[test]
    fn rts() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0x12, PUSH_A, A, 0x00, PUSH_A; 0x60); pc: 0x1202);
    }

    #[test]
    fn sax() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 0b1010, X, 0b1100, Y, 0; 0x87, 0x25), 0x25:0b1000; pc: +2);
    }

    #[test]
    fn sbc() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 0x08, X, 0x00, Y, 0x00; 0xe9, 0x06), a:0x01; pc: +2, ps: C);
        test!(&mut core, rom!(A, 0x08, X, 0x00, Y, 0x00; 0xe9, 0x07), a:0x00; pc: +2, ps: Z+C);
        test!(&mut core, rom!(A, 0x40, X, 0x00, Y, 0x00; 0xe9, 0x80), a:0xbf; pc: +2, ps: V+N);
    }

    #[test]
    fn sec() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 1, X, 1, Y, 1; 0x38),; pc: +1, ps: C);
    }

    #[test]
    fn sed() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 1, X, 1, Y, 1; 0xf8),; pc: +1, ps: D);
    }

    #[test]
    fn sei() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 1, X, 1, Y, 1; 0x78),; pc: +1, ps: I);
    }

    #[test]
    fn sta() {
        let mut core = preloaded_core();
        test!(&mut core, rom!(A, 10, X, 0x00, Y, 0x00; 0x8d, 0x05, 0x00), 0x0005:10; pc: +3);
    }

    #[test]
    fn stx() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 0, X, 10, Y, 0; 0x8e, 0x05, 0x00), 0x0005:10; pc: +3);
    }

    #[test]
    fn sty() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 0, X, 0, Y, 10; 0x8c, 0x05, 0x00), 0x0005:10; pc: +3);
    }

    #[test]
    fn tax() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 0x10, X, 5, Y, 0; 0xaa), x:0x10; pc: +1, ps: 0);
    }

    #[test]
    fn tay() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 0x10, X, 0, Y, 5; 0xa8), y:0x10; pc: +1, ps: 0);
    }

    #[test]
    fn tsx() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 1, X, 1, Y, 1; 0xba), x:0xfd; pc: +1, ps: N);
    }

    #[test]
    fn txa() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 0, X, 0x05, Y, 0; 0x8a), a:0x05; pc: +1, ps: 0);
    }

    #[test]
    fn txs() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 0, Y, 0, X, 0x12; 0x9a), x:0x12; pc: +1);
        assert_eq!(core.sp.0, 0x12);
    }

    #[test]
    fn tya() {
        let mut core = Core6502::new();
        test!(&mut core, rom!(A, 0, X, 0, Y, 0x05; 0x98), a:0x05; pc: +1, ps: 0);
    }

    #[test]
    fn adc_ops() {
        let mut core = Core6502::new();

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
        let mut core = Core6502::new();

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
        let mut core = Core6502::new();
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
        assert_eq!(core.get_addr(&AddressingMode::IDY), 0x0010);

        // IND bug
        core.pc = 0x0200;
        core.memory.write_u16(0x200, 0x03ff);
        core.memory.write(0x0300, 0x12);
        core.memory.write(0x03ff, 0x34);
        core.memory.write(0x0400, 0x56);
        assert_eq!(core.get_addr(&AddressingMode::IND), 0x1234);
    }

    fn test_branch(rom: Program, jmp: u16) {
        let mut core = NoveCore::default();
        test!(&mut core, rom; pc: START_ADDR + 1 + jmp + 1 + 1);
    }

    fn preloaded_core() -> Core6502 {
        let mut core = Core6502::new();
        core.memory.write(0x0005, 0x000a);
        core.memory.write(0x0050, 0x0005);
        core
    }
}
