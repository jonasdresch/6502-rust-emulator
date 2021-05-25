use emulator6502::Mem;
use rstest::fixture;

pub struct Operation {
    pub cycles: u32, // Cycles to execute
    pub bytes: i32,  // Bytes used
    pub mem: Mem,    // Memory instance
    pub addr: u16,   // Absolute/indexed operations calculated address
}

#[fixture]
pub fn mem_implied(#[default(0)] instruction: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instruction]);
    Operation { cycles: 2, bytes: 1, mem, addr: 0 }
}

#[fixture]
pub fn mem_imm(#[default(0)] instruction: u8, #[default(0)] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instruction, value]);
    Operation { cycles: 2, bytes: 2, mem, addr: 0 }
}

#[fixture]
pub fn mem_zero(#[default(0)] instruction: u8, #[default(0)] addr: u8, #[default(0)] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instruction, addr]);
    mem.write8(addr as usize, value);
    Operation { cycles: 3, bytes: 2, mem, addr: addr as u16 }
}

#[fixture]
pub fn mem_zero_read_store(#[default(0)] instruction: u8, #[default(0)] addr: u8, #[default(0)] value: u8) -> Operation {
    let mut op = mem_zero(instruction, addr, value);
    op.cycles = 5;
    op
}

#[fixture]
pub fn mem_zero_index(#[default(0)] instruction: u8, #[default(0)] addr: u8, #[default(0)] index: u8, #[default(0)] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instruction, addr]);
    let real_addr = (addr as u16 + index as u16) as u8;
    mem.write8(real_addr as usize, value);
    Operation { cycles: 4, bytes: 2, mem, addr: real_addr as u16 }
}

#[fixture]
pub fn mem_zero_index_read_store(#[default(0)] instruction: u8, #[default(0)] addr: u8, #[default(0)] index: u8, #[default(0)] value: u8) -> Operation {
    let mut op = mem_zero_index(instruction, addr, index, value);
    op.cycles = 6;
    op
}

#[fixture]
pub fn mem_abs(#[default(0)] instruction: u8, #[default(0)] addr: u16, #[default(0)] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instruction, addr as u8, ((addr & 0xFF00) >> 8) as u8]);
    mem.write8(addr as usize, value);
    Operation { cycles: 4, bytes: 3, mem, addr: addr as u16 }
}

#[fixture]
pub fn mem_abs_read_store(#[default(0)] instruction: u8, #[default(0)] addr: u16, #[default(0)] value: u8) -> Operation {
    let mut op = mem_abs(instruction, addr, value);
    op.cycles = 6;
    op
}

#[fixture]
pub fn mem_abs_index(#[default(0)] instruction: u8, #[default(0)] addr: u16, #[default(0)] index: u8, #[default(0)] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instruction, addr as u8, ((addr & 0xFF00) >> 8) as u8]);
    let real_addr = addr + index as u16;
    mem.write8(real_addr as usize, value);
    let mut cycles = 4;
    if index as u16 + (addr as u8) as u16 > 255 {
        cycles = 5
    }
    Operation { cycles, bytes: 3, mem, addr: real_addr as u16 }
}

#[fixture]
pub fn mem_abs_index_store(#[default(0)] instruction: u8, #[default(0)] addr: u16, #[default(0)] index: u8) -> Operation {
    let mut op = mem_abs_index(instruction, addr, index, 0);
    op.cycles = 5;
    op
}

#[fixture]
pub fn mem_abs_index_read_store(#[default(0)] instruction: u8, #[default(0)] addr: u16, #[default(0)] index: u8, #[default(0)] value: u8) -> Operation {
    let mut op = mem_abs_index(instruction, addr, index, value);
    op.cycles = 7;
    op
}

#[fixture]
pub fn mem_ind_x(#[default(0)] instruction: u8, #[default(0)] ind_addr: u8, #[default(0)] addr: u16, #[default(0)] index: u8, #[default(0)] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instruction, ind_addr]);
    // we only want the lower byte
    let real_addr = (ind_addr as u16 + index as u16) as u8;
    mem.write16(real_addr as usize, addr);
    mem.write8(addr as usize, value);
    Operation { cycles: 6, bytes: 2, mem, addr: addr as u16 }
}

#[fixture]
pub fn mem_ind_x_store(#[default(0)] instruction: u8, #[default(0)] ind_addr: u8, #[default(0)] addr: u16, #[default(0)] index: u8) -> Operation {
    let mut op = mem_ind_x(instruction, ind_addr, addr, index, 0);
    op.cycles = 6;
    op
}

#[fixture]
pub fn mem_ind_y(#[default(0)] instruction: u8, #[default(0)] ind_addr: u8, #[default(0)] addr: u16, #[default(0)] index: u8, #[default(0)] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instruction, ind_addr]);
    // we only want the lower byte
    let real_addr = addr + index as u16;
    mem.write16(ind_addr as usize, addr);
    mem.write8(real_addr as usize, value);
    let mut cycles = 5;
    if index as u16 + (addr as u8) as u16 > 255 {
        cycles = 6
    }
    Operation { cycles, bytes: 2, mem, addr: real_addr }
}

#[fixture]
pub fn mem_ind_y_store(#[default(0)] instruction: u8, #[default(0)] ind_addr: u8, #[default(0)] addr: u16, #[default(0)] index: u8) -> Operation {
    let mut op = mem_ind_y(instruction, ind_addr, addr, index, 0);
    op.cycles = 6;
    op
}

#[fixture]
pub fn mem_trans(#[default(0)] instruction: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instruction]);
    Operation { cycles: 2, bytes: 1, mem, addr: 0 }
}

#[fixture]
pub fn mem_trans_store(#[default(0)] instruction: u8, #[default(0)] addr: u16) -> Operation {
    let mut op = mem_trans(instruction);
    op.addr = addr;
    op.cycles = 3;
    op
}

#[fixture]
pub fn mem_branch(#[default(0)] instruction: u8, #[default(0)] offset: i8, #[default(true)] must_branch: bool, #[default(false)] page_cross: bool, #[default(0)] load_addr: u16) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm_at(load_addr, &[instruction, offset as u8]);
    // the instrunction increments pc by 2 before branching, so we must account it
    let mut new_ofset = 2i32;
    let mut cycles = 2;
    if must_branch {
        cycles += 1;
        new_ofset += offset as i32;
        if page_cross {
            cycles += 1;
        }
    }
    Operation { cycles, bytes: new_ofset, mem, addr: load_addr }
}
