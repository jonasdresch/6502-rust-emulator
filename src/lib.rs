// http://www.obelisk.me.uk/6502/architecture.html
// http://www.emulator101.com/6502-addressing-modes.html
// http://nesdev.com/6502_cpu.txt
// https://slark.me/c64-downloads/6502-addressing-modes.pdf
// https://sites.google.com/site/6502asembly/6502-instruction-set/plp
// https://www.masswerk.at/6502/6502_instruction_set.html
// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html

struct OpImm {
    reg_index: usize,
}
struct OpRead {
    reg_index: usize,
}
struct OpReadAnd {
    reg_index: usize,
}
struct OpReadEor {
    reg_index: usize,
}
struct OpReadOra {
    reg_index: usize,
}
struct OpWrite {
    reg_index: usize,
}
struct OpInv;

trait Operation: Send + Sync {
    fn execute(&self, cpu: &mut CPU, addr: u16);
}

impl Operation for OpImm {
    fn execute(&self, cpu: &mut CPU, addr: u16) {
        cpu.regs[self.reg_index] = addr as u8;
        cpu.set_load_instructions_flags(self.reg_index);
    }
}

impl Operation for OpRead {
    fn execute(&self, cpu: &mut CPU, addr: u16) {
        cpu.regs[self.reg_index] = cpu.read8(addr);
        cpu.set_load_instructions_flags(self.reg_index);
    }
}

impl Operation for OpReadAnd {
    fn execute(&self, cpu: &mut CPU, addr: u16) {
        cpu.regs[self.reg_index] &= cpu.read8(addr);
        cpu.set_load_instructions_flags(self.reg_index);
    }
}

impl Operation for OpReadEor {
    fn execute(&self, cpu: &mut CPU, addr: u16) {
        cpu.regs[self.reg_index] ^= cpu.read8(addr);
        cpu.set_load_instructions_flags(self.reg_index);
    }
}

impl Operation for OpReadOra {
    fn execute(&self, cpu: &mut CPU, addr: u16) {
        cpu.regs[self.reg_index] |= cpu.read8(addr);
        cpu.set_load_instructions_flags(self.reg_index);
    }
}

impl Operation for OpWrite {
    fn execute(&self, cpu: &mut CPU, addr: u16) {
        cpu.write8(addr, cpu.regs[self.reg_index]);
    }
}

impl Operation for OpInv {
    fn execute(&self, _cpu: &mut CPU, _addr: u16) {
        println!("Invalid OP");
    }
}

struct AddrModeImm;
struct AddrModeZero;
struct AddrModeZeroX;
struct AddrModeAbs;
struct AddrModeAbsX {
    is_read_op: bool,
}
struct AddrModeAbsY {
    is_read_op: bool,
}
struct AddrModeIndX;
struct AddrModeIndY {
    is_read_op: bool,
}
struct AddrModeInv;

trait AddrMode: Send + Sync {
    fn process(&self, cpu: &mut CPU) -> u16;
}

impl AddrMode for AddrModeImm {
    fn process(&self, cpu: &mut CPU) -> u16 {
        cpu.read_pc() as u16
    }
}

impl AddrMode for AddrModeZero {
    fn process(&self, cpu: &mut CPU) -> u16 {
        cpu.read_pc() as u16
    }
}

impl AddrMode for AddrModeZeroX {
    fn process(&self, cpu: &mut CPU) -> u16 {
        cpu.fetch_zero_page_addr(cpu.regs[CPU::REG_X])
    }
}

impl AddrMode for AddrModeAbs {
    fn process(&self, cpu: &mut CPU) -> u16 {
        cpu.fetch_absolute_addr()
    }
}

impl AddrMode for AddrModeAbsX {
    fn process(&self, cpu: &mut CPU) -> u16 {
        cpu.fetch_absolute_indexed_addr(cpu.regs[CPU::REG_X], self.is_read_op)
    }
}

impl AddrMode for AddrModeAbsY {
    fn process(&self, cpu: &mut CPU) -> u16 {
        cpu.fetch_absolute_indexed_addr(cpu.regs[CPU::REG_Y], self.is_read_op)
    }
}

