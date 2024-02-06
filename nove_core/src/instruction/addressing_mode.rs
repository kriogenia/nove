#[allow(clippy::upper_case_acronyms)]
pub enum AddressingMode {
    /// Implied
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IMP
    IMP,
    /// Immediate
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IMM
    IMM,
    /// Zero Page
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ZPG
    ZPG,
    /// Zero Page,X
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ZPX
    ZPX,
    /// Absolute
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ABS
    ABS,
    /// Absolute,X
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ABX
    ABX,
    /// Absolute,Y
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ABY
    ABY,
    /// Indexed Indirect
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IDX
    IDX,
    /// Indirect Indexed
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IDY
    IDY,
}