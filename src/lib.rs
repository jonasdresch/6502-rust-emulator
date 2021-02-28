//http://www.obelisk.me.uk/6502/architecture.html
//http://www.emulator101.com/6502-addressing-modes.html

// lifetime anotation <'b>
pub struct CPU<'a>
{
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub status: u8,
    pub cycles_run: u32,
    mem: &'a mut MEM
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

    // status flags
    pub const FLAG_CARRY: u8 = 0b0100_0000;
    pub const FLAG_ZERO: u8 = 0b0010_0000;
    pub const FLAG_INTERRUPT: u8 = 0b0001_0000;
    pub const FLAG_DECIMAL: u8 = 0b0000_1000;
    pub const FLAG_BREAK: u8 = 0b0000_0100;
    pub const FLAG_OVERFLOW: u8 = 0b0000_0010;
    pub const FLAG_NEGATIVE: u8 = 0b0000_0001;

    pub fn new(mem: &'a mut MEM) -> Self {
        CPU {pc:0, sp:0, a:0, x:0, y:0, status:0, cycles_run:0, mem: mem}
    }

    pub fn reset(&mut self) {
        self.pc = (self.mem.read8(RESET_VECTOR_ADDR) as u16) << 8 | self.mem.read8(RESET_VECTOR_ADDR+1) as u16;
        self.status = 0;
        self.cycles_run = 0;
    }

    fn read8(&mut self, addr: u16) -> u8{
        let val = self.mem.read8(addr as usize);
        self.cycles_run += 1;
        return val;
    }

    fn read16(&mut self, addr: u16) -> u16{
        let low = self.read8(addr);
        let high = self.read8(addr+1);
        let val = (high as u16) << 8 | low as u16;
        return val;
    }

    fn write8(&mut self, addr: u16, val: u8) {
        self.mem.write8(addr as usize, val);
        self.cycles_run += 1;
    }

    fn read_pc(&mut self) -> u8{
        let val = self.read8(self.pc);
        self.pc += 1;
        return val;
    }

