use emulator6502::*;
use rstest::*;

struct Operation {
    cycles: u32,
    bytes: u16,
    mem: Mem,
    addr: u16, // used for store operations
}

#[fixture]
fn mem_implied(#[default = 0] instrunction: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instrunction]);
    Operation { cycles: 2, bytes: 1, mem, addr: 0 }
}

#[fixture]
fn mem_imm(#[default = 0] instrunction: u8, #[default = 0] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instrunction, value]);
    Operation { cycles: 2, bytes: 2, mem, addr: 0 }
}

#[fixture]
fn mem_zero(#[default = 0] instrunction: u8, #[default = 0] addr: u8, #[default = 0] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instrunction, addr]);
    mem.write8(addr as usize, value);
    Operation { cycles: 3, bytes: 2, mem, addr: addr as u16 }
}

#[fixture]
fn mem_zero_read_store(#[default = 0] instrunction: u8, #[default = 0] addr: u8, #[default = 0] value: u8) -> Operation {
    let mut op = mem_zero(instrunction, addr, value);
    op.cycles = 5;
    op
}

#[fixture]
fn mem_zero_index(#[default = 0] instrunction: u8, #[default = 0] addr: u8, #[default = 0] index: u8, #[default = 0] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instrunction, addr]);
    let real_addr = (addr as u16 + index as u16) as u8;
    mem.write8(real_addr as usize, value);
    Operation { cycles: 4, bytes: 2, mem, addr: real_addr as u16 }
}

#[fixture]
fn mem_zero_index_read_store(#[default = 0] instrunction: u8, #[default = 0] addr: u8, #[default = 0] index: u8, #[default = 0] value: u8) -> Operation {
    let mut op = mem_zero_index(instrunction, addr, index, value);
    op.cycles = 6;
    op
}

#[fixture]
fn mem_abs(#[default = 0] instrunction: u8, #[default = 0] addr: u16, #[default = 0] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instrunction, addr as u8, ((addr & 0xFF00) >> 8) as u8]);
    mem.write8(addr as usize, value);
    Operation { cycles: 4, bytes: 3, mem, addr: addr as u16 }
}

#[fixture]
fn mem_abs_read_store(#[default = 0] instrunction: u8, #[default = 0] addr: u16, #[default = 0] value: u8) -> Operation {
    let mut op = mem_abs(instrunction, addr, value);
    op.cycles = 6;
    op
}

#[fixture]
fn mem_abs_index(#[default = 0] instrunction: u8, #[default = 0] addr: u16, #[default = 0] index: u8, #[default = 0] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instrunction, addr as u8, ((addr & 0xFF00) >> 8) as u8]);
    let real_addr = addr + index as u16;
    mem.write8(real_addr as usize, value);
    let mut cycles = 4;
    if index as u16 + (addr as u8) as u16 > 255 {
        cycles = 5
    }
    Operation { cycles, bytes: 3, mem, addr: real_addr as u16 }
}

#[fixture]
fn mem_abs_index_store(#[default = 0] instrunction: u8, #[default = 0] addr: u16, #[default = 0] index: u8) -> Operation {
    let mut op = mem_abs_index(instrunction, addr, index, 0);
    op.cycles = 5;
    op
}

#[fixture]
fn mem_abs_index_read_store(#[default = 0] instrunction: u8, #[default = 0] addr: u16, #[default = 0] index: u8, #[default = 0] value: u8) -> Operation {
    let mut op = mem_abs_index(instrunction, addr, index, value);
    op.cycles = 7;
    op
}

#[fixture]
fn mem_ind_x(#[default = 0] instrunction: u8, #[default = 0] ind_addr: u8, #[default = 0] addr: u16, #[default = 0] index: u8, #[default = 0] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instrunction, ind_addr]);
    // we only want the lower byte
    let real_addr = (ind_addr as u16 + index as u16) as u8;
    mem.write16(real_addr as usize, addr);
    mem.write8(addr as usize, value);
    Operation { cycles: 6, bytes: 2, mem, addr: addr as u16 }
}

#[fixture]
fn mem_ind_x_store(#[default = 0] instrunction: u8, #[default = 0] ind_addr: u8, #[default = 0] addr: u16, #[default = 0] index: u8) -> Operation {
    let mut op = mem_ind_x(instrunction, ind_addr, addr, index, 0);
    op.cycles = 6;
    op
}

