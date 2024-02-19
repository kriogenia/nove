#[allow(clippy::upper_case_acronyms)]
pub enum Mnemonic {
    /// Add with Carry
    ///
    /// A,Z,C,N = A+M+C
    ///
    /// This instruction adds the contents of a memory location to the accumulator together with the carry bit.
    /// If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
    ///
    /// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    ADC,
    /// Logical AND
    ///
    /// A,Z,N = A&M
    ///
    /// A logical AND is performed, bit by bit, on the a using the contents of a byte of memory.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#AND
    AND,
    /// ASL - Arithmetic Shift Left
    ///
    /// A,Z,C,N = M*2 or M,Z,C,N = M*2
    ///
    /// This operation shifts all the bits one bit left.
    /// Bit 0 is set to 0 and bit 7 is placed in the carry flag.
    /// The effect of this operation is to multiply the memory contents by 2 (ignoring 2's complement considerations),
    /// setting the carry if the result will not fit in 8 bits.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#ASL
    ASL,
    /// Branch if Carry Clear
    ///
    /// If the carry flag is clear then add the relative displacement to the program counter
    /// to cause a branch to a new location.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BCC
    BCC,
    /// Branch if Carry Set
    ///
    /// If the carry flag is set then add the relative displacement to the program counter
    /// to cause a branch to a new location.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BCS
    BCS,
    /// Branch if Equal
    ///
    /// If the zero flag is set then add the relative displacement to the program counter
    /// to cause a branch to a new location.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BEQ
    BEQ,
    /// Bit Test
    ///
    /// A & M, N = M7, V = M6
    ///
    /// This instruction is used to test if one or more bits are set in a target memory location.
    /// The mask pattern in A is ANDed with the value in memory to set or clear the zero flag,
    /// but the result is not kept.
    /// Bits 7 and 6 of the value from memory are copied into the N and V flags.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BIT
    BIT,
    /// Branch if Minus
    ///
    /// If the negative flag is set then add the relative displacement to the program counter
    /// to cause a branch to a new location.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BMI
    BMI,
    /// Branch if Not Equal
    ///
    /// If the zero flag is clear then add the relative displacement to the program counter
    /// to cause a branch to a new location.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BNE
    BNE,
    /// Force Interrupt
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BRK
    BRK,
    /// Clear Carry flag
    ///
    /// C = 0
    ///
    /// Set the carry flag to zero.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CCF
    CLC,
    /// Clear Overflow flag
    ///
    /// V = 0
    ///
    /// Set the overflow flag to zero.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CLV
    CLV,
    /// Compare
    ///
    /// Z,C,N = A-M
    ///
    /// This instruction compares the contents of the accumulator with another memory held value
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CMP
    CMP,
    /// Compare X register
    ///
    /// Z,C,N = X-M
    ///
    /// This instruction compares the contents of the X register with another memory held value
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CMP
    CPX,
    /// Compare Y register
    ///
    /// Z,C,N = Y-M
    ///
    /// This instruction compares the contents of the Y register with another memory held value
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CPY
    CPY,
    /// Decrement X register
    ///
    /// X,Z,N = X-1
    ///
    /// Subtracts one from the X register.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#DEX
    DEX,
    /// Logical Exclusive OR
    ///
    /// A,Z,N = A^M
    ///
    /// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#EOR
    EOR,
    /// Increment memory
    ///
    /// M,Z,N = M+1
    ///
    /// Adds one to the value held at a specified memory location setting the zero and negative flags as appropriate.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#INC
    INC,
    /// Increment X register
    ///
    /// X,Z,N = X+1
    ///
    /// Adds one to the X register.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#INX
    INX,
    /// Jump
    ///
    /// Sets the program counter to the address specified by the operand.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
    JMP,
    /// Load Accumulator
    ///
    /// A,Z,N = M
    ///
    /// Loads a byte of memory into the accumulator.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
    LDA,
    /// Load X register
    ///
    /// X,Z,N = M
    ///
    /// Loads a byte of memory into the X register.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#LDX
    LDX,
    /// Load Y register
    ///
    /// Y,Z,N = M
    ///
    /// Loads a byte of memory into the Y register.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#LDY
    LDY,
    /// No Operation
    ///
    /// The NOP instruction causes no changes to the processor other than incrementing the PC.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#NOP
    NOP,
    /// Logical Inclusive OR
    ///
    /// A,Z,N = A|M
    ///
    /// An inclusive OR is performed on the acc contents using the contents of a byte of memory.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#ORA
    ORA,
    /// Push Accumulator
    ///
    /// Pushes a copy of the accumulator on to the stack.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#PHA
    PHA,
    /// Push Processor Status
    ///
    /// Pushes a copy of the status flags on to the stack.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#PHP
    PHP,
    /// Pull Accumulator
    ///
    /// Pulls an 8 bit value from the stack and into the accumulator.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#PLA
    PLA,
    /// Pull Processor Status
    ///
    /// Pulls an 8 bit value from the stack and into the processor flags.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#PLP
    PLP,
    /// Rotate Left
    ///
    /// Move each of the bits one place to the left.
    /// Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#ROL
    ROL,
    /// Rotate Right
    ///
    /// Move each of the bits one place to the left.
    /// Bit 7 is filled with the current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#ROR
    ROR,
    /// Substract with Carry
    ///
    /// A,Z,C,N = A-M-(1-C)
    ///
    /// This instruction subtracts the contents of a memory location to the accumulator together
    /// with the not of the carry bit.
    /// If overflow occurs the carry bit is clear, this enables multiple byte subtraction to be performed.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#SBC
    SBC,
    /// Set Carry flag
    ///
    /// C = 1
    ///
    /// Set the carry flag to one
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#SEC
    SEC,
    /// Set Interrupt Disable
    ///
    /// I = 1
    ///
    /// Set the interrupt disable flag to one.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#SEI
    SEI,
    /// Store Accumulator
    ///
    /// M = A
    ///
    /// Stores the contents of the accumulator into memory.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#STA
    STA,
    /// Store X register
    ///
    /// M = X
    ///
    /// Stores the contents of the X register into memory.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#STX
    STX,
    /// Store Y register
    ///
    /// M = Y
    ///
    /// Stores the contents of the Y register into memory.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#STY
    STY,
    /// Transfer Accumulator to X
    ///
    /// X,Z,N = A
    ///
    /// Copies the current contents of the accumulator into the X register.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
    TAX,
    /// Transfer Accumulator to Y
    ///
    /// Y,Z,N = A
    ///
    /// Copies the current contents of the accumulator into the Y register.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#TAY
    TAY,
    /// Transfer X to Accumulator
    ///
    /// A,Z,N = X
    ///
    /// Copies the current contents of the X register into the accumulator.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#TXA
    TXA,
    /// Transfer Y to Accumulator
    ///
    /// A,Z,N = Y
    ///
    /// Copies the current contents of the Y register into the accumulator.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#TXY
    TYA,
}