impl AddrMode for AddrModeIndX {
    fn process(&self, cpu: &mut CPU) -> u16 {
        cpu.fetch_indirect_x_addr()
    }
}

impl AddrMode for AddrModeIndY {
    fn process(&self, cpu: &mut CPU) -> u16 {
        cpu.fetch_indirect_y_addr(self.is_read_op)
    }
}

impl AddrMode for AddrModeInv {
    fn process(&self, _cpu: &mut CPU) -> u16 {
        0
    }
}

// Hash is very slow compared to match (3x, 4x slower)
// static _INST_TABLE: phf::Map<u8, (&dyn AddrMode, &dyn Operation)> = phf::phf_map! {
//     0xA9u8 => (&AddrModeImm, &OpImm {reg_index: CPU::REG_A}),
//     0xA5u8 => (&AddrModeZero, &OpRead {reg_index: CPU::REG_A}),
//     0xB5u8 => (&AddrModeZeroX, &OpRead {reg_index: CPU::REG_A}),
//     0xADu8 => (&AddrModeAbs, &OpRead {reg_index: CPU::REG_A}),
//     0xBDu8 => (&AddrModeAbsX {is_read_op: true}, &OpRead {reg_index: CPU::REG_A}),
//     0xB9u8 => (&AddrModeAbsY {is_read_op: true}, &OpRead {reg_index: CPU::REG_A}),
//     0xA1u8 => (&AddrModeIndX, &OpRead {reg_index: CPU::REG_A}),
//     0xB1u8 => (&AddrModeIndY {is_read_op: true}, &OpRead {reg_index: CPU::REG_A}),
//     0xA2u8 => (&AddrModeImm, &OpImm {reg_index: CPU::REG_X}),
//     0xBCu8 => (&AddrModeAbsX {is_read_op: true}, &OpRead {reg_index: CPU::REG_Y}),
// };

// This is still slower than match, but pretty close
static INST_VEC: [(&dyn AddrMode, &dyn Operation); 7] = [
    (&AddrModeInv, &OpInv),                                //BRK
    (&AddrModeIndX, &OpReadOra { reg_index: CPU::REG_A }), // ORA
    (&AddrModeInv, &OpInv),
    (&AddrModeInv, &OpInv),
    (&AddrModeInv, &OpInv),
    (&AddrModeZero, &OpReadOra { reg_index: CPU::REG_A }),
    (&AddrModeImm, &OpImm { reg_index: CPU::REG_A }),
];

// lifetime anotation <'b>
pub struct CPU<'a> {
    pub pc: u16,
    pub regs: [u8; 5],
    pub cycles_run: u32,
    mem: &'a mut MEM,
}

