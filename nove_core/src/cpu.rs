mod processor_status;

use std::ops::Range;
use crate::cpu::processor_status::{Flag, ProcessorStatus};
use crate::exception::Exception;
use crate::instruction::Instruction;
use crate::OP_CODE_SLICE_SIZE;

#[derive(Default, Debug)]
pub struct CPU {
    program_counter: u16,
    reg_a: u8,
    reg_x: u8,
    /// N V _ B D I Z C
    processor_status: ProcessorStatus,
}

impl CPU {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, opcodes: Vec<u8>) -> Result<(), Exception> {
        let size = opcodes.len();

        self.program_counter = 0;

        'game_loop: loop {
            let range = next_slice_range(self.program_counter as usize, size);
            let opcodes = &opcodes[range];
            self.program_counter += 1;

            use Instruction::*;
            match Instruction::try_from(opcodes)? {
                BRK => break 'game_loop,
                LDA(param) => {
                    self.program_counter += 1;
                    self.reg_a = param;
                    self.set_z(self.reg_a);
                    self.set_n(self.reg_a);
                },
                TAX => {
                    self.reg_x = self.reg_a;
                    self.set_z(self.reg_x);
                    self.set_n(self.reg_x);
                }
            }

        }

        Ok(())
    }

    #[inline]
    fn set_z(&mut self, value: u8) {
        if value == 0 {
            self.processor_status.raise(Flag::Zero);
        } else {
            self.processor_status.low(Flag::Zero);
        }
    }

    #[inline]
    fn set_n(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.processor_status.raise(Flag::Negative)
        } else {
            self.processor_status.low(Flag::Negative);
        }
    }

}


fn next_slice_range(start: usize, len: usize) -> Range<usize> {
    if start + OP_CODE_SLICE_SIZE > len {
        start..len
    } else {
        start..start + OP_CODE_SLICE_SIZE
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lda() {
        let mut cpu = CPU::new();
        cpu.run(vec![0xa9, 0x05, 0x00]).unwrap();
        assert_eq!(cpu.reg_a, 0x05);
        assert!(cpu.processor_status.is_lowered(Flag::Zero));
        assert!(cpu.processor_status.is_lowered(Flag::Negative));
        assert_eq!(cpu.program_counter, 3);

        cpu.run(vec![0xa9, 0x00, 0x00]).unwrap();
        assert!(cpu.processor_status.is_raised(Flag::Zero));

        cpu.run(vec![0xa9, 0xF0, 0x00]).unwrap();
        assert!(cpu.processor_status.is_raised(Flag::Negative));
    }

    #[test]
    fn tax() {
        let mut cpu = CPU::new();
        cpu.run(vec![0xa9, 0x05, 0xaa, 0x00]).unwrap();
        assert_eq!(cpu.reg_x, 0x05);
        assert!(cpu.processor_status.is_lowered(Flag::Zero));
        assert!(cpu.processor_status.is_lowered(Flag::Negative));
        assert_eq!(cpu.program_counter, 4);

        cpu.run(vec![0xa9, 0x00, 0xaa, 0x00]).unwrap();
        assert!(cpu.processor_status.is_raised(Flag::Zero));

        cpu.run(vec![0xa9, 0xF0, 0xaa, 0x00]).unwrap();
        assert!(cpu.processor_status.is_raised(Flag::Negative));
    }

}