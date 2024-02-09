#[allow(clippy::upper_case_acronyms)]
pub enum Mnemonic {
    /// ADd with Carry
    /// A,Z,C,N = A+M+C
    /// This instruction adds the contents of a memory location to the accumulator together with the carry bit.
    /// If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
    /// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    ADC,
    /// Logical AND
    /// A,Z,N = A&M
    /// A logical AND is performed, bit by bit, on the a using the contents of a byte of memory.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#AND
    AND,
    /// Force Interrupt
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BRK
    BRK,
    /// CLear Carry flag
    /// C = 0
    /// Set the carry flag to zero.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CCF
    CLC,
    /// CLear oVerflow flag
    /// V = 0
    /// Set the overflow flag to zero.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CLV
    CLV,
    /// CoMPare
    /// Z,C,N = A-M
    /// This instruction compares the contents of the accumulator with another memory held value
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CMP
    CMP,
    /// ComPare X register
    /// Z,C,N = X-M
    /// This instruction compares the contents of the X register with another memory held value
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CMP
    CPX,
    /// ComPare Y register
    /// Z,C,N = Y-M
    /// This instruction compares the contents of the Y register with another memory held value
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CPY
    CPY,
    /// DEcrement X register
    /// X,Z,N = X-1
    /// Subtracts one from the X register.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#DEX
    DEX,
    /// INcrement X register
    /// X,Z,N = X+1
    /// Adds one to the X register.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#INX
    INX,
    /// LoaD Accumulator
    /// A,Z,N = M
    /// Loads a byte of memory into the accumulator.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
    LDA,
    /// LoaD X register
    /// X,Z,N = M
    /// Loads a byte of memory into the X register.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#LDX
    LDX,
    /// LoaD Y register
    /// Y,Z,N = M
    /// Loads a byte of memory into the Y register.
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#LDY
    LDY,
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