impl<'a> CPU<'a> {
    // Instructions
    // Load Operations
    pub const LDA_IMMEDIATE: u8 = 0xA9;
    pub const LDA_ZERO: u8 = 0xA5;
    pub const LDA_ZERO_X: u8 = 0xB5;
    pub const LDA_ABSOLUTE: u8 = 0xAD;
    pub const LDA_ABSOLUTE_X: u8 = 0xBD;
    pub const LDA_ABSOLUTE_Y: u8 = 0xB9;
    pub const LDA_INDIRECT_X: u8 = 0xA1;
    pub const LDA_INDIRECT_Y: u8 = 0xB1;
    pub const LDX_IMMEDIATE: u8 = 0xA2;
    pub const LDX_ZERO: u8 = 0xA6;
    pub const LDX_ZERO_Y: u8 = 0xB6;
    pub const LDX_ABSOLUTE: u8 = 0xAE;
    pub const LDX_ABSOLUTE_Y: u8 = 0xBE;
    pub const LDY_IMMEDIATE: u8 = 0xA0;
    pub const LDY_ZERO: u8 = 0xA4;
    pub const LDY_ZERO_X: u8 = 0xB4;
    pub const LDY_ABSOLUTE: u8 = 0xAC;
    pub const LDY_ABSOLUTE_X: u8 = 0xBC;
    // Store operations
    pub const STA_ZERO: u8 = 0x85;
    pub const STA_ZERO_X: u8 = 0x95;
    pub const STA_ABSOLUTE: u8 = 0x8D;
    pub const STA_ABSOLUTE_X: u8 = 0x9D;
    pub const STA_ABSOLUTE_Y: u8 = 0x99;
    pub const STA_INDIRECT_X: u8 = 0x81;
    pub const STA_INDIRECT_Y: u8 = 0x91;
    pub const STX_ZERO: u8 = 0x86;
    pub const STX_ZERO_Y: u8 = 0x96;
    pub const STX_ABSOLUTE: u8 = 0x8E;
    pub const STY_ZERO: u8 = 0x84;
    pub const STY_ZERO_X: u8 = 0x94;
    pub const STY_ABSOLUTE: u8 = 0x8C;
    // Register Transfers
    pub const TRANS_A_TO_X: u8 = 0xAA;
    pub const TRANS_A_TO_Y: u8 = 0xA8;
    pub const TRANS_X_TO_A: u8 = 0x8A;
    pub const TRANS_Y_TO_A: u8 = 0x98;
    // Stack Operations
    pub const TRANS_SP_TO_X: u8 = 0xBA;
    pub const TRANS_X_TO_SP: u8 = 0x9A;
    pub const PUSH_A_TO_SP: u8 = 0x48;
    pub const PUSH_STAT_TO_SP: u8 = 0x08;
    pub const PULL_SP_TO_A: u8 = 0x68;
    pub const PULL_SP_TO_STAT: u8 = 0x28;
    // Logic Operations
    pub const AND_IMMEDIATE: u8 = 0x29;
    pub const AND_ZERO: u8 = 0x25;
    pub const AND_ZERO_X: u8 = 0x35;
    pub const AND_ABSOLUTE: u8 = 0x2D;
    pub const AND_ABSOLUTE_X: u8 = 0x3D;
    pub const AND_ABSOLUTE_Y: u8 = 0x39;
    pub const AND_INDIRECT_X: u8 = 0x21;
    pub const AND_INDIRECT_Y: u8 = 0x31;
    pub const EOR_IMMEDIATE: u8 = 0x49;
    pub const EOR_ZERO: u8 = 0x45;
    pub const EOR_ZERO_X: u8 = 0x55;
    pub const EOR_ABSOLUTE: u8 = 0x4D;
    pub const EOR_ABSOLUTE_X: u8 = 0x5D;
    pub const EOR_ABSOLUTE_Y: u8 = 0x59;
    pub const EOR_INDIRECT_X: u8 = 0x41;
    pub const EOR_INDIRECT_Y: u8 = 0x51;
    pub const ORA_IMMEDIATE: u8 = 0x09;
    pub const ORA_ZERO: u8 = 0x05;
    pub const ORA_ZERO_X: u8 = 0x15;
    pub const ORA_ABSOLUTE: u8 = 0x0D;
    pub const ORA_ABSOLUTE_X: u8 = 0x1D;
    pub const ORA_ABSOLUTE_Y: u8 = 0x19;
    pub const ORA_INDIRECT_X: u8 = 0x01;
    pub const ORA_INDIRECT_Y: u8 = 0x11;
    // bit test
    pub const BIT_TEST_ZERO: u8 = 0x24;
    pub const BIT_TEST_ABSOLUTE: u8 = 0x2C;
    // adc
    pub const ADC_IMMEDIATE: u8 = 0x69;
    pub const ADC_ZERO: u8 = 0x65;
    pub const ADC_ZERO_X: u8 = 0x75;
    pub const ADC_ABSOLUTE: u8 = 0x6D;
    pub const ADC_ABSOLUTE_X: u8 = 0x7D;
    pub const ADC_ABSOLUTE_Y: u8 = 0x79;
    pub const ADC_INDIRECT_X: u8 = 0x61;
    pub const ADC_INDIRECT_Y: u8 = 0x71;
    // sbc
    pub const SBC_IMMEDIATE: u8 = 0xE9;
    pub const SBC_ZERO: u8 = 0xE5;
    pub const SBC_ZERO_X: u8 = 0xF5;
    pub const SBC_ABSOLUTE: u8 = 0xED;
    pub const SBC_ABSOLUTE_X: u8 = 0xFD;
    pub const SBC_ABSOLUTE_Y: u8 = 0xF9;
    pub const SBC_INDIRECT_X: u8 = 0xE1;
    pub const SBC_INDIRECT_Y: u8 = 0xF1;
    // cmp
    pub const CMP_IMMEDIATE: u8 = 0xC9;
    pub const CMP_ZERO: u8 = 0xC5;
    pub const CMP_ZERO_X: u8 = 0xD5;
    pub const CMP_ABSOLUTE: u8 = 0xCD;
    pub const CMP_ABSOLUTE_X: u8 = 0xDD;
    pub const CMP_ABSOLUTE_Y: u8 = 0xD9;
    pub const CMP_INDIRECT_X: u8 = 0xC1;
    pub const CMP_INDIRECT_Y: u8 = 0xD1;

