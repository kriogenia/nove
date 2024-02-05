use std::ops::Range;
use crate::exception::Exception;
use crate::instruction::Instruction;
use crate::OP_CODE_SLICE_SIZE;

#[derive(Default, Debug)]
pub struct CPU {
    pub program_counter: u16,
    pub register_a: u8,
    /// N V _ B D I Z C
    pub processor_status: u8,
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
                    self.register_a = param;

                    // todo move flag logic to struct
                    // raise/low zero flag if the loaded value is zero
                    if self.register_a == 0 {
                        self.processor_status = self.processor_status | 0b0000_0010;
                    } else {
                        self.processor_status = self.processor_status & 0b1111_1101;
                    }

                    // raise/lower neg flag if the load value is zero
                    if self.register_a & 0b1000_0000 != 0 {
                        self.processor_status = self.processor_status | 0b1000_0000;
                    } else {
                        self.processor_status = self.processor_status & 0b0111_1111;
                    }
                }
            }

        }

        Ok(())
    }

}


fn next_slice_range(start: usize, len: usize) -> Range<usize> {
    return if start + OP_CODE_SLICE_SIZE > len {
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
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.processor_status & 0b0000_0010, 0);
        assert_eq!(cpu.processor_status & 0b1000_0000, 0);
        assert_eq!(cpu.program_counter, 3);

        cpu.run(vec![0xa9, 0x00, 0x00]).unwrap();
        assert_eq!(cpu.processor_status >> 1, 1);

        cpu.run(vec![0xa9, 0xF0, 0x00]).unwrap();
        assert_eq!(cpu.processor_status >> 7, 1);
    }

}