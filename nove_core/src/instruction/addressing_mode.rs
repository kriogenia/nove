#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum AddressingMode {
    /// Absolute
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ABS
    ABS,
    /// Absolute,X
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ABX
    ABX,
    /// Absolute,Y
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ABY
    ABY,
    /// Indirect
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IND
    IND,
    /// Indexed Indirect
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IDX
    IDX,
    /// Indirect Indexed
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IDY
    IDY,
    /// Immediate
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IMM
    IMM,
    /// Implied
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IMP
    IMP,
    /// Zero Page
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ZPG
    ZPG,
    /// Zero Page,X
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ZPX
    ZPX,
    /// Zero Page,Y
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ZPY
    ZPY,
}