    // status flags
    pub const FLAG_CARRY: u8 = 0b0000_0001;
    pub const FLAG_ZERO: u8 = 0b0000_0010;
    pub const FLAG_INTERRUPT: u8 = 0b0000_0100;
    pub const FLAG_DECIMAL: u8 = 0b0000_1000;
    pub const FLAG_BREAK: u8 = 0b0001_0000;
    pub const FLAG_OVERFLOW: u8 = 0b0100_0000;
    pub const FLAG_NEGATIVE: u8 = 0b1000_0000;

    // regs
    pub const REG_SP: usize = 0;
    pub const REG_A: usize = 1;
    pub const REG_X: usize = 2;
    pub const REG_Y: usize = 3;
    pub const REG_STAT: usize = 4;

    pub fn new(mem: &'a mut MEM) -> Self {
        CPU { pc: 0, regs: [0; 5], cycles_run: 0, mem }
    }
    pub fn reset(&mut self) {
        self.pc = (self.mem.read8(RESET_VECTOR_ADDR) as u16) << 8
            | self.mem.read8(RESET_VECTOR_ADDR + 1) as u16;
        self.regs = [0; 5];
        self.regs[CPU::REG_SP] = STACK_OFFSET_START;
        self.cycles_run = 0;
    }

    // The methods below cost some cycles to run.
    // Try to use them when processing instructions instead of incrementing the cycles counter on each instruction
    fn read8(&mut self, addr: u16) -> u8 {
        let val = self.mem.read8(addr as usize);
        self.cycles_run += 1;
        val
    }

    fn read16(&mut self, addr: u16) -> u16 {
        let low = self.read8(addr);
        let high = self.read8(addr + 1);
        (high as u16) << 8 | low as u16
    }

    fn write8(&mut self, addr: u16, val: u8) {
        self.mem.write8(addr as usize, val);
        self.cycles_run += 1;
    }

    fn write_to_stack(&mut self, val: u8) {
        self.write8(self.regs[CPU::REG_SP] as u16 + STACK_START_ADDR, val);
        self.regs[CPU::REG_SP] = self.sub(self.regs[CPU::REG_SP], 1);
    }

    fn read_from_stack(&mut self) -> u8 {
        self.regs[CPU::REG_SP] = self.sum(self.regs[CPU::REG_SP], 1);
        self.cycles_run += 1;
        self.read8(self.regs[CPU::REG_SP] as u16 + STACK_START_ADDR)
    }

    fn read_pc(&mut self) -> u8 {
        let val = self.read8(self.pc);
        self.pc += 1;
        val
    }

    fn sum(&mut self, val1: u8, val2: u8) -> u8 {
        self.cycles_run += 1;
        let sum = val1 as u16 + val2 as u16;
        sum as u8
    }

    fn sub(&mut self, val1: u8, val2: u8) -> u8 {
        self.cycles_run += 1;
        val1 - val2
    }