#[fixture]
fn mem_ind_y(#[default = 0] instrunction: u8, #[default = 0] ind_addr: u8, #[default = 0] addr: u16, #[default = 0] index: u8, #[default = 0] value: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instrunction, ind_addr]);
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
fn mem_ind_y_store(#[default = 0] instrunction: u8, #[default = 0] ind_addr: u8, #[default = 0] addr: u16, #[default = 0] index: u8) -> Operation {
    let mut op = mem_ind_y(instrunction, ind_addr, addr, index, 0);
    op.cycles = 6;
    op
}

#[fixture]
fn mem_trans(#[default = 0] instrunction: u8) -> Operation {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[instrunction]);
    Operation { cycles: 2, bytes: 1, mem, addr: 0 }
}

#[fixture]
fn mem_trans_store(#[default = 0] instrunction: u8, #[default = 0] addr: u16) -> Operation {
    let mut op = mem_trans(instrunction);
    op.addr = addr;
    op.cycles = 3;
    op
}

#[rstest]
// LDA
#[case::lda_imm1(mem_imm(Cpu::LDA_IMMEDIATE, 0xCA), 0xCA, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, 10, 0)]
#[case::lda_imm2(mem_imm(Cpu::LDA_IMMEDIATE, 0x0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, 10, 0)]
#[case::lda_zero1(mem_zero(Cpu::LDA_ZERO, 0xCA, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, 10, 0)]
#[case::lda_zero2(mem_zero(Cpu::LDA_ZERO, 0, 0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, 10, 0)]
#[case::lda_zero_x1(mem_zero_index(Cpu::LDA_ZERO_X, 0x80, 0x0F, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::lda_zero_x2(mem_zero_index(Cpu::LDA_ZERO_X, 0x80, 0xFF, 0xA), 0xA, 0, Cpu::REG_A, 0, Cpu::REG_X, 0xFF)]
#[case::lda_abs1(mem_abs(Cpu::LDA_ABSOLUTE, 0x1234, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, 10, 0)]
#[case::lda_abs_x1(mem_abs_index(Cpu::LDA_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::lda_abs_x2(mem_abs_index(Cpu::LDA_ABSOLUTE_X, 0x12AA, 0xBB, 0x11), 0x11, 0, Cpu::REG_A, 0, Cpu::REG_X, 0xBB)]
#[case::lda_abs_y1(mem_abs_index(Cpu::LDA_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_Y, 0x0F)]
#[case::lda_abs_y2(mem_abs_index(Cpu::LDA_ABSOLUTE_Y, 0x12AA, 0xBB, 0x11), 0x11, 0, Cpu::REG_A, 0, Cpu::REG_Y, 0xBB)]
#[case::lda_ind_x1(mem_ind_x(Cpu::LDA_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::lda_ind_x2(mem_ind_x(Cpu::LDA_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x11), 0x11, 0, Cpu::REG_A, 0, Cpu::REG_X, 0xBB)]
#[case::lda_ind_y1(mem_ind_y(Cpu::LDA_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_Y, 0x0F)]
#[case::lda_ind_y2(mem_ind_y(Cpu::LDA_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x11), 0x11, 0, Cpu::REG_A, 0, Cpu::REG_Y, 0xBB)]
// LDX
#[case::ldx_imm1(mem_imm(Cpu::LDX_IMMEDIATE, 0xCA), 0xCA, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, 10, 0)]
#[case::ldx_imm2(mem_imm(Cpu::LDX_IMMEDIATE, 0x0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_X, 0, 10, 0)]
#[case::ldx_zero1(mem_zero(Cpu::LDX_ZERO, 0xCA, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, 10, 0)]
#[case::ldx_zero2(mem_zero(Cpu::LDX_ZERO, 0xCB, 0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_X, 0, 10, 0)]
#[case::ldx_zero_y1(mem_zero_index(Cpu::LDX_ZERO_Y, 0x80, 0x0F, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, Cpu::REG_Y, 0x0F)]
#[case::ldx_zero_y2(mem_zero_index(Cpu::LDX_ZERO_Y, 0x80, 0xFF, 0xA), 0xA, 0, Cpu::REG_X, 0, Cpu::REG_Y, 0xFF)]
#[case::ldx_abs1(mem_abs(Cpu::LDX_ABSOLUTE, 0x1234, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, 10, 0)]
#[case::ldx_abs_y1(mem_abs_index(Cpu::LDX_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, Cpu::REG_Y, 0x0F)]
#[case::ldx_abs_y2(mem_abs_index(Cpu::LDX_ABSOLUTE_Y, 0x12AA, 0xBB, 0x11), 0x11, 0, Cpu::REG_X, 0, Cpu::REG_Y, 0xBB)]
// LDY
#[case::ldy_imm1(mem_imm(Cpu::LDY_IMMEDIATE, 0xCA), 0xCA, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, 10, 0)]
#[case::ldy_imm2(mem_imm(Cpu::LDY_IMMEDIATE, 0x0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0, 10, 0)]
#[case::ldy_zero1(mem_zero(Cpu::LDY_ZERO, 0xCA, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, 10, 0)]
#[case::ldy_zero2(mem_zero(Cpu::LDY_ZERO, 0xCB, 0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0, 10, 0)]
#[case::ldy_zero_x1(mem_zero_index(Cpu::LDY_ZERO_X, 0x80, 0x0F, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, Cpu::REG_X, 0x0F)]
#[case::ldy_zero_x2(mem_zero_index(Cpu::LDY_ZERO_X, 0x80, 0xFF, 0xA), 0xA, 0, Cpu::REG_Y, 0, Cpu::REG_X, 0xFF)]
#[case::ldy_abs1(mem_abs(Cpu::LDY_ABSOLUTE, 0x1234, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, 10, 0)]
#[case::ldy_abs_x1(mem_abs_index(Cpu::LDY_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, Cpu::REG_X, 0x0F)]
#[case::ldy_abs_x2(mem_abs_index(Cpu::LDY_ABSOLUTE_X, 0x12AA, 0xBB, 0x11), 0x11, 0, Cpu::REG_Y, 0, Cpu::REG_X, 0xBB)]
// TRANS
#[case::trans_a_to_x1(mem_trans(Cpu::TRANS_A_TO_X), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, Cpu::REG_A, 0xFE)]
#[case::trans_a_to_x2(mem_trans(Cpu::TRANS_A_TO_X), 0, Cpu::FLAG_ZERO, Cpu::REG_X, 0, Cpu::REG_A, 0)]
#[case::trans_a_to_y1(mem_trans(Cpu::TRANS_A_TO_Y), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, Cpu::REG_A, 0xFE)]
#[case::trans_a_to_y2(mem_trans(Cpu::TRANS_A_TO_Y), 0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0, Cpu::REG_A, 0)]
#[case::trans_x_to_a1(mem_trans(Cpu::TRANS_X_TO_A), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_X, 0xFE)]
#[case::trans_x_to_a2(mem_trans(Cpu::TRANS_X_TO_A), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_X, 0)]
#[case::trans_y_to_a1(mem_trans(Cpu::TRANS_Y_TO_A), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_Y, 0xFE)]
#[case::trans_y_to_a2(mem_trans(Cpu::TRANS_Y_TO_A), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_Y, 0)]
#[case::trans_sp_to_x1(mem_trans(Cpu::TRANS_SP_TO_X), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, Cpu::REG_SP, 0xFE)]
#[case::trans_sp_to_x2(mem_trans(Cpu::TRANS_SP_TO_X), 0, Cpu::FLAG_ZERO, Cpu::REG_X, 0, Cpu::REG_SP, 0)]
#[case::trans_x_to_sp(mem_trans(Cpu::TRANS_X_TO_SP), 0xFE, 0, Cpu::REG_SP, 0, Cpu::REG_X, 0xFE)]
#[case::trans_x_to_sp(mem_trans(Cpu::TRANS_X_TO_SP), 0, 0, Cpu::REG_SP, 0, Cpu::REG_X, 0)]
// AND
#[case::and_imm1(mem_imm(Cpu::AND_IMMEDIATE, 0xCA), 0xA, 0, Cpu::REG_A, 0xB, 10, 0)]
#[case::and_imm2(mem_imm(Cpu::AND_IMMEDIATE, 0x12), 0x2, 0, Cpu::REG_A, 0xB, 10, 0)]
#[case::and_zero1(mem_zero(Cpu::AND_ZERO, 0x1, 0xCA), 0xA, 0, Cpu::REG_A, 0xB, 10, 0)]
#[case::and_zero2(mem_zero(Cpu::AND_ZERO, 0x2, 0x12), 0x2, 0, Cpu::REG_A, 0xA, 10, 0)]
#[case::and_zero_x1(mem_zero_index(Cpu::AND_ZERO_X, 0x80, 0x0F, 0xCA), 0xA, 0, Cpu::REG_A, 0xB, Cpu::REG_X, 0x0F)]
#[case::and_zero_x2(mem_zero_index(Cpu::AND_ZERO_X, 0x80, 0xFF, 0x12), 0x2, 0, Cpu::REG_A, 0xA, Cpu::REG_X, 0xFF)]
#[case::and_abs1(mem_abs(Cpu::AND_ABSOLUTE, 0x1234, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, 10, 0)]
#[case::and_abs_x1(mem_abs_index(Cpu::AND_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::and_abs_x2(mem_abs_index(Cpu::AND_ABSOLUTE_X, 0x12AA, 0xBB, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xA1, Cpu::REG_X, 0xBB)]
#[case::and_abs_y1(mem_abs_index(Cpu::AND_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::and_abs_y2(mem_abs_index(Cpu::AND_ABSOLUTE_Y, 0x12AA, 0xBB, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xA1, Cpu::REG_Y, 0xBB)]
#[case::and_ind_x1(mem_ind_x(Cpu::AND_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::and_ind_x2(mem_ind_x(Cpu::AND_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_X, 0xBB)]
#[case::and_ind_y1(mem_ind_y(Cpu::AND_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::and_ind_y2(mem_ind_y(Cpu::AND_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_Y, 0xBB)]
// EOR
#[case::eor_imm1(mem_imm(Cpu::EOR_IMMEDIATE, 0xCA), 0xC1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, 10, 0)]
#[case::eor_imm2(mem_imm(Cpu::EOR_IMMEDIATE, 0x12), 0xD3, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xC1, 10, 0)]
#[case::eor_zero1(mem_zero(Cpu::EOR_ZERO, 0x1, 0xCA), 0xC1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, 10, 0)]
#[case::eor_zero2(mem_zero(Cpu::EOR_ZERO, 0x2, 0x12), 0xD3, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xC1, 10, 0)]
#[case::eor_zero_x1(mem_zero_index(Cpu::EOR_ZERO_X, 0x80, 0x0F, 0xCA), 0xC1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, Cpu::REG_X, 0x0F)]
#[case::eor_zero_x2(mem_zero_index(Cpu::EOR_ZERO_X, 0x80, 0xFF, 0x12), 0xD3, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xC1, Cpu::REG_X, 0xFF)]
#[case::eor_abs1(mem_abs(Cpu::EOR_ABSOLUTE, 0x1234, 0xAB), 0x1A, 0, Cpu::REG_A, 0xB1, 10, 0)]
#[case::eor_abs_x1(mem_abs_index(Cpu::EOR_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0x1A, 0, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::eor_abs_x2(mem_abs_index(Cpu::EOR_ABSOLUTE_X, 0x12AA, 0xBB, 0x5E), 0x44, 0, Cpu::REG_A, 0x1A, Cpu::REG_X, 0xBB)]
#[case::eor_abs_y1(mem_abs_index(Cpu::EOR_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0x1A, 0, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::eor_abs_y2(mem_abs_index(Cpu::EOR_ABSOLUTE_Y, 0x12AA, 0xBB, 0x5E), 0x44, 0, Cpu::REG_A, 0x1A, Cpu::REG_Y, 0xBB)]
#[case::eor_ind_x1(mem_ind_x(Cpu::EOR_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0x1A, 0, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::eor_ind_x2(mem_ind_x(Cpu::EOR_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x5E), 0x44, 0, Cpu::REG_A, 0x1A, Cpu::REG_X, 0xBB)]
#[case::eor_ind_y1(mem_ind_y(Cpu::EOR_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0x1A, 0, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::eor_ind_y2(mem_ind_y(Cpu::EOR_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x5E), 0x44, 0, Cpu::REG_A, 0x1A, Cpu::REG_Y, 0xBB)]
// ORA
#[case::ora_imm1(mem_imm(Cpu::ORA_IMMEDIATE, 0xCA), 0xCB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, 10, 0)]
#[case::ora_imm2(mem_imm(Cpu::ORA_IMMEDIATE, 0x12), 0xDB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xCB, 10, 0)]
#[case::ora_zero1(mem_zero(Cpu::ORA_ZERO, 0x1, 0xCA), 0xCB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, 10, 0)]
#[case::ora_zero2(mem_zero(Cpu::ORA_ZERO, 0x2, 0x12), 0xDB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xCB, 10, 0)]
#[case::ora_zero_x1(mem_zero_index(Cpu::ORA_ZERO_X, 0x80, 0x0F, 0xCA), 0xCB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, Cpu::REG_X, 0x0F)]
#[case::ora_zero_x2(mem_zero_index(Cpu::ORA_ZERO_X, 0x80, 0xFF, 0x12), 0xDB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xCB, Cpu::REG_X, 0xFF)]
#[case::ora_abs1(mem_abs(Cpu::ORA_ABSOLUTE, 0x1234, 0xAB), 0xBB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, 10, 0)]
#[case::ora_abs_x1(mem_abs_index(Cpu::ORA_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xBB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::ora_abs_x2(mem_abs_index(Cpu::ORA_ABSOLUTE_X, 0x12AA, 0xBB, 0x5E), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xBB, Cpu::REG_X, 0xBB)]
#[case::ora_abs_y1(mem_abs_index(Cpu::ORA_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xBB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::ora_abs_y2(mem_abs_index(Cpu::ORA_ABSOLUTE_Y, 0x12AA, 0xBB, 0x5E), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xBB, Cpu::REG_Y, 0xBB)]
#[case::ora_ind_x1(mem_ind_x(Cpu::ORA_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0xBB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::ora_ind_x2(mem_ind_x(Cpu::ORA_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x5E), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xBB, Cpu::REG_X, 0xBB)]
#[case::ora_ind_y1(mem_ind_y(Cpu::ORA_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0xBB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::ora_ind_y2(mem_ind_y(Cpu::ORA_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x5E), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xBB, Cpu::REG_Y, 0xBB)]
// ADC
// TODO test input carry flag
// TODO test if status flags are not affected
// NOTE: before adding we must clear the CARRY with CLC: https://retro64.altervista.org/blog/an-introduction-to-6502-math-addiction-subtraction-and-more/
#[case::adc1(mem_imm(Cpu::ADC_IMMEDIATE, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_STAT, 0)]
#[case::adc2(mem_imm(Cpu::ADC_IMMEDIATE, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_STAT, 0)]
#[case::adc_zero1(mem_zero(Cpu::ADC_ZERO, 0x1, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_STAT, 0)]
#[case::adc_zero2(mem_zero(Cpu::ADC_ZERO, 0x2, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_STAT, 0)]
#[case::adc_zero_x1(mem_zero_index(Cpu::ADC_ZERO_X, 0x80, 0x0F, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::adc_zero_x2(mem_zero_index(Cpu::ADC_ZERO_X, 0x80, 0xFF, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_X, 0xFF)]
#[case::adc_abs1(mem_abs(Cpu::ADC_ABSOLUTE, 0x1234, 0x66), 0xDD, Cpu::FLAG_OVERFLOW |Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, 10, 0)]
#[case::adc_abs_x1(mem_abs_index(Cpu::ADC_ABSOLUTE_X, 0x1225, 0x0F, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::adc_abs_x2(mem_abs_index(Cpu::ADC_ABSOLUTE_X, 0x12AA, 0xBB, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_X, 0xBB)]
#[case::adc_abs_y1(mem_abs_index(Cpu::ADC_ABSOLUTE_Y, 0x1225, 0x0F, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_Y, 0x0F)]
#[case::adc_abs_y2(mem_abs_index(Cpu::ADC_ABSOLUTE_Y, 0x12AA, 0xBB, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_Y, 0xBB)]
#[case::adc_ind_x1(mem_ind_x(Cpu::ADC_INDIRECT_X, 0x25, 0x1234, 0x0F, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::adc_ind_x2(mem_ind_x(Cpu::ADC_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_X, 0xBB)]
#[case::adc_ind_y1(mem_ind_y(Cpu::ADC_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_Y, 0x0F)]
#[case::adc_ind_y2(mem_ind_y(Cpu::ADC_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_Y, 0xBB)]
// SBC
// There is not a “borrow flag” in the 6502 CPU. The carry flag is not a borrow either,
// but it acts as a reverse borrow. So, before performing a subtraction,
// we should clear the borrow, or, as it is done in practice, we must SET the carry.
// https://retro64.altervista.org/blog/an-introduction-to-6502-math-addiction-subtraction-and-more/
// http://forum.6502.org/viewtopic.php?t=18
#[case::sbc_imm1(mem_imm(Cpu::SBC_IMMEDIATE, 0x41), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x3F, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::sbc_imm2(mem_imm(Cpu::SBC_IMMEDIATE, 0xFF), 0x02, 0, Cpu::REG_A, 0x1, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::sbc_zero1(mem_zero(Cpu::SBC_ZERO, 0x1, 0x66), 0x11, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::sbc_zero2(mem_zero(Cpu::SBC_ZERO, 0x2, 0x8A), 0x87, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x11, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
// When we do not set the carry flag we assume we are borrowing one and the result will be -1 the
#[case::sbc_zero_x1(mem_zero_index(Cpu::SBC_ZERO_X, 0x80, 0x0F, 0x66), 0x10, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::sbc_zero_x2(mem_zero_index(Cpu::SBC_ZERO_X, 0x80, 0xFF, 0x8A), 0x85, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x10, Cpu::REG_X, 0xFF)]
#[case::sbc_abs1(mem_abs(Cpu::SBC_ABSOLUTE, 0x1234, 0x66), 0x11, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::sbc_abs_x1(mem_abs_index(Cpu::SBC_ABSOLUTE_X, 0x1225, 0x0F, 0x66), 0x10, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::sbc_abs_x2(mem_abs_index(Cpu::SBC_ABSOLUTE_X, 0x12AA, 0xBB, 0x8A), 0x85, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x10, Cpu::REG_X, 0xBB)]
#[case::sbc_abs_y1(mem_abs_index(Cpu::SBC_ABSOLUTE_Y, 0x1225, 0x0F, 0x66), 0x10, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_Y, 0x0F)]
#[case::sbc_abs_y2(mem_abs_index(Cpu::SBC_ABSOLUTE_Y, 0x12AA, 0xBB, 0x8A), 0x85, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x10, Cpu::REG_Y, 0xBB)]
#[case::sbc_ind_x1(mem_ind_x(Cpu::SBC_INDIRECT_X, 0x25, 0x1234, 0x0F, 0x66), 0x10, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::sbc_ind_x2(mem_ind_x(Cpu::SBC_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x8A), 0x85, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x10, Cpu::REG_X, 0xBB)]
#[case::sbc_ind_y1(mem_ind_y(Cpu::SBC_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0x66), 0x10, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_Y, 0x0F)]
#[case::sbc_ind_y2(mem_ind_y(Cpu::SBC_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x8A), 0x85, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x10, Cpu::REG_Y, 0xBB)]
#[case::bit_test_zero1(mem_zero(Cpu::BIT_TEST_ZERO, 0x1, 0xCA), 0xB, Cpu::FLAG_NEGATIVE | Cpu::FLAG_OVERFLOW, Cpu::REG_A, 0xB, 10, 0)]
#[case::bit_test_zero2(mem_zero(Cpu::BIT_TEST_ZERO, 0x2, 0x10), 0xB, Cpu::FLAG_ZERO, Cpu::REG_A, 0xB, 10, 0)]
#[case::bit_test_abs1(mem_abs(Cpu::BIT_TEST_ABSOLUTE, 0x1234, 0xAB), 0x54, Cpu::FLAG_ZERO | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x54, 10, 0)]
// CMP http://www.6502.org/tutorials/compare_beyond.html
#[case::cmp_imm1(mem_imm(Cpu::CMP_IMMEDIATE, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_STAT, 0)]
#[case::cmp_imm2(mem_imm(Cpu::CMP_IMMEDIATE, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_STAT, 0)]
#[case::cmp_zero1(mem_zero(Cpu::CMP_ZERO, 0x1, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_STAT, 0)]
#[case::cmp_zero2(mem_zero(Cpu::CMP_ZERO, 0x2, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_STAT, 0)]
#[case::cmp_zero_x1(mem_zero_index(Cpu::CMP_ZERO_X, 0x70, 0xF, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_zero_x2(mem_zero_index(Cpu::CMP_ZERO_X, 0x70, 0xFF, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_X, 0xFF)]
#[case::cmp_abs1(mem_abs(Cpu::CMP_ABSOLUTE, 0x1234, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_abs2(mem_abs(Cpu::CMP_ABSOLUTE, 0x1234, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_X, 0xFF)]
#[case::cmp_abs_x1(mem_abs_index(Cpu::CMP_ABSOLUTE_X, 0x1234, 0xF, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_abs_x2(mem_abs_index(Cpu::CMP_ABSOLUTE_X, 0x1234, 0xBB, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_X, 0xBB)]
#[case::cmp_abs_y1(mem_abs_index(Cpu::CMP_ABSOLUTE_Y, 0x1234, 0xF, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_Y, 0xF)]
#[case::cmp_abs_y2(mem_abs_index(Cpu::CMP_ABSOLUTE_Y, 0x1234, 0xBB, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_Y, 0xBB)]
#[case::cmp_ind_x1(mem_ind_x(Cpu::CMP_INDIRECT_X, 0x25, 0x1234, 0xF, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_ind_x2(mem_ind_x(Cpu::CMP_INDIRECT_X, 0xAA, 0x1234, 0xBB, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_X, 0xBB)]
#[case::cmp_ind_y1(mem_ind_y(Cpu::CMP_INDIRECT_Y, 0x25, 0x1225, 0xF, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_Y, 0xF)]
#[case::cmp_ind_y2(mem_ind_y(Cpu::CMP_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_Y, 0xBB)]
// CPX
#[case::cpx_imm1(mem_imm(Cpu::CPX_IMMEDIATE, 0xFF), 0x1, 0, Cpu::REG_X, 0x1, Cpu::REG_STAT, 0)]
#[case::cpx_imm2(mem_imm(Cpu::CPX_IMMEDIATE, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0x7F, Cpu::REG_STAT, 0)]
#[case::cpx_zero1(mem_zero(Cpu::CPX_ZERO, 0x1, 0xFF), 0x1, 0, Cpu::REG_X, 0x1, Cpu::REG_STAT, 0)]
#[case::cpx_zero2(mem_zero(Cpu::CPX_ZERO, 0x2, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0x7F, Cpu::REG_STAT, 0)]
#[case::cpx_abs1(mem_abs(Cpu::CPX_ABSOLUTE, 0x1234, 0xFF), 0x1, 0, Cpu::REG_X, 0x1, Cpu::REG_STAT, 0)]
#[case::cpx_abs2(mem_abs(Cpu::CPX_ABSOLUTE, 0x1234, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0x7F, Cpu::REG_STAT, 0)]
// CPY
#[case::cpy_imm1(mem_imm(Cpu::CPY_IMMEDIATE, 0xFF), 0x1, 0, Cpu::REG_Y, 0x1, Cpu::REG_STAT, 0)]
#[case::cpy_imm2(mem_imm(Cpu::CPY_IMMEDIATE, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0x7F, Cpu::REG_STAT, 0)]
#[case::cpy_zero1(mem_zero(Cpu::CPY_ZERO, 0x1, 0xFF), 0x1, 0, Cpu::REG_Y, 0x1, Cpu::REG_STAT, 0)]
#[case::cpy_zero2(mem_zero(Cpu::CPY_ZERO, 0x2, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0x7F, Cpu::REG_STAT, 0)]
#[case::cpy_abs1(mem_abs(Cpu::CPY_ABSOLUTE, 0x1234, 0xFF), 0x1, 0, Cpu::REG_Y, 0x1, Cpu::REG_STAT, 0)]
#[case::cpy_abs2(mem_abs(Cpu::CPY_ABSOLUTE, 0x1234, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0x7F, Cpu::REG_STAT, 0)]
// INX
#[case::inx_implied1(mem_implied(Cpu::INX_IMPLIED), 0x80, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0x7F, 10, 0)]
#[case::inx_implied2(mem_implied(Cpu::INX_IMPLIED), 0, Cpu::FLAG_ZERO, Cpu::REG_X, 0xFF, 10, 0)]
// INY
#[case::iny_implied1(mem_implied(Cpu::INY_IMPLIED), 0x80, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0x7F, 10, 0)]
#[case::iny_implied2(mem_implied(Cpu::INY_IMPLIED), 0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0xFF, 10, 0)]
// DEX
#[case::dex_implied1(mem_implied(Cpu::DEX_IMPLIED), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, 10, 0)]
#[case::dex_implied2(mem_implied(Cpu::DEX_IMPLIED), 0, Cpu::FLAG_ZERO, Cpu::REG_X, 0x1, 10, 0)]
// DEY
#[case::dey_implied1(mem_implied(Cpu::DEY_IMPLIED), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, 10, 0)]
#[case::dey_implied2(mem_implied(Cpu::DEY_IMPLIED), 0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0x1, 10, 0)]
fn load_tests(
    #[case] mut op: Operation,
    #[case] expected_result: u8,
    #[case] expected_stat: u8,
    #[case] to_register: usize,
    #[case] register_init_val: u8,
    #[case] aux_register: usize,
    #[case] aux_register_init_val: u8,
) {
    let mut cpu = Cpu::new(&mut op.mem);
    cpu.reset();
    cpu.regs[to_register] = register_init_val;
    if aux_register <= Cpu::REG_STAT {
        cpu.regs[aux_register] = aux_register_init_val;
    }
    cpu.process(op.cycles);
    assert_eq!(RESET_EXEC_ADDRESS + op.bytes, cpu.pc);
    assert_eq!(expected_result, cpu.regs[to_register]);
    assert_eq!(expected_stat, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(op.cycles, cpu.cycles_run);
}

#[rstest]
// STA
#[case::sta_zero1(mem_zero(Cpu::STA_ZERO, 0xCA, 0), 0xFE, 0, Cpu::REG_A, 0xFE, 10, 0)]
#[case::sta_zero2(mem_zero(Cpu::STA_ZERO, 0xCB, 0), 0x12, 0, Cpu::REG_A, 0x12, 10, 0)]
#[case::sta_zero_x1(mem_zero_index(Cpu::STA_ZERO_X, 0x80, 0x0F, 0), 0xFE, 0, Cpu::REG_A, 0xFE, Cpu::REG_X, 0x0F)]
#[case::sta_zero_x2(mem_zero_index(Cpu::STA_ZERO_X, 0x80, 0xFF, 0), 0xA, 0, Cpu::REG_A, 0xA, Cpu::REG_X, 0xFF)]
#[case::sta_abs1(mem_abs(Cpu::STA_ABSOLUTE, 0x1225, 0), 0xFE, 0, Cpu::REG_A, 0xFE, 10, 0)]
#[case::sta_abs2(mem_abs(Cpu::STA_ABSOLUTE, 0x12AA, 0), 0x12, 0, Cpu::REG_A, 0x12, 10, 0)]
#[case::sta_abs_x1(mem_abs_index_store(Cpu::STA_ABSOLUTE_X, 0x1225, 0x0F), 0xAB, 0, Cpu::REG_A, 0xAB, Cpu::REG_X, 0x0F)]
#[case::sta_abs_x2(mem_abs_index_store(Cpu::STA_ABSOLUTE_X, 0x12AA, 0xBB), 0x11, 0, Cpu::REG_A, 0x11, Cpu::REG_X, 0xBB)]
#[case::sta_abs_y1(mem_abs_index_store(Cpu::STA_ABSOLUTE_Y, 0x1225, 0x0F), 0xAB, 0, Cpu::REG_A, 0xAB, Cpu::REG_Y, 0x0F)]
#[case::sta_abs_y2(mem_abs_index_store(Cpu::STA_ABSOLUTE_Y, 0x12AA, 0xBB), 0x11, 0, Cpu::REG_A, 0x11, Cpu::REG_Y, 0xBB)]
#[case::sta_ind_x1(mem_ind_x_store(Cpu::STA_INDIRECT_X, 0x25, 0x1234, 0x0F), 0xAB, 0, Cpu::REG_A, 0xAB, Cpu::REG_X, 0x0F)]
#[case::sta_ind_x2(mem_ind_x_store(Cpu::STA_INDIRECT_X, 0xAA, 0x1365, 0xBB), 0x11, 0, Cpu::REG_A, 0x11, Cpu::REG_X, 0xBB)]
#[case::sta_ind_y1(mem_ind_y_store(Cpu::STA_INDIRECT_Y, 0x25, 0x1225, 0x0F), 0xAB, 0, Cpu::REG_A, 0xAB, Cpu::REG_Y, 0x0F)]
#[case::sta_ind_y2(mem_ind_y_store(Cpu::STA_INDIRECT_Y, 0xAA, 0x12AA, 0xBB), 0x11, 0, Cpu::REG_A, 0x11, Cpu::REG_Y, 0xBB)]
// STX
#[case::stx_zero1(mem_zero(Cpu::STX_ZERO, 0xCA, 0), 0xFE, 0, Cpu::REG_X, 0xFE, 10, 0)]
#[case::stx_zero2(mem_zero(Cpu::STX_ZERO, 0xCB, 0), 0x12, 0, Cpu::REG_X, 0x12, 10, 0)]
#[case::stx_zero_y1(mem_zero_index(Cpu::STX_ZERO_Y, 0x80, 0x0F, 0), 0xFE, 0, Cpu::REG_X, 0xFE, Cpu::REG_Y, 0x0F)]
#[case::stx_zero_y2(mem_zero_index(Cpu::STX_ZERO_Y, 0x80, 0xFF, 0), 0xA, 0, Cpu::REG_X, 0xA, Cpu::REG_Y, 0xFF)]
#[case::stx_abs1(mem_abs(Cpu::STX_ABSOLUTE, 0x1225, 0), 0xFE, 0, Cpu::REG_X, 0xFE, 10, 0)]
#[case::stx_abs2(mem_abs(Cpu::STX_ABSOLUTE, 0x12AA, 0), 0x12, 0, Cpu::REG_X, 0x12, 10, 0)]
#[case::stx_zero1(mem_zero(Cpu::STX_ZERO, 0xCA, 0), 0xFE, 0, Cpu::REG_X, 0xFE, 10, 0)]
#[case::stx_zero2(mem_zero(Cpu::STX_ZERO, 0xCB, 0), 0x12, 0, Cpu::REG_X, 0x12, 10, 0)]
#[case::stx_zero_y1(mem_zero_index(Cpu::STX_ZERO_Y, 0x80, 0x0F, 0), 0xFE, 0, Cpu::REG_X, 0xFE, Cpu::REG_Y, 0x0F)]
#[case::stx_zero_y2(mem_zero_index(Cpu::STX_ZERO_Y, 0x80, 0xFF, 0), 0xA, 0, Cpu::REG_X, 0xA, Cpu::REG_Y, 0xFF)]
#[case::stx_abs1(mem_abs(Cpu::STX_ABSOLUTE, 0x1225, 0), 0xFE, 0, Cpu::REG_X, 0xFE, 10, 0)]
#[case::stx_abs2(mem_abs(Cpu::STX_ABSOLUTE, 0x12AA, 0), 0x12, 0, Cpu::REG_X, 0x12, 10, 0)]
// INC
#[case::inc_zero1(mem_zero_read_store(Cpu::INC_ZERO, 0xCA, 0x7F), 0x80, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, 10, 0)]
#[case::inc_zero2(mem_zero_read_store(Cpu::INC_ZERO, 0xCB, 0xFF), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, 10, 0)]
#[case::inc_zero_x1(mem_zero_index_read_store(Cpu::INC_ZERO_X, 0x80, 0x0F, 0x7F), 0x80, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::inc_zero_x2(mem_zero_index_read_store(Cpu::INC_ZERO_X, 0x80, 0xFF, 0xFF), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_X, 0xFF)]
#[case::inc_abs1(mem_abs_read_store(Cpu::INC_ABSOLUTE, 0x1225, 0x7F), 0x80, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xFE, 10, 0)]
#[case::inc_abs2(mem_abs_read_store(Cpu::INC_ABSOLUTE, 0x12AA, 0xFF), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x12, 10, 0)]
#[case::inc_abs_x1(mem_abs_index_read_store(Cpu::INC_ABSOLUTE_X, 0x1225, 0xF, 0x7F), 0x80, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xAB, Cpu::REG_X, 0x0F)]
#[case::inc_abs_x2(mem_abs_index_read_store(Cpu::INC_ABSOLUTE_X, 0x12AA, 0xBB, 0xFF), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x11, Cpu::REG_X, 0xBB)]
// DEC
#[case::dec_zero1(mem_zero_read_store(Cpu::DEC_ZERO, 0xCA, 0x0), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, 10, 0)]
#[case::dec_zero2(mem_zero_read_store(Cpu::DEC_ZERO, 0xCB, 0x1), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, 10, 0)]
#[case::dec_zero_x1(mem_zero_index_read_store(Cpu::DEC_ZERO_X, 0x80, 0x0F, 0), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::dec_zero_x2(mem_zero_index_read_store(Cpu::DEC_ZERO_X, 0x80, 0xFF, 0x1), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_X, 0xFF)]
#[case::dec_abs1(mem_abs_read_store(Cpu::DEC_ABSOLUTE, 0x1225, 0), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xFE, 10, 0)]
#[case::dec_abs2(mem_abs_read_store(Cpu::DEC_ABSOLUTE, 0x12AA, 0x1), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x12, 10, 0)]
#[case::dec_abs_x1(mem_abs_index_read_store(Cpu::DEC_ABSOLUTE_X, 0x1225, 0xF, 0), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xAB, Cpu::REG_X, 0x0F)]
#[case::dec_abs_x2(mem_abs_index_read_store(Cpu::DEC_ABSOLUTE_X, 0x12AA, 0xBB, 0x1), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x11, Cpu::REG_X, 0xBB)]
fn store_tests(
    #[case] mut op: Operation,
    #[case] expected_result: u8,
    #[case] expected_stat: u8,
    #[case] from_register: usize,
    #[case] register_init_val: u8,
    #[case] aux_register: usize,
    #[case] aux_register_init_val: u8,
) {
    let mut cpu = Cpu::new(&mut op.mem);
    cpu.reset();
    cpu.regs[from_register] = register_init_val;
    if aux_register <= Cpu::REG_STAT {
        cpu.regs[aux_register] = aux_register_init_val;
    }
    cpu.process(op.cycles);
    assert_eq!(RESET_EXEC_ADDRESS + op.bytes, cpu.pc);
    assert_eq!(expected_stat, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(op.cycles, cpu.cycles_run);
    assert_eq!(op.mem.read8(op.addr as usize), expected_result);
}

#[test]
fn test_push_a_to_sp() {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[Cpu::PUSH_A_TO_SP, Cpu::PUSH_A_TO_SP]);
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    cpu.regs[Cpu::REG_A] = 0xFE;
    cpu.regs[Cpu::REG_STAT] = Cpu::FLAG_ZERO | Cpu::FLAG_CARRY;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(STACK_OFFSET_START - 1, cpu.regs[Cpu::REG_SP]);
    assert_eq!(3, cpu.cycles_run);
    cpu.regs[Cpu::REG_A] = 0x12;
    cpu.regs[Cpu::REG_STAT] = 0;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(STACK_OFFSET_START - 2, cpu.regs[Cpu::REG_SP]);
    assert_eq!(6, cpu.cycles_run);
    assert_eq!(mem.read8(STACK_REAL_START), 0xFE);
    assert_eq!(mem.read8(STACK_REAL_START - 1), 0x12);
}

#[test]
fn test_push_stat_to_sp() {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[Cpu::PUSH_STAT_TO_SP, Cpu::PUSH_STAT_TO_SP]);
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    cpu.regs[Cpu::REG_STAT] = Cpu::FLAG_ZERO | Cpu::FLAG_CARRY;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(STACK_OFFSET_START - 1, cpu.regs[Cpu::REG_SP]);
    assert_eq!(3, cpu.cycles_run);
    cpu.regs[Cpu::REG_STAT] = Cpu::FLAG_INTERRUPT;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(Cpu::FLAG_INTERRUPT, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(STACK_OFFSET_START - 2, cpu.regs[Cpu::REG_SP]);
    assert_eq!(6, cpu.cycles_run);
    assert_eq!(mem.read8(STACK_REAL_START), Cpu::FLAG_ZERO | Cpu::FLAG_CARRY);
    assert_eq!(mem.read8(STACK_REAL_START - 1), Cpu::FLAG_INTERRUPT);
}

#[test]
fn test_pull_sp_to_a() {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[Cpu::PULL_SP_TO_A, Cpu::PULL_SP_TO_A]);
    mem.write8(STACK_REAL_START, 0xFE);
    mem.write8(STACK_REAL_START - 1, 0x12);
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    cpu.regs[Cpu::REG_SP] = STACK_OFFSET_START - 2;
    cpu.regs[Cpu::REG_STAT] = Cpu::FLAG_INTERRUPT;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(Cpu::FLAG_INTERRUPT, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(STACK_OFFSET_START - 1, cpu.regs[Cpu::REG_SP]);
    assert_eq!(0x12, cpu.regs[Cpu::REG_A]);
    assert_eq!(4, cpu.cycles_run);
    cpu.regs[Cpu::REG_A] = 0;
    cpu.regs[Cpu::REG_STAT] = 0;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(Cpu::FLAG_NEGATIVE, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(STACK_OFFSET_START, cpu.regs[Cpu::REG_SP]);
    assert_eq!(8, cpu.cycles_run);
    assert_eq!(0xFE, cpu.regs[Cpu::REG_A]);
}

#[test]
fn test_cpu_reset_vector() {
    let mut mem = Mem::new();
    mem.reset();
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    assert_eq!(RESET_EXEC_ADDRESS, cpu.pc)
}

#[test]
fn test_mem_read_limits_ok() {
    let mut mem = Mem::new();
    mem.reset();
    assert_eq!(0, mem.read8(0));
    assert_eq!(0, mem.read8(65535));
}

#[test]
#[should_panic(expected = "memory access out ouf bounds")]
fn test_mem_read_limits_nok() {
    let mut mem = Mem::new();
    mem.reset();
    assert_eq!(0, mem.read8(65536));
}

#[test]
#[should_panic(expected = "memory access out ouf bounds")]
fn test_mem_write_limits_nok() {
    let mut mem = Mem::new();
    mem.reset();
    mem.write8(65536, 1)
}

#[test]
fn test_mem_read_reset_vector_ok() {
    let mut mem = Mem::new();
    mem.reset();
    assert_eq!(0xFC, mem.read8(RESET_VECTOR_ADDR));
    assert_eq!(0xE2, mem.read8(RESET_VECTOR_ADDR + 1));
    assert_eq!(0, mem.read8(667));
}

#[test]
fn test_mem_write_read_ok() {
    let mut mem = Mem::new();
    mem.reset();
    mem.write8(666, 200);
    assert_eq!(200, mem.read8(666));
    assert_eq!(0, mem.read8(665));
    assert_eq!(0, mem.read8(667));
}
