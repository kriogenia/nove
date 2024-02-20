# Nove

**Nove** is a NES emulator started as a project on the **Open Source Club** at [Chaitin School](https://chaitinschool.org/). 
The core will be developed with Rust and the front-end will be compiled to WebAssembly (hopefully).

## Instructions

The vast majority of the work on this project is the implementation of all the NMOS 6502 Opcodes, the following table
details what the current state:

| ~~ADC~~ | ~~AND~~ | ~~ASL~~ | ~~BCC~~ |
|---------|---------|---------|---------|
| ~~BCS~~ | ~~BEQ~~ | ~~BIT~~ | ~~BMI~~ |
| ~~BNE~~ | ~~BPL~~ | ~~BRK~~ | ~~BVC~~ |
| ~~BVS~~ | ~~CLC~~ | ~~CLD~~ | ~~CLI~~ |
| ~~CLV~~ | ~~CMP~~ | ~~CPX~~ | ~~CPY~~ |
| ~~DEC~~ | ~~DEX~~ | DEY     | EOR     |
| ~~INC~~ | ~~INX~~ | INY     | ~~JMP~~ |
| JSR     | ~~LDA~~ | ~~LDX~~ | ~~LDY~~ |
| LSR     | ~~NOP~~ | ~~ORA~~ | ~~PHA~~ |
| ~~PHP~~ | ~~PLA~~ | ~~PLP~~ | ~~ROL~~ |
| ~~ROR~~ | RTI     | RTS     | ~~SBC~~ |
| ~~SEC~~ | SED     | ~~SEI~~ | ~~STA~~ |
| ~~STX~~ | ~~STY~~ | ~~TAX~~ | ~~TAY~~ |
| TSX     | ~~TXA~~ | TXS     | ~~TYA~~ |