    fn adc(&mut self, val: u8) -> u8 {
        let carry = self.regs[CPU::REG_STAT] & CPU::FLAG_CARRY;
        let sum = self.regs[CPU::REG_A] as u16 + val as u16 + carry as u16;
        if sum > 255 {
            self.regs[CPU::REG_STAT] |= CPU::FLAG_CARRY;
        } else {
            self.regs[CPU::REG_STAT] &= !CPU::FLAG_CARRY;
        }
        let of = (self.regs[CPU::REG_A] ^ sum as u8) & (val ^ sum as u8) & 0x80;
        if of > 0 {
            self.regs[CPU::REG_STAT] |= CPU::FLAG_OVERFLOW;
        } else {
            self.regs[CPU::REG_STAT] &= !CPU::FLAG_OVERFLOW;
        }
        sum as u8
    }

    fn set_load_instructions_flags(&mut self, reg: usize) {
        if self.regs[reg] == 0 {
            self.regs[CPU::REG_STAT] |= CPU::FLAG_ZERO;
        } else {
            self.regs[CPU::REG_STAT] &= !CPU::FLAG_ZERO;
        }
        if (self.regs[reg] & CPU::FLAG_NEGATIVE) > 0 {
            self.regs[CPU::REG_STAT] |= CPU::FLAG_NEGATIVE;
        } else {
            self.regs[CPU::REG_STAT] &= !CPU::FLAG_NEGATIVE;
        }
    }

    fn set_compare_flags(&mut self, reg: usize, val: u8) {
        self.regs[CPU::REG_STAT] &= !CPU::FLAG_CARRY;
        self.regs[CPU::REG_STAT] &= !CPU::FLAG_ZERO;
        self.regs[CPU::REG_STAT] &= !CPU::FLAG_NEGATIVE;
        if self.regs[reg] >= val {
            self.regs[CPU::REG_STAT] |= CPU::FLAG_CARRY;
        }
        if self.regs[reg] == val {
            self.regs[CPU::REG_STAT] |= CPU::FLAG_ZERO;
        }
        self.regs[CPU::REG_STAT] |= (self.regs[reg] - val) & CPU::FLAG_NEGATIVE;
    }

    // Methods for the addressing modes
    fn fetch_zero_page_addr(&mut self, reg: u8) -> u16 {
        let val = self.read_pc();
        self.sum(val, reg) as u16
    }

    fn fetch_absolute_addr(&mut self) -> u16 {
        let low = self.read_pc() as u16;
        let high = self.read_pc() as u16;
        high << 8 | low
    }

    // https://retrocomputing.stackexchange.com/questions/15621/why-dont-all-absolute-x-instructions-take-an-extra-cycle-to-cross-page-boundari
    fn fetch_absolute_indexed_addr(&mut self, reg: u8, read_from_addr: bool) -> u16 {
        let low = self.read_pc();
        let high = self.read_pc();
        let mut addr = (reg as u16) + (low as u16);
        if addr > 255 || !read_from_addr {
            // penalty
            self.read8(addr);
        }
        addr += (high as u16) << 8;
        addr
    }

    fn fetch_indirect_x_addr(&mut self) -> u16 {
        let mut ind_addr = self.read_pc();
        ind_addr = self.sum(ind_addr, self.regs[CPU::REG_X]);
        self.read16(ind_addr as u16)
    }

    fn fetch_indirect_y_addr(&mut self, read_from_addr: bool) -> u16 {
        let ind_addr = self.read_pc();
        let low = self.read8(ind_addr as u16);
        let high = self.read8((ind_addr + 1) as u16);
        let mut addr = (self.regs[CPU::REG_Y] as u16) + (low as u16);
        if addr > 255 || !read_from_addr {
            // penalty
            self.read8(addr);
        }
        addr += (high as u16) << 8;
        addr
    }

    pub fn process2(&mut self, cycles: u32) {
        let init_cycles = self.cycles_run;
        loop {
            let instruction = self.read_pc();
            let (inst, op) = INST_VEC[instruction as usize];
            let addr2 = inst.process(self);
            op.execute(self, addr2);
            if self.cycles_run - init_cycles >= cycles {
                break;
            }
        }
    }

