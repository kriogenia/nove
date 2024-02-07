#[allow(clippy::upper_case_acronyms)]
pub enum Mnemonic {
    /// Force Interrupt
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BRK
    BRK,
    /// Increment X Register
    /// X,Z,N = X+1
    /// Adds one to the X register.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#INX
    INX,
    /// LoaD Accumulator
    /// A,Z,N = M
    /// Loads a byte of memory into the accumulator.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
    LDA,
    /// STore Accumulator
    /// M = A
    /// Stores the contents of the accumulator into memory.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#STA
    STA,
    /// Transfer Accumulator to X
    /// X,Z,N = A
    /// Copies the current contents of the accumulator into the X register.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
    TAX,
}
