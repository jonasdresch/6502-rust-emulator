// http://www.obelisk.me.uk/6502/architecture.html
// http://www.emulator101.com/6502-addressing-modes.html
// http://nesdev.com/6502_cpu.txt
// https://slark.me/c64-downloads/6502-addressing-modes.pdf
// https://sites.google.com/site/6502asembly/6502-instruction-set/plp

// lifetime anotation <'b>
pub struct CPU<'a> {
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub status: u8,
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

    // status flags
    pub const FLAG_CARRY: u8 = 0b0100_0000;
    pub const FLAG_ZERO: u8 = 0b0010_0000;
    pub const FLAG_INTERRUPT: u8 = 0b0001_0000;
    pub const FLAG_DECIMAL: u8 = 0b0000_1000;
    pub const FLAG_BREAK: u8 = 0b0000_0100;
    pub const FLAG_OVERFLOW: u8 = 0b0000_0010;
    pub const FLAG_NEGATIVE: u8 = 0b0000_0001;

    pub fn new(mem: &'a mut MEM) -> Self {
        CPU { pc: 0, sp: 0, a: 0, x: 0, y: 0, status: 0, cycles_run: 0, mem }
    }

    pub fn reset(&mut self) {
        self.pc = (self.mem.read8(RESET_VECTOR_ADDR) as u16) << 8
            | self.mem.read8(RESET_VECTOR_ADDR + 1) as u16;
        self.status = 0;
        self.cycles_run = 0;
        self.sp = STACK_OFFSET_START;
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
        self.write8(self.sp as u16 + STACK_START_ADDR, val);
        self.sp = self.sub(self.sp, 1);
    }

    fn read_from_stack(&mut self) -> u8 {
        self.sp = self.sum(self.sp, 1);
        self.cycles_run += 1;
        self.read8(self.sp as u16 + STACK_START_ADDR)
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

    /* fn sum(&mut self, val1: u8, val2: u8, simult: bool) -> u8{
        // some operations, like reads, could execute in parallel with sum
        // in this case we do not increment the cycles
        if !simult {
            self.cycles_run += 1;
        }
        let sum = val1 as u16 + val2 as u16;
        if sum > 255{
            self.status |= CPU::FLAG_CARRY;
        } else {
            self.status &= !CPU::FLAG_CARRY;
        }
        return sum as u8;
    } */

    fn set_load_instructions_flags(&mut self, reg: u8) {
        if reg == 0 {
            self.status |= CPU::FLAG_ZERO;
        } else {
            self.status &= !CPU::FLAG_ZERO;
        }
        if (reg & 0b1000_0000) > 0 {
            self.status |= CPU::FLAG_NEGATIVE;
        } else {
            self.status &= !CPU::FLAG_NEGATIVE;
        }
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
        ind_addr = self.sum(ind_addr, self.x);
        self.read16(ind_addr as u16)
    }

    fn fetch_indirect_y_addr(&mut self, read_from_addr: bool) -> u16 {
        let ind_addr = self.read_pc();
        let low = self.read8(ind_addr as u16);
        let high = self.read8((ind_addr + 1) as u16);
        let mut addr = (self.y as u16) + (low as u16);
        if addr > 255 || !read_from_addr {
            // penalty
            self.read8(addr);
        }
        addr += (high as u16) << 8;
        addr
    }

    pub fn process(&mut self, cycles: u32) {
        let init_cycles = self.cycles_run;
        loop {
            let instruction = self.read_pc();
            match instruction {
                CPU::LDA_IMMEDIATE => {
                    self.a = self.read_pc();
                    self.set_load_instructions_flags(self.a);
                }
                CPU::LDA_ZERO => {
                    let addr = self.read_pc();
                    self.a = self.read8(addr as u16);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::LDA_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.x);
                    self.a = self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::LDA_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.a = self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::LDA_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.x, true);
                    self.a = self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::LDA_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.y, true);
                    self.a = self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::LDA_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    self.a = self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::LDA_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(true);
                    self.a = self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::LDX_IMMEDIATE => {
                    self.x = self.read_pc();
                    self.set_load_instructions_flags(self.x);
                }
                CPU::LDX_ZERO => {
                    let addr = self.read_pc();
                    self.x = self.read8(addr as u16);
                    self.set_load_instructions_flags(self.x);
                }
                CPU::LDX_ZERO_Y => {
                    let addr = self.fetch_zero_page_addr(self.y);
                    self.x = self.read8(addr);
                    self.set_load_instructions_flags(self.x);
                }
                CPU::LDX_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.x = self.read8(addr);
                    self.set_load_instructions_flags(self.x);
                }
                CPU::LDX_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.y, true);
                    self.x = self.read8(addr);
                    self.set_load_instructions_flags(self.x);
                }
                CPU::LDY_IMMEDIATE => {
                    self.y = self.read_pc();
                    self.set_load_instructions_flags(self.y);
                }
                CPU::LDY_ZERO => {
                    let addr = self.read_pc();
                    self.y = self.read8(addr as u16);
                    self.set_load_instructions_flags(self.y);
                }
                CPU::LDY_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.x);
                    self.y = self.read8(addr);
                    self.set_load_instructions_flags(self.y);
                }
                CPU::LDY_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.y = self.read8(addr);
                    self.set_load_instructions_flags(self.y);
                }
                CPU::LDY_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.x, true);
                    self.y = self.read8(addr);
                    self.set_load_instructions_flags(self.y);
                }
                CPU::STA_ZERO => {
                    let addr = self.read_pc();
                    self.write8(addr as u16, self.a);
                }
                CPU::STA_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.x);
                    self.write8(addr, self.a);
                }
                CPU::STA_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.write8(addr, self.a);
                }
                CPU::STA_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.x, false);
                    self.write8(addr, self.a);
                }
                CPU::STA_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.y, false);
                    self.write8(addr, self.a);
                }
                CPU::STA_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    self.write8(addr, self.a);
                }
                CPU::STA_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(false);
                    self.write8(addr, self.a);
                }
                CPU::STX_ZERO => {
                    let addr = self.read_pc();
                    self.write8(addr as u16, self.x);
                }
                CPU::STX_ZERO_Y => {
                    let addr = self.fetch_zero_page_addr(self.y);
                    self.write8(addr, self.x);
                }
                CPU::STX_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.write8(addr, self.x);
                }
                CPU::STY_ZERO => {
                    let addr = self.read_pc();
                    self.write8(addr as u16, self.y);
                }
                CPU::STY_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.x);
                    self.write8(addr, self.y);
                }
                CPU::STY_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.write8(addr, self.y);
                }
                CPU::TRANS_A_TO_X => {
                    self.x = self.a;
                    self.cycles_run += 1;
                    self.set_load_instructions_flags(self.x);
                }
                CPU::TRANS_A_TO_Y => {
                    self.y = self.a;
                    self.cycles_run += 1;
                    self.set_load_instructions_flags(self.y);
                }
                CPU::TRANS_X_TO_A => {
                    self.a = self.x;
                    self.cycles_run += 1;
                    self.set_load_instructions_flags(self.a);
                }
                CPU::TRANS_Y_TO_A => {
                    self.a = self.y;
                    self.cycles_run += 1;
                    self.set_load_instructions_flags(self.a);
                }
                CPU::TRANS_SP_TO_X => {
                    self.x = self.sp;
                    self.cycles_run += 1;
                    self.set_load_instructions_flags(self.x);
                }
                CPU::TRANS_X_TO_SP => {
                    self.sp = self.x;
                    self.cycles_run += 1;
                }
                CPU::PUSH_A_TO_SP => {
                    self.write_to_stack(self.a);
                }
                CPU::PUSH_STAT_TO_SP => {
                    self.write_to_stack(self.status);
                }
                CPU::PULL_SP_TO_A => {
                    self.a = self.read_from_stack();
                    self.set_load_instructions_flags(self.a);
                }
                CPU::PULL_SP_TO_STAT => {
                    self.status = self.read_from_stack();
                }
                CPU::AND_IMMEDIATE => {
                    self.a &= self.read_pc();
                    self.set_load_instructions_flags(self.a);
                }
                CPU::AND_ZERO => {
                    let addr = self.read_pc();
                    self.a &= self.read8(addr as u16);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::AND_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.x);
                    self.a &= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::AND_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.a &= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::AND_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.x, true);
                    self.a &= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::AND_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.y, true);
                    self.a &= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::AND_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    self.a &= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::AND_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(true);
                    self.a &= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }

                CPU::EOR_IMMEDIATE => {
                    self.a ^= self.read_pc();
                    self.set_load_instructions_flags(self.a);
                }
                CPU::EOR_ZERO => {
                    let addr = self.read_pc();
                    self.a ^= self.read8(addr as u16);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::EOR_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.x);
                    self.a ^= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::EOR_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.a ^= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::EOR_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.x, true);
                    self.a ^= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::EOR_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.y, true);
                    self.a ^= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::EOR_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    self.a ^= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::EOR_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(true);
                    self.a ^= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }

                CPU::ORA_IMMEDIATE => {
                    self.a |= self.read_pc();
                    self.set_load_instructions_flags(self.a);
                }
                CPU::ORA_ZERO => {
                    let addr = self.read_pc();
                    self.a |= self.read8(addr as u16);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::ORA_ZERO_X => {
                    let addr = self.fetch_zero_page_addr(self.x);
                    self.a |= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::ORA_ABSOLUTE => {
                    let addr = self.fetch_absolute_addr();
                    self.a |= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::ORA_ABSOLUTE_X => {
                    let addr = self.fetch_absolute_indexed_addr(self.x, true);
                    self.a |= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::ORA_ABSOLUTE_Y => {
                    let addr = self.fetch_absolute_indexed_addr(self.y, true);
                    self.a |= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::ORA_INDIRECT_X => {
                    let addr = self.fetch_indirect_x_addr();
                    self.a |= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
                }
                CPU::ORA_INDIRECT_Y => {
                    let addr = self.fetch_indirect_y_addr(true);
                    self.a |= self.read8(addr);
                    self.set_load_instructions_flags(self.a);
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