    pub fn process(&mut self, cycles: u32) {
        let init_cycles = self.cycles_run;
        loop {
            let instruction = self.read_pc();
            match instruction {
                CPU::LDA_IMMEDIATE => {
                    self.regs[CPU::REG_A] = self.read_pc();
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::LDA_ZERO => {
                    let addr = self.read_pc();
                    self.regs[CPU::REG_A] = self.read8(addr as u16);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::LDA_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_X]);
                    self.regs[CPU::REG_A] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::LDA_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.regs[CPU::REG_A] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::LDA_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_X], true);
                    self.regs[CPU::REG_A] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::LDA_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_Y], true);
                    self.regs[CPU::REG_A] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::LDA_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    self.regs[CPU::REG_A] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::LDA_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(true);
                    self.regs[CPU::REG_A] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::LDX_IMMEDIATE => {
                    self.regs[CPU::REG_X] = self.read_pc();
                    self.set_load_instructions_flags(CPU::REG_X);
                }
                CPU::LDX_ZERO => {
                    let addr = self.read_pc();
                    self.regs[CPU::REG_X] = self.read8(addr as u16);
                    self.set_load_instructions_flags(CPU::REG_X);
                }
                CPU::LDX_ZERO_Y => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_Y]);
                    self.regs[CPU::REG_X] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_X);
                }
                CPU::LDX_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.regs[CPU::REG_X] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_X);
                }
                CPU::LDX_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_Y], true);
                    self.regs[CPU::REG_X] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_X);
                }
                CPU::LDY_IMMEDIATE => {
                    self.regs[CPU::REG_Y] = self.read_pc();
                    self.set_load_instructions_flags(CPU::REG_Y);
                }
                CPU::LDY_ZERO => {
                    let addr = self.read_pc();
                    self.regs[CPU::REG_Y] = self.read8(addr as u16);
                    self.set_load_instructions_flags(CPU::REG_Y);
                }
                CPU::LDY_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_X]);
                    self.regs[CPU::REG_Y] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_Y);
                }
                CPU::LDY_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.regs[CPU::REG_Y] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_Y);
                }
                CPU::LDY_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_X], true);
                    self.regs[CPU::REG_Y] = self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_Y);
                }
                CPU::STA_ZERO => {
                    let addr = self.read_pc();
                    self.write8(addr as u16, self.regs[CPU::REG_A]);
                }
                CPU::STA_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_X]);
                    self.write8(addr, self.regs[CPU::REG_A]);
                }
                CPU::STA_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.write8(addr, self.regs[CPU::REG_A]);
                }
                CPU::STA_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_X], false);
                    self.write8(addr, self.regs[CPU::REG_A]);
                }
                CPU::STA_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_Y], false);
                    self.write8(addr, self.regs[CPU::REG_A]);
                }
                CPU::STA_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    self.write8(addr, self.regs[CPU::REG_A]);
                }
                CPU::STA_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(false);
                    self.write8(addr, self.regs[CPU::REG_A]);
                }
                CPU::STX_ZERO => {
                    let addr = self.read_pc();
                    self.write8(addr as u16, self.regs[CPU::REG_X]);
                }
                CPU::STX_ZERO_Y => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_Y]);
                    self.write8(addr, self.regs[CPU::REG_X]);
                }
                CPU::STX_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.write8(addr, self.regs[CPU::REG_X]);
                }
                CPU::STY_ZERO => {
                    let addr = self.read_pc();
                    self.write8(addr as u16, self.regs[CPU::REG_Y]);
                }
                CPU::STY_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_X]);
                    self.write8(addr, self.regs[CPU::REG_Y]);
                }
                CPU::STY_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.write8(addr, self.regs[CPU::REG_Y]);
                }
                CPU::TRANS_A_TO_X => {
                    self.regs[CPU::REG_X] = self.regs[CPU::REG_A];
                    self.cycles_run += 1;
                    self.set_load_instructions_flags(CPU::REG_X);
                }
                CPU::TRANS_A_TO_Y => {
                    self.regs[CPU::REG_Y] = self.regs[CPU::REG_A];
                    self.cycles_run += 1;
                    self.set_load_instructions_flags(CPU::REG_Y);
                }
                CPU::TRANS_X_TO_A => {
                    self.regs[CPU::REG_A] = self.regs[CPU::REG_X];
                    self.cycles_run += 1;
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::TRANS_Y_TO_A => {
                    self.regs[CPU::REG_A] = self.regs[CPU::REG_Y];
                    self.cycles_run += 1;
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::TRANS_SP_TO_X => {
                    self.regs[CPU::REG_X] = self.regs[CPU::REG_SP];
                    self.cycles_run += 1;
                    self.set_load_instructions_flags(CPU::REG_X);
                }
                CPU::TRANS_X_TO_SP => {
                    self.regs[CPU::REG_SP] = self.regs[CPU::REG_X];
                    self.cycles_run += 1;
                }
                CPU::PUSH_A_TO_SP => {
                    self.write_to_stack(self.regs[CPU::REG_A]);
                }
                CPU::PUSH_STAT_TO_SP => {
                    self.write_to_stack(self.regs[CPU::REG_STAT]);
                }
                CPU::PULL_SP_TO_A => {
                    self.regs[CPU::REG_A] = self.read_from_stack();
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::PULL_SP_TO_STAT => {
                    self.regs[CPU::REG_STAT] = self.read_from_stack();
                }
                CPU::AND_IMMEDIATE => {
                    self.regs[CPU::REG_A] &= self.read_pc();
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::AND_ZERO => {
                    let addr = self.read_pc();
                    self.regs[CPU::REG_A] &= self.read8(addr as u16);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::AND_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_X]);
                    self.regs[CPU::REG_A] &= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::AND_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.regs[CPU::REG_A] &= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::AND_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_X], true);
                    self.regs[CPU::REG_A] &= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::AND_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_Y], true);
                    self.regs[CPU::REG_A] &= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::AND_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    self.regs[CPU::REG_A] &= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::AND_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(true);
                    self.regs[CPU::REG_A] &= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }

                CPU::EOR_IMMEDIATE => {
                    self.regs[CPU::REG_A] ^= self.read_pc();
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::EOR_ZERO => {
                    let addr = self.read_pc();
                    self.regs[CPU::REG_A] ^= self.read8(addr as u16);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::EOR_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_X]);
                    self.regs[CPU::REG_A] ^= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::EOR_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.regs[CPU::REG_A] ^= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::EOR_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_X], true);
                    self.regs[CPU::REG_A] ^= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::EOR_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_Y], true);
                    self.regs[CPU::REG_A] ^= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::EOR_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    self.regs[CPU::REG_A] ^= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::EOR_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(true);
                    self.regs[CPU::REG_A] ^= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ORA_IMMEDIATE => {
                    self.regs[CPU::REG_A] |= self.read_pc();
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ORA_ZERO => {
                    let addr = self.read_pc();
                    self.regs[CPU::REG_A] |= self.read8(addr as u16);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ORA_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_X]);
                    self.regs[CPU::REG_A] |= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ORA_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.regs[CPU::REG_A] |= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ORA_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_X], true);
                    self.regs[CPU::REG_A] |= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ORA_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_Y], true);
                    self.regs[CPU::REG_A] |= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ORA_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    self.regs[CPU::REG_A] |= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ORA_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(true);
                    self.regs[CPU::REG_A] |= self.read8(addr);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::BIT_TEST_ZERO => {
                    let addr = self.read_pc();
                    let val = self.read8(addr as u16);
                    let masked = self.regs[CPU::REG_A] & val;
                    if masked == 0 {
                        self.regs[CPU::REG_STAT] |= CPU::FLAG_ZERO;
                    } else {
                        self.regs[CPU::REG_STAT] &= !CPU::FLAG_ZERO;
                    }
                    self.regs[CPU::REG_STAT] &= !(CPU::FLAG_NEGATIVE | CPU::FLAG_OVERFLOW);
                    self.regs[CPU::REG_STAT] |= val & (CPU::FLAG_NEGATIVE | CPU::FLAG_OVERFLOW);
                }
                CPU::BIT_TEST_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    let val = self.read8(addr);
                    let masked = self.regs[CPU::REG_A] & val;
                    if masked == 0 {
                        self.regs[CPU::REG_STAT] |= CPU::FLAG_ZERO;
                    } else {
                        self.regs[CPU::REG_STAT] &= !CPU::FLAG_ZERO;
                    }
                    self.regs[CPU::REG_STAT] &= !(CPU::FLAG_NEGATIVE | CPU::FLAG_OVERFLOW);
                    self.regs[CPU::REG_STAT] |= val & (CPU::FLAG_NEGATIVE | CPU::FLAG_OVERFLOW);
                }
                CPU::ADC_IMMEDIATE => {
                    let val = self.read_pc();
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ADC_ZERO => {
                    let addr = self.read_pc();
                    let val = self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ADC_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_X]);
                    let val = self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ADC_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    let val = self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ADC_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_X], true);
                    let val = self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ADC_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_Y], true);
                    let val = self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ADC_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    let val = self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::ADC_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(true);
                    let val = self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::SBC_IMMEDIATE => {
                    let val = 255 - self.read_pc();
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::SBC_ZERO => {
                    let addr = self.read_pc();
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::SBC_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_X]);
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::SBC_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::SBC_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_X], true);
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::SBC_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_Y], true);
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::SBC_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::SBC_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(true);
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }

                CPU::CMP_IMMEDIATE => {
                    let val = 255 - self.read_pc();
                    self.set_compare_flags(CPU::REG_A, val);
                }
                CPU::CMP_ZERO => {
                    let addr = self.read_pc();
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::CMP_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.regs[CPU::REG_X]);
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::CMP_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::CMP_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_X], true);
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::CMP_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.regs[CPU::REG_Y], true);
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::CMP_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                CPU::CMP_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(true);
                    let val = 255 - self.read8(addr as u16);
                    self.regs[CPU::REG_A] = self.adc(val);
                    self.set_load_instructions_flags(CPU::REG_A);
                }
                _ => println!("Invalid OP"),
            }
            if self.cycles_run - init_cycles >= cycles {
                break;
            }
        }
    }
}

