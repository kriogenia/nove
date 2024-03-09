#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    /// Accumulator
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IMP
    ACC,
    /// Absolute
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ABS
    ABS,
    /// Absolute,X
    ///
    /// Can add 1 cycle if page crossed
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ABX
    ABX,
    /// Absolute,Y
    ///
    /// Can add 1 cycle if page crossed
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ABY
    ABY,
    /// Indirect
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IND
    IND,
    /// Indexed Indirect
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IDX
    IDX,
    /// Indirect Indexed
    ///
    /// Can add 1 cycle if page crossed
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IDY
    IDY,
    /// Immediate
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IMM
    IMM,
    /// Implied
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#IMP
    IMP,
    /// Relative
    ///
    /// All REL jumps +1 if branch succeeds and +2 if it does to a new page
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#REL
    REL,
    /// Zero Page
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ZPG
    ZPG,
    /// Zero Page,X
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ZPX
    ZPX,
    /// Zero Page,Y
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/addressing.html#ZPY
    ZPY,
}
