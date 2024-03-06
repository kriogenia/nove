#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
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
    /// Branch if Positive
    ///
    /// If the negative flag is clear then add the relative displacement to the program counter
    /// to cause a branch to a new location.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BPL
    BPL,
    /// Branch if Overflow Clear
    ///
    /// If the overflow flag is clear then add the relative displacement to the program counter
    /// to cause a branch to a new location.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BVC
    BVC,
    /// Branch if Overflow Set
    ///
    /// If the overflow flag is set then add the relative displacement to the program counter
    /// to cause a branch to a new location.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#BVS
    BVS,
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
    /// Clear Decimal flag
    ///
    /// D = 0
    ///
    /// Set the decimal flag to zero.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CLD
    CLD,
    /// Clear Interrupt flag
    ///
    /// I = 0
    ///
    /// Set the interrupt flag to zero.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#CLI
    CLI,
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
    /// Decrement memory
    ///
    /// M,Z,N = M-1
    ///
    /// Subtracts one to the value held at a specified memory location.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#DEC
    DEC,
    /// Decrement X register
    ///
    /// X,Z,N = X-1
    ///
    /// Subtracts one from the X register.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#DEX
    DEX,
    /// Decrement Y register
    ///
    /// Y,Z,N = Y-1
    ///
    /// Subtracts one from the Y register.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#DEY
    DEY,
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
    /// Adds one to the value held at a specified memory location.
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
    /// Increment Y register
    ///
    /// Y,Z,N = Y+1
    ///
    /// Adds one to the Y register.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#INY
    INY,
    /// Jump
    ///
    /// Sets the program counter to the address specified by the operand.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
    JMP,
    /// Jump to Subroutine
    ///
    /// The JSR instruction pushes the address (minus one) of the return point on to the stack
    /// and then sets the program counter to the target memory address.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#JSR
    ///
    JSR,
    /// Load Accumulator and X register
    ///
    /// A,X,Z,N = M
    ///
    /// Loads a byte of memory into the accumulator and the X register.
    ///
    /// _Unofficial_
    LAX,
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
    /// Logical Shift Right
    ///
    /// A,C,Z,N = A/2 or M,C,Z,N = M/2
    ///
    /// Each of the bits in A or M is shift one place to the right.
    /// The bit that was in bit 0 is shifted into the carry flag. Bit 7 is set to zero
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#LSR
    LSR,
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
    /// Return from Interrupt
    ///
    /// The RTI instruction is used at the end of an interrupt processing routine.
    /// It pulls the processor flags from the stack followed by the program counter.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#RTI
    RTI,
    /// Return from Subroutine
    ///
    /// The RTS instruction is used at the end of a subroutine to return to the calling routine.
    /// It pulls the program counter (minus one) from the stack.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#RTS
    RTS,
    /// AND Accumulator and X register
    ///
    /// M,Z,N = A & M
    ///
    /// AND X register with accumulator and store result in memory.
    ///
    /// _Unofficial_
    SAX,
    /// Subtract with Carry
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
    /// Set Decimal flag
    ///
    /// D = 1
    ///
    /// Set the decimal flag to one
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#SED
    SED,
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
    /// Transfer Stack Pointer to X
    ///
    /// X = S
    ///
    /// Copies the current contents of the stack register into the X register.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#TSX
    TSX,
    /// Transfer X to Accumulator
    ///
    /// A,Z,N = X
    ///
    /// Copies the current contents of the X register into the accumulator.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#TXA
    TXA,
    /// Transfer X to Stack Pointer
    ///
    /// S = X
    ///
    /// Copies the current contents of the X register into the stack register.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#TSX
    TXS,
    /// Transfer Y to Accumulator
    ///
    /// A,Z,N = Y
    ///
    /// Copies the current contents of the Y register into the accumulator.
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html#TXY
    TYA,
}