pub const MEM_SIZE: usize = 64 * 1024;
pub const RESET_VECTOR_ADDR: usize = 0xFFFC;
pub const RESET_EXEC_ADDRESS: u16 = 0xFCE2;
// Stack goes from 0x100 to 0x1FF. Empty stack points to 0x1FF and it grows downwards
pub const STACK_START_ADDR: u16 = 0x100;
pub const STACK_OFFSET_START: u8 = 0xFF;
// for reading directly fro mthe memmory on tests
pub const STACK_REAL_START: usize = STACK_START_ADDR as usize + STACK_OFFSET_START as usize;

pub struct MEM {
    mem: [u8; MEM_SIZE],
}

impl Default for MEM {
    fn default() -> Self {
        Self::new()
    }
}

impl MEM {
    pub fn new() -> MEM {
        MEM { mem: [0; MEM_SIZE] }
    }

    pub fn read8(&self, address: usize) -> u8 {
        assert!(address < MEM_SIZE, "memory access out ouf bounds");
        self.mem[address]
    }

    pub fn write8(&mut self, address: usize, value: u8) {
        assert!(address < MEM_SIZE, "memory access out ouf bounds");
        self.mem[address] = value;
    }

    pub fn write16(&mut self, address: usize, value: u16) {
        assert!(address < MEM_SIZE, "memory access out ouf bounds");
        self.write8(address, value as u8);
        self.write8(address + 1, ((value & 0xFF00) >> 8) as u8);
    }

    pub fn load_programm(&mut self, programm: &[u8]) {
        for (i, elem) in programm.iter().enumerate() {
            self.write8(RESET_EXEC_ADDRESS as usize + i, *elem);
        }
    }

    pub fn reset(&mut self) {
        self.mem = [0; MEM_SIZE];
        // Execution address of cold reset. based on C64 https://sta.c64.org/cbm64mem.html
        self.mem[RESET_VECTOR_ADDR] = ((RESET_EXEC_ADDRESS >> 8) & 0xFF) as u8;
        self.mem[RESET_VECTOR_ADDR + 1] = (RESET_EXEC_ADDRESS & 0x00FF) as u8;
    }
}