    fn sum(&mut self, val1: u8, val2: u8) -> u8{
        self.cycles_run += 1;
        let sum = val1 as u16 + val2 as u16;
        return sum as u8;
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

    fn set_load_flags(&mut self, reg: u8) {
        if reg == 0 {
            self.status |= CPU::FLAG_ZERO;
        }
        else {
            self.status &= !CPU::FLAG_ZERO;
        }
        if (reg & 0b1000_0000) > 0 {
            self.status |= CPU::FLAG_NEGATIVE;
        }
        else {
            self.status &= !CPU::FLAG_NEGATIVE;
        }
    }

    pub fn process(&mut self, cycles: u32) {
        let init_cycles = self.cycles_run;
        loop {
            let instruction = self.read_pc();
            match instruction {
                CPU::LDA_IMMEDIATE => {
                    println!("LDA immediate");
                    self.a = self.read_pc();
                    self.set_load_flags(self.a);
                }
                CPU::LDA_ZERO => {
                    println!("LDA Zero page");
                    let addr = self.read_pc();
                    self.a = self.read8(addr as u16);
                    self.set_load_flags(self.a);
                }
                CPU::LDA_ZERO_X => {
                    println!("LDA Zero page X");
                    let val = self.read_pc();
                    let addr = self.sum(val, self.x);
                    self.a = self.read8(addr as u16);
                    self.set_load_flags(self.a);
                }
                CPU::LDA_ABSOLUTE => {
                    println!("LDA Absolute");
                    let low = self.read_pc() as u16;
                    let high = self.read_pc() as u16;
                    let addr = high << 8 | low;
                    self.a = self.read8(addr);
                    self.set_load_flags(self.a);
                }
                CPU::LDA_ABSOLUTE_X => {
                    // https://retrocomputing.stackexchange.com/questions/15621/why-dont-all-absolute-x-instructions-take-an-extra-cycle-to-cross-page-boundari
                    println!("LDA Absolute X");
                    let low = self.read_pc();
                    let high = self.read_pc();
                    let mut addr = (self.x as u16) + (low as u16);
                    if addr > 255 {
                        // penalty
                        self.a = self.read8(addr);
                    }
                    addr += (high as u16) << 8;
                    self.a = self.read8(addr);
                    self.set_load_flags(self.a);
                }
                CPU::LDA_ABSOLUTE_Y => {
                    println!("LDA Absolute Y");
                    let low = self.read_pc();
                    let high = self.read_pc();
                    let mut addr = (self.y as u16) + (low as u16);
                    if addr > 255 {
                        // penalty
                        self.a = self.read8(addr);
                    }
                    addr += (high as u16) << 8;
                    self.a = self.read8(addr);
                    self.set_load_flags(self.a);
                }
                CPU::LDA_INDIRECT_X => {
                    println!("LDA Indirect X");
                    let mut ind_addr = self.read_pc();
                    ind_addr = self.sum(ind_addr, self.x);
                    let addr = self.read16(ind_addr as u16);
                    self.a = self.read8(addr);
                    self.set_load_flags(self.a);
                }
                CPU::LDA_INDIRECT_Y => {
                    println!("LDA Indirect Y");
                    let ind_addr = self.read_pc();
                    let low = self.read8(ind_addr as u16);
                    let high = self.read8((ind_addr+1) as u16);
                    let mut addr = (self.y as u16) + (low as u16);
                    if addr > 255 {
                        // penalty
                        self.a = self.read8(addr);
                    }
                    addr += (high as u16) << 8;
                    self.a = self.read8(addr);
                    self.set_load_flags(self.a);
                }
                CPU::LDX_IMMEDIATE => {
                    println!("LDX immediate");
                    self.x = self.read_pc();
                    self.set_load_flags(self.x);
                }
                CPU::LDX_ZERO => {
                    println!("LDX Zero page");
                    let addr = self.read_pc();
                    self.x = self.read8(addr as u16);
                    self.set_load_flags(self.x);
                }
                CPU::LDX_ZERO_Y => {
                    println!("LDX Zero page Y");
                    let val = self.read_pc();
                    let addr = self.sum(val, self.y);
                    self.x = self.read8(addr as u16);
                    self.set_load_flags(self.x);
                }
                CPU::LDX_ABSOLUTE => {
                    println!("LDX Absolute");
                    let low = self.read_pc() as u16;
                    let high = self.read_pc() as u16;
                    let addr = high << 8 | low;
                    self.x = self.read8(addr);
                    self.set_load_flags(self.x);
                }
                CPU::LDX_ABSOLUTE_Y => {
                    // https://retrocomputing.stackexchange.com/questions/15621/why-dont-all-absolute-x-instructions-take-an-extra-cycle-to-cross-page-boundari
                    println!("LDX Absolute Y");
                    let low = self.read_pc();
                    let high = self.read_pc();
                    let mut addr = (self.y as u16) + (low as u16);
                    if addr > 255 {
                        // penalty
                        self.x = self.read8(addr);
                    }
                    addr += (high as u16) << 8;
                    self.x = self.read8(addr);
                    self.set_load_flags(self.x);
                }
                CPU::LDY_IMMEDIATE => {
                    println!("LDY immediate");
                    self.y = self.read_pc();
                    self.set_load_flags(self.y);
                }
                CPU::LDY_ZERO => {
                    println!("LDY Zero page");
                    let addr = self.read_pc();
                    self.y = self.read8(addr as u16);
                    self.set_load_flags(self.y);
                }
                CPU::LDY_ZERO_X => {
                    println!("LDY Zero page X");
                    let val = self.read_pc();
                    let addr = self.sum(val, self.x);
                    self.y = self.read8(addr as u16);
                    self.set_load_flags(self.y);
                }
                CPU::LDY_ABSOLUTE => {
                    println!("LDY Absolute");
                    let low = self.read_pc() as u16;
                    let high = self.read_pc() as u16;
                    let addr = high << 8 | low;
                    self.y = self.read8(addr);
                    self.set_load_flags(self.y);
                }
                CPU::LDY_ABSOLUTE_X => {
                    // https://retrocomputing.stackexchange.com/questions/15621/why-dont-all-absolute-x-instructions-take-an-extra-cycle-to-cross-page-boundari
                    println!("LDY Absolute X");
                    let low = self.read_pc();
                    let high = self.read_pc();
                    let mut addr = (self.x as u16) + (low as u16);
                    if addr > 255 {
                        // penalty
                        self.y = self.read8(addr);
                    }
                    addr += (high as u16) << 8;
                    self.y = self.read8(addr);
                    self.set_load_flags(self.y);
                }
                CPU::STA_ZERO => {
                    println!("STA Zero page");
                    let addr = self.read_pc();
                    self.write8(addr as u16, self.a);
                }
                CPU::STA_ZERO_X => {
                    println!("STA Zero page X");
                    let mut addr = self.read_pc();
                    addr = self.sum(addr, self.x);
                    self.write8(addr as u16, self.a);
                }
                CPU::STA_ABSOLUTE => {
                    println!("STA Absolute");
                    let low = self.read_pc() as u16;
                    let high = self.read_pc() as u16;
                    let addr = high << 8 | low;
                    self.write8(addr, self.a);
                }
                CPU::STA_ABSOLUTE_X => {
                    println!("STA Absolute X");
                    let low = self.read_pc();
                    let high = self.read_pc();
                    let mut addr = (self.x as u16) + (low as u16);
                    if addr > 255 {
                        // penalty
                        self.read8(addr);
                    }
                    addr += (high as u16) << 8;
                    self.write8(addr, self.a);
                }
                CPU::STA_ABSOLUTE_Y => {
                    println!("STA Absolute Y");
                    let low = self.read_pc();
                    let high = self.read_pc();
                    let mut addr = (self.y as u16) + (low as u16);
                    if addr > 255 {
                        // penalty
                        self.read8(addr);
                    }
                    addr += (high as u16) << 8;
                    self.write8(addr, self.a);
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
pub struct MEM
{
    mem: [u8; MEM_SIZE]
}

impl MEM {
    pub fn new() -> MEM {
        MEM {mem : [0; MEM_SIZE]}
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
        self.write8(address+1, ((value & 0xFF00) >> 8) as u8);
    }

    pub fn reset(&mut self) {
        self.mem = [0; MEM_SIZE];
        // Execution address of cold reset. based on C64 https://sta.c64.org/cbm64mem.html
        self.mem[RESET_VECTOR_ADDR] = ((RESET_EXEC_ADDRESS >> 8) & 0xFF) as u8;
        self.mem[RESET_VECTOR_ADDR+1] = (RESET_EXEC_ADDRESS & 0x00FF) as u8;
    }
}
