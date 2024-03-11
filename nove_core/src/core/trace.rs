use crate::core::NoveCore;
use crate::instruction;
use crate::instruction::addressing_mode::AddressingMode;
use crate::instruction::addressing_mode::AddressingMode::{ABS, ABX, ABY, IDX, IDY, ZPG, ZPX, ZPY};
use crate::memory::Memory;
use log::Level::Trace;
use log::{log_enabled, trace};

const TARGET: &'static str = "cpu";

pub(crate) trait CpuTrace {
    fn trace(&self) {}
}

impl<M: Memory> CpuTrace for NoveCore<M> {
    fn trace(&self) {
        if !log_enabled!(target: TARGET, Trace) {
            return;
        }

        use instruction::addressing_mode::AddressingMode::*;

        let pc = self.pc;
        let op = instruction::OPCODES_MAP
            .get(&self.next_byte())
            .unwrap_or_else(|| panic!("unknown opcode: {:#04x}", &self.next_byte()));

        let mut hex_dump = vec![op.code];

        let (mem_addr, stored_val) = match &op.addressing_mode {
            IMM | IMP | REL | ACC | IND => (0, 0),
            mode => {
                let addr = get_absolute_address(self, mode, pc + 1);
                (addr, self.memory.read(addr))
            }
        };

        let tmp = match op.bytes {
            1 => match op.code {
                0x0a | 0x4a | 0x2a | 0x6a => "A ".to_string(),
                0x4c | 0x20 => {
                    hex_dump.push(self.memory.read(pc + 1));
                    hex_dump.push(self.memory.read(pc + 2));
                    format!("${mem_addr:04x}")
                }
                0x6c => {
                    hex_dump.push(self.memory.read(pc + 1));
                    hex_dump.push(self.memory.read(pc + 2));

                    let addr = self.memory.read_u16(pc + 1);
                    let jmp_addr = if addr & 0x00FF == 0x00FF {
                        let lo = self.memory.read(addr);
                        let hi = self.memory.read(addr & 0xFF00);
                        u16::from_le_bytes([lo, hi])
                    } else {
                        self.memory.read_u16(addr)
                    };
                    format!("(${addr:04x}) = {jmp_addr:04x}")
                }
                _ => "".to_string(),
            },
            2 => {
                let addr = self.memory.read(pc + 1);
                hex_dump.push(addr);

                match op.addressing_mode {
                    IMM => format!("#${addr:02x}"),
                    ZPG => format!("${mem_addr:02x} = {stored_val:02x}"),
                    ZPX => format!("${addr:02x},X @ {mem_addr:02x} = {stored_val:02x}"),
                    ZPY => format!("${addr:02x},Y @ {mem_addr:02x} = {stored_val:02x}"),
                    IDX => format!(
                        "(${addr:02x},X) @ {:02x} = {mem_addr:04x} = {stored_val:02x}",
                        addr.wrapping_add(self.x.get())
                    ),
                    IDY => format!(
                        "(${addr:02x}),Y = {:04x} @ {mem_addr:04x} = {stored_val:02x}",
                        mem_addr.wrapping_sub(self.y.get() as u16)
                    ),
                    _ => format!(
                        "${:04x}",
                        (pc as usize + 2).wrapping_add((addr as i8) as usize)
                    ),
                }
            }
            3 => {
                hex_dump.push(self.memory.read(pc + 1));
                hex_dump.push(self.memory.read(pc + 2));

                let addr = self.memory.read_u16(pc + 1);

                match op.addressing_mode {
                    ABS => format!("${mem_addr:04x} = {stored_val:02x}"),
                    ABX => format!("${addr:04x},X @ {mem_addr:04x} = {stored_val:02x}"),
                    ABY => format!("${addr:04x},Y @ {mem_addr:04x} = {stored_val:02x}"),
                    _ => format!("${:04x}", addr),
                }
            }
            _ => "".to_string(),
        };

        let hex_str = hex_dump
            .iter()
            .map(|z| format!("{:02x}", z))
            .collect::<Vec<String>>()
            .join(" ");
        let unofficial = if op.unofficial { "*" } else { " " }.to_string();
        let asm_str = format!(
            "{pc:04x}  {hex_str:8} {unofficial}{:>4?} {tmp}",
            op.mnemonic
        )
        .trim()
        .to_string();

        let msg = format!(
            "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
            asm_str,
            self.a.get(),
            self.x.get(),
            self.y.get(),
            self.ps.0,
            self.sp.0,
        )
        .to_ascii_uppercase();
        trace!(target: TARGET, "{msg}")
    }
}

// todo check if we can use the base method
fn get_absolute_address<M: Memory>(core: &NoveCore<M>, mode: &AddressingMode, addr: u16) -> u16 {
    match mode {
        ABS => core.memory.read_u16(addr),
        ABX => core.memory.read_u16(addr).wrapping_add(core.x.get() as u16),
        ABY => core.memory.read_u16(addr).wrapping_add(core.y.get() as u16),
        ZPG => core.memory.read(addr) as u16,
        ZPX => core.memory.read(addr).wrapping_add(core.x.get()) as u16,
        ZPY => core.memory.read(addr).wrapping_add(core.y.get()) as u16,
        IDX => {
            let addr = core.memory.read(addr).wrapping_add(core.x.get());
            let lo = core.memory.read(addr as u16);
            let hi = core.memory.read(addr.wrapping_add(1) as u16);
            u16::from_le_bytes([lo, hi])
        }
        IDY => {
            let addr = core.memory.read(addr);
            let lo = core.memory.read(addr as u16);
            let hi = core.memory.read(addr.wrapping_add(1) as u16);
            u16::from_le_bytes([lo, hi]).wrapping_add(core.y.get() as u16)
        }
        _ => panic!("mode {:?} is not supported", mode),
    }
}
