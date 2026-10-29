# 6502-rust-emulator

A 6502 CPU emulator written in Rust to learn about emulators and systems programming.

## 6502 Instruction Set Status

| Category | Instruction | Modes | Status |
|----------|-------------|-------|--------|
| **Load/Store** | LDA | IMM, ZP, ZP,X, ABS, ABS,X, ABS,Y, IND,X, IND,Y | ✅ Tested |
| | LDX | IMM, ZP, ZP,Y, ABS, ABS,Y | ✅ Tested |
| | LDY | IMM, ZP, ZP,X, ABS, ABS,X | ✅ Tested |
| | STA | ZP, ZP,X, ABS, ABS,X, ABS,Y, IND,X, IND,Y | ✅ Tested |
| | STX | ZP, ZP,Y, ABS | ✅ Tested |
| | STY | ZP, ZP,X, ABS | ✅ Tested |
| **Register Transfers** | TAX, TAY, TXA, TYA | Implied | ✅ Tested |
| | TSX, TXS | Implied | ✅ Tested |
| **Stack** | PHA, PHP | Implied | ✅ Tested |
| | PLA, PLP | Implied | ✅ Tested |
| **Logic** | AND | IMM, ZP, ZP,X, ABS, ABS,X, ABS,Y, IND,X, IND,Y | ✅ Tested |
| | EOR (XOR) | IMM, ZP, ZP,X, ABS, ABS,X, ABS,Y, IND,X, IND,Y | ✅ Tested |
| | ORA | IMM, ZP, ZP,X, ABS, ABS,X, ABS,Y, IND,X, IND,Y | ✅ Tested |
| | BIT | ZP, ABS | ✅ Tested |
| **Arithmetic** | ADC | IMM, ZP, ZP,X, ABS, ABS,X, ABS,Y, IND,X, IND,Y | ✅ Tested |
| | SBC | IMM, ZP, ZP,X, ABS, ABS,X, ABS,Y, IND,X, IND,Y | ✅ Tested |
| **Compare** | CMP | IMM, ZP, ZP,X, ABS, ABS,X, ABS,Y, IND,X, IND,Y | ✅ Tested |
| | CPX | IMM, ZP, ABS | ✅ Tested |
| | CPY | IMM, ZP, ABS | ✅ Tested |
| **Increment/Decrement** | INC | ZP, ZP,X, ABS, ABS,X | ✅ Tested |
| | INX, INY | Implied | ✅ Tested |
| | DEC | ZP, ZP,X, ABS, ABS,X | ✅ Tested |
| | DEX, DEY | Implied | ✅ Tested |
| **Shift/Rotate** | ASL | A, ZP, ZP,X, ABS, ABS,X | ✅ Tested |
| | LSR | A, ZP, ZP,X, ABS, ABS,X | ✅ Tested |
| | ROL | A, ZP, ZP,X, ABS, ABS,X | ✅ Tested |
| | ROR | A, ZP, ZP,X, ABS, ABS,X | ✅ Tested |
| **Branch** | BCC, BCS, BEQ, BMI | Relative | ✅ Tested |
| | BNE, BPL, BVC, BVS | Relative | ✅ Tested |
| **Flag** | CLC, CLD, CLI, CLV | Implied | ✅ Tested |
| | SEC, SED, SEI | Implied | ✅ Tested |
| **System** | NOP | Implied | ✅ Tested |
| | BRK, RTI | Implied | ✅ Tested |
| **Control Flow** | JMP | ABS, IND | ✅ Tested |
| | JSR, RTS | ABS, Implied | ✅ Tested |

**Legend**: IMM=Immediate, ZP=Zero Page, ABS=Absolute, IND=Indirect, A=Accumulator, Relative=Relative Addressing
