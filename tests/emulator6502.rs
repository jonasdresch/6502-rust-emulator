use emulator6502::*;
use rstest::*;

struct Operation {
    cycles: u32,
    bytes: u16,
    mem: MEM,
    addr: u16, // used for store operations
}

#[fixture]
fn mem_imm(#[default = 0] instrunction: u8, #[default = 0] value: u8) -> Operation {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[instrunction, value]);
    Operation { cycles: 2, bytes: 2, mem, addr: 0 }
}

#[fixture]
fn mem_zero(#[default = 0] instrunction: u8, #[default = 0] addr: u8, #[default = 0] value: u8) -> Operation {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[instrunction, addr]);
    mem.write8(addr as usize, value);
    Operation { cycles: 3, bytes: 2, mem, addr: addr as u16 }
}

#[fixture]
fn mem_zero_index(#[default = 0] instrunction: u8, #[default = 0] addr: u8, #[default = 0] index: u8, #[default = 0] value: u8) -> Operation {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[instrunction, addr]);
    let real_addr = (addr as u16 + index as u16) as u8;
    mem.write8(real_addr as usize, value);
    Operation { cycles: 4, bytes: 2, mem, addr: real_addr as u16 }
}

#[fixture]
fn mem_abs(#[default = 0] instrunction: u8, #[default = 0] addr: u16, #[default = 0] value: u8) -> Operation {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[instrunction, addr as u8, ((addr & 0xFF00) >> 8) as u8]);
    mem.write8(addr as usize, value);
    Operation { cycles: 4, bytes: 3, mem, addr: addr as u16 }
}

#[fixture]
fn mem_abs_index(#[default = 0] instrunction: u8, #[default = 0] addr: u16, #[default = 0] index: u8, #[default = 0] value: u8) -> Operation {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[instrunction, addr as u8, ((addr & 0xFF00) >> 8) as u8]);
    let real_addr = addr + index as u16;
    mem.write8(real_addr as usize, value);
    let mut cycles = 4;
    if index as u16 + (addr as u8) as u16 > 255 {
        cycles = 5
    }
    Operation { cycles: cycles, bytes: 3, mem, addr: real_addr as u16 }
}

#[fixture]
fn mem_abs_index_store(#[default = 0] instrunction: u8, #[default = 0] addr: u16, #[default = 0] index: u8) -> Operation {
    let mut op = mem_abs_index(instrunction, addr, index, 0);
    op.cycles = 5;
    op
}

#[fixture]
fn mem_ind_x(#[default = 0] instrunction: u8, #[default = 0] ind_addr: u8, #[default = 0] addr: u16, #[default = 0] index: u8, #[default = 0] value: u8) -> Operation {
    let mut mem = MEM::new();
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
    let mut mem = MEM::new();
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
    Operation { cycles: cycles, bytes: 2, mem, addr: real_addr }
}

#[fixture]
fn mem_ind_y_store(#[default = 0] instrunction: u8, #[default = 0] ind_addr: u8, #[default = 0] addr: u16, #[default = 0] index: u8) -> Operation {
    let mut op = mem_ind_y(instrunction, ind_addr, addr, index, 0);
    op.cycles = 6;
    op
}

#[fixture]
fn mem_trans(#[default = 0] instrunction: u8) -> Operation {
    let mut mem = MEM::new();
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
#[case::lda_imm1(mem_imm(CPU::LDA_IMMEDIATE, 0xCA), 0xCA, CPU::FLAG_NEGATIVE, CPU::REG_A, 0, 10, 0)]
#[case::lda_imm2(mem_imm(CPU::LDA_IMMEDIATE, 0x0), 0x0, CPU::FLAG_ZERO, CPU::REG_A, 0, 10, 0)]
#[case::lda_zero1(mem_zero(CPU::LDA_ZERO, 0xCA, 0xFE), 0xFE, CPU::FLAG_NEGATIVE, CPU::REG_A, 0, 10, 0)]
#[case::lda_zero2(mem_zero(CPU::LDA_ZERO, 0, 0), 0x0, CPU::FLAG_ZERO, CPU::REG_A, 0, 10, 0)]
#[case::lda_zero_x1(mem_zero_index(CPU::LDA_ZERO_X, 0x80, 0x0F, 0xFE), 0xFE, CPU::FLAG_NEGATIVE, CPU::REG_A, 0, CPU::REG_X, 0x0F)]
#[case::lda_zero_x2(mem_zero_index(CPU::LDA_ZERO_X, 0x80, 0xFF, 0xA), 0xA, 0, CPU::REG_A, 0, CPU::REG_X, 0xFF)]
#[case::lda_abs1(mem_abs(CPU::LDA_ABSOLUTE, 0x1234, 0xAB), 0xAB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0, 10, 0)]
#[case::lda_abs_x1(mem_abs_index(CPU::LDA_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xAB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0, CPU::REG_X, 0x0F)]
#[case::lda_abs_x2(mem_abs_index(CPU::LDA_ABSOLUTE_X, 0x12AA, 0xBB, 0x11), 0x11, 0, CPU::REG_A, 0, CPU::REG_X, 0xBB)]
#[case::lda_abs_y1(mem_abs_index(CPU::LDA_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xAB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0, CPU::REG_Y, 0x0F)]
#[case::lda_abs_y2(mem_abs_index(CPU::LDA_ABSOLUTE_Y, 0x12AA, 0xBB, 0x11), 0x11, 0, CPU::REG_A, 0, CPU::REG_Y, 0xBB)]
#[case::lda_ind_x1(mem_ind_x(CPU::LDA_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0xAB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0, CPU::REG_X, 0x0F)]
#[case::lda_ind_x2(mem_ind_x(CPU::LDA_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x11), 0x11, 0, CPU::REG_A, 0, CPU::REG_X, 0xBB)]
#[case::lda_ind_y1(mem_ind_y(CPU::LDA_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0xAB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0, CPU::REG_Y, 0x0F)]
#[case::lda_ind_y2(mem_ind_y(CPU::LDA_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x11), 0x11, 0, CPU::REG_A, 0, CPU::REG_Y, 0xBB)]
#[case::ldx_imm1(mem_imm(CPU::LDX_IMMEDIATE, 0xCA), 0xCA, CPU::FLAG_NEGATIVE, CPU::REG_X, 0, 10, 0)]
#[case::ldx_imm2(mem_imm(CPU::LDX_IMMEDIATE, 0x0), 0x0, CPU::FLAG_ZERO, CPU::REG_X, 0, 10, 0)]
#[case::ldx_zero1(mem_zero(CPU::LDX_ZERO, 0xCA, 0xFE), 0xFE, CPU::FLAG_NEGATIVE, CPU::REG_X, 0, 10, 0)]
#[case::ldx_zero2(mem_zero(CPU::LDX_ZERO, 0xCB, 0), 0x0, CPU::FLAG_ZERO, CPU::REG_X, 0, 10, 0)]
#[case::ldx_zero_y1(mem_zero_index(CPU::LDX_ZERO_Y, 0x80, 0x0F, 0xFE), 0xFE, CPU::FLAG_NEGATIVE, CPU::REG_X, 0, CPU::REG_Y, 0x0F)]
#[case::ldx_zero_y2(mem_zero_index(CPU::LDX_ZERO_Y, 0x80, 0xFF, 0xA), 0xA, 0, CPU::REG_X, 0, CPU::REG_Y, 0xFF)]
#[case::ldx_abs1(mem_abs(CPU::LDX_ABSOLUTE, 0x1234, 0xAB), 0xAB, CPU::FLAG_NEGATIVE, CPU::REG_X, 0, 10, 0)]
#[case::ldx_abs_y1(mem_abs_index(CPU::LDX_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xAB, CPU::FLAG_NEGATIVE, CPU::REG_X, 0, CPU::REG_Y, 0x0F)]
#[case::ldx_abs_y2(mem_abs_index(CPU::LDX_ABSOLUTE_Y, 0x12AA, 0xBB, 0x11), 0x11, 0, CPU::REG_X, 0, CPU::REG_Y, 0xBB)]
#[case::ldy_imm1(mem_imm(CPU::LDY_IMMEDIATE, 0xCA), 0xCA, CPU::FLAG_NEGATIVE, CPU::REG_Y, 0, 10, 0)]
#[case::ldy_imm2(mem_imm(CPU::LDY_IMMEDIATE, 0x0), 0x0, CPU::FLAG_ZERO, CPU::REG_Y, 0, 10, 0)]
#[case::ldy_zero1(mem_zero(CPU::LDY_ZERO, 0xCA, 0xFE), 0xFE, CPU::FLAG_NEGATIVE, CPU::REG_Y, 0, 10, 0)]
#[case::ldy_zero2(mem_zero(CPU::LDY_ZERO, 0xCB, 0), 0x0, CPU::FLAG_ZERO, CPU::REG_Y, 0, 10, 0)]
#[case::ldy_zero_x1(mem_zero_index(CPU::LDY_ZERO_X, 0x80, 0x0F, 0xFE), 0xFE, CPU::FLAG_NEGATIVE, CPU::REG_Y, 0, CPU::REG_X, 0x0F)]
#[case::ldy_zero_x2(mem_zero_index(CPU::LDY_ZERO_X, 0x80, 0xFF, 0xA), 0xA, 0, CPU::REG_Y, 0, CPU::REG_X, 0xFF)]
#[case::ldy_abs1(mem_abs(CPU::LDY_ABSOLUTE, 0x1234, 0xAB), 0xAB, CPU::FLAG_NEGATIVE, CPU::REG_Y, 0, 10, 0)]
#[case::ldy_abs_x1(mem_abs_index(CPU::LDY_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xAB, CPU::FLAG_NEGATIVE, CPU::REG_Y, 0, CPU::REG_X, 0x0F)]
#[case::ldy_abs_x2(mem_abs_index(CPU::LDY_ABSOLUTE_X, 0x12AA, 0xBB, 0x11), 0x11, 0, CPU::REG_Y, 0, CPU::REG_X, 0xBB)]
#[case::trans_a_to_x1(mem_trans(CPU::TRANS_A_TO_X), 0xFE, CPU::FLAG_NEGATIVE, CPU::REG_X, 0, CPU::REG_A, 0xFE)]
#[case::trans_a_to_x2(mem_trans(CPU::TRANS_A_TO_X), 0, CPU::FLAG_ZERO, CPU::REG_X, 0, CPU::REG_A, 0)]
#[case::trans_a_to_y1(mem_trans(CPU::TRANS_A_TO_Y), 0xFE, CPU::FLAG_NEGATIVE, CPU::REG_Y, 0, CPU::REG_A, 0xFE)]
#[case::trans_a_to_y2(mem_trans(CPU::TRANS_A_TO_Y), 0, CPU::FLAG_ZERO, CPU::REG_Y, 0, CPU::REG_A, 0)]
#[case::trans_x_to_a1(mem_trans(CPU::TRANS_X_TO_A), 0xFE, CPU::FLAG_NEGATIVE, CPU::REG_A, 0, CPU::REG_X, 0xFE)]
#[case::trans_x_to_a2(mem_trans(CPU::TRANS_X_TO_A), 0, CPU::FLAG_ZERO, CPU::REG_A, 0, CPU::REG_X, 0)]
#[case::trans_y_to_a1(mem_trans(CPU::TRANS_Y_TO_A), 0xFE, CPU::FLAG_NEGATIVE, CPU::REG_A, 0, CPU::REG_Y, 0xFE)]
#[case::trans_y_to_a2(mem_trans(CPU::TRANS_Y_TO_A), 0, CPU::FLAG_ZERO, CPU::REG_A, 0, CPU::REG_Y, 0)]
#[case::trans_sp_to_x1(mem_trans(CPU::TRANS_SP_TO_X), 0xFE, CPU::FLAG_NEGATIVE, CPU::REG_X, 0, CPU::REG_SP, 0xFE)]
#[case::trans_sp_to_x2(mem_trans(CPU::TRANS_SP_TO_X), 0, CPU::FLAG_ZERO, CPU::REG_X, 0, CPU::REG_SP, 0)]
#[case::trans_x_to_sp(mem_trans(CPU::TRANS_X_TO_SP), 0xFE, 0, CPU::REG_SP, 0, CPU::REG_X, 0xFE)]
#[case::trans_x_to_sp(mem_trans(CPU::TRANS_X_TO_SP), 0, 0, CPU::REG_SP, 0, CPU::REG_X, 0)]
#[case::and_imm1(mem_imm(CPU::AND_IMMEDIATE, 0xCA), 0xA, 0, CPU::REG_A, 0xB, 10, 0)]
#[case::and_imm2(mem_imm(CPU::AND_IMMEDIATE, 0x12), 0x2, 0, CPU::REG_A, 0xB, 10, 0)]
#[case::and_zero1(mem_zero(CPU::AND_ZERO, 0x1, 0xCA), 0xA, 0, CPU::REG_A, 0xB, 10, 0)]
#[case::and_zero2(mem_zero(CPU::AND_ZERO, 0x2, 0x12), 0x2, 0, CPU::REG_A, 0xA, 10, 0)]
#[case::and_zero_x1(mem_zero_index(CPU::AND_ZERO_X, 0x80, 0x0F, 0xCA), 0xA, 0, CPU::REG_A, 0xB, CPU::REG_X, 0x0F)]
#[case::and_zero_x2(mem_zero_index(CPU::AND_ZERO_X, 0x80, 0xFF, 0x12), 0x2, 0, CPU::REG_A, 0xA, CPU::REG_X, 0xFF)]
#[case::and_abs1(mem_abs(CPU::AND_ABSOLUTE, 0x1234, 0xAB), 0xA1, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB1, 10, 0)]
#[case::and_abs_x1(mem_abs_index(CPU::AND_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xA1, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB1, CPU::REG_X, 0x0F)]
#[case::and_abs_x2(mem_abs_index(CPU::AND_ABSOLUTE_X, 0x12AA, 0xBB, 0x5E), 0, CPU::FLAG_ZERO, CPU::REG_A, 0xA1, CPU::REG_X, 0xBB)]
#[case::and_abs_y1(mem_abs_index(CPU::AND_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xA1, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB1, CPU::REG_Y, 0x0F)]
#[case::and_abs_y2(mem_abs_index(CPU::AND_ABSOLUTE_Y, 0x12AA, 0xBB, 0x5E), 0, CPU::FLAG_ZERO, CPU::REG_A, 0xA1, CPU::REG_Y, 0xBB)]
#[case::and_ind_x1(mem_ind_x(CPU::AND_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0xA1, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB1, CPU::REG_X, 0x0F)]
#[case::and_ind_x2(mem_ind_x(CPU::AND_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x5E), 0, CPU::FLAG_ZERO, CPU::REG_A, 0, CPU::REG_X, 0xBB)]
#[case::and_ind_y1(mem_ind_y(CPU::AND_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0xA1, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB1, CPU::REG_Y, 0x0F)]
#[case::and_ind_y2(mem_ind_y(CPU::AND_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x5E), 0, CPU::FLAG_ZERO, CPU::REG_A, 0, CPU::REG_Y, 0xBB)]
#[case::eor_imm1(mem_imm(CPU::EOR_IMMEDIATE, 0xCA), 0xC1, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB, 10, 0)]
#[case::eor_imm2(mem_imm(CPU::EOR_IMMEDIATE, 0x12), 0xD3, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xC1, 10, 0)]
#[case::eor_zero1(mem_zero(CPU::EOR_ZERO, 0x1, 0xCA), 0xC1, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB, 10, 0)]
#[case::eor_zero2(mem_zero(CPU::EOR_ZERO, 0x2, 0x12), 0xD3, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xC1, 10, 0)]
#[case::eor_zero_x1(mem_zero_index(CPU::EOR_ZERO_X, 0x80, 0x0F, 0xCA), 0xC1, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB, CPU::REG_X, 0x0F)]
#[case::eor_zero_x2(mem_zero_index(CPU::EOR_ZERO_X, 0x80, 0xFF, 0x12), 0xD3, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xC1, CPU::REG_X, 0xFF)]
#[case::eor_abs1(mem_abs(CPU::EOR_ABSOLUTE, 0x1234, 0xAB), 0x1A, 0, CPU::REG_A, 0xB1, 10, 0)]
#[case::eor_abs_x1(mem_abs_index(CPU::EOR_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0x1A, 0, CPU::REG_A, 0xB1, CPU::REG_X, 0x0F)]
#[case::eor_abs_x2(mem_abs_index(CPU::EOR_ABSOLUTE_X, 0x12AA, 0xBB, 0x5E), 0x44, 0, CPU::REG_A, 0x1A, CPU::REG_X, 0xBB)]
#[case::eor_abs_y1(mem_abs_index(CPU::EOR_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0x1A, 0, CPU::REG_A, 0xB1, CPU::REG_Y, 0x0F)]
#[case::eor_abs_y2(mem_abs_index(CPU::EOR_ABSOLUTE_Y, 0x12AA, 0xBB, 0x5E), 0x44, 0, CPU::REG_A, 0x1A, CPU::REG_Y, 0xBB)]
#[case::eor_ind_x1(mem_ind_x(CPU::EOR_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0x1A, 0, CPU::REG_A, 0xB1, CPU::REG_X, 0x0F)]
#[case::eor_ind_x2(mem_ind_x(CPU::EOR_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x5E), 0x44, 0, CPU::REG_A, 0x1A, CPU::REG_X, 0xBB)]
#[case::eor_ind_y1(mem_ind_y(CPU::EOR_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0x1A, 0, CPU::REG_A, 0xB1, CPU::REG_Y, 0x0F)]
#[case::eor_ind_y2(mem_ind_y(CPU::EOR_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x5E), 0x44, 0, CPU::REG_A, 0x1A, CPU::REG_Y, 0xBB)]
#[case::ora_imm1(mem_imm(CPU::ORA_IMMEDIATE, 0xCA), 0xCB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB, 10, 0)]
#[case::ora_imm2(mem_imm(CPU::ORA_IMMEDIATE, 0x12), 0xDB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xCB, 10, 0)]
#[case::ora_zero1(mem_zero(CPU::ORA_ZERO, 0x1, 0xCA), 0xCB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB, 10, 0)]
#[case::ora_zero2(mem_zero(CPU::ORA_ZERO, 0x2, 0x12), 0xDB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xCB, 10, 0)]
#[case::ora_zero_x1(mem_zero_index(CPU::ORA_ZERO_X, 0x80, 0x0F, 0xCA), 0xCB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB, CPU::REG_X, 0x0F)]
#[case::ora_zero_x2(mem_zero_index(CPU::ORA_ZERO_X, 0x80, 0xFF, 0x12), 0xDB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xCB, CPU::REG_X, 0xFF)]
#[case::ora_abs1(mem_abs(CPU::ORA_ABSOLUTE, 0x1234, 0xAB), 0xBB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB1, 10, 0)]
#[case::ora_abs_x1(mem_abs_index(CPU::ORA_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xBB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB1, CPU::REG_X, 0x0F)]
#[case::ora_abs_x2(mem_abs_index(CPU::ORA_ABSOLUTE_X, 0x12AA, 0xBB, 0x5E), 0xFF, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xBB, CPU::REG_X, 0xBB)]
#[case::ora_abs_y1(mem_abs_index(CPU::ORA_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xBB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB1, CPU::REG_Y, 0x0F)]
#[case::ora_abs_y2(mem_abs_index(CPU::ORA_ABSOLUTE_Y, 0x12AA, 0xBB, 0x5E), 0xFF, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xBB, CPU::REG_Y, 0xBB)]
#[case::ora_ind_x1(mem_ind_x(CPU::ORA_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0xBB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB1, CPU::REG_X, 0x0F)]
#[case::ora_ind_x2(mem_ind_x(CPU::ORA_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x5E), 0xFF, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xBB, CPU::REG_X, 0xBB)]
#[case::ora_ind_y1(mem_ind_y(CPU::ORA_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0xBB, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xB1, CPU::REG_Y, 0x0F)]
#[case::ora_ind_y2(mem_ind_y(CPU::ORA_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x5E), 0xFF, CPU::FLAG_NEGATIVE, CPU::REG_A, 0xBB, CPU::REG_Y, 0xBB)]
// TODO test input carry flag
// TODO test if status flags are not affected
#[case::adc1(mem_imm(CPU::ADC_IMMEDIATE, 0x66), 0xDD, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x77, CPU::REG_STAT, 0)]
#[case::adc2(mem_imm(CPU::ADC_IMMEDIATE, 0x8A), 0x67, CPU::FLAG_OVERFLOW | CPU::FLAG_CARRY, CPU::REG_A, 0xDD, CPU::REG_STAT, 0)]
#[case::adc_zero1(mem_zero(CPU::ADC_ZERO, 0x1, 0x66), 0xDD, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x77, CPU::REG_STAT, 0)]
#[case::adc_zero2(mem_zero(CPU::ADC_ZERO, 0x2, 0x8A), 0x67, CPU::FLAG_OVERFLOW | CPU::FLAG_CARRY, CPU::REG_A, 0xDD, CPU::REG_STAT, 0)]
#[case::adc_zero_x1(mem_zero_index(CPU::ADC_ZERO_X, 0x80, 0x0F, 0x66), 0xDD, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x77, CPU::REG_X, 0x0F)]
#[case::adc_zero_x2(mem_zero_index(CPU::ADC_ZERO_X, 0x80, 0xFF, 0x8A), 0x67, CPU::FLAG_OVERFLOW | CPU::FLAG_CARRY, CPU::REG_A, 0xDD, CPU::REG_X, 0xFF)]
#[case::adc_abs1(mem_abs(CPU::ADC_ABSOLUTE, 0x1234, 0x66), 0xDD, CPU::FLAG_OVERFLOW |CPU::FLAG_NEGATIVE, CPU::REG_A, 0x77, 10, 0)]
#[case::adc_abs_x1(mem_abs_index(CPU::ADC_ABSOLUTE_X, 0x1225, 0x0F, 0x66), 0xDD, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x77, CPU::REG_X, 0x0F)]
#[case::adc_abs_x2(mem_abs_index(CPU::ADC_ABSOLUTE_X, 0x12AA, 0xBB, 0x8A), 0x67, CPU::FLAG_OVERFLOW | CPU::FLAG_CARRY, CPU::REG_A, 0xDD, CPU::REG_X, 0xBB)]
#[case::adc_abs_y1(mem_abs_index(CPU::ADC_ABSOLUTE_Y, 0x1225, 0x0F, 0x66), 0xDD, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x77, CPU::REG_Y, 0x0F)]
#[case::adc_abs_y2(mem_abs_index(CPU::ADC_ABSOLUTE_Y, 0x12AA, 0xBB, 0x8A), 0x67, CPU::FLAG_OVERFLOW | CPU::FLAG_CARRY, CPU::REG_A, 0xDD, CPU::REG_Y, 0xBB)]
#[case::adc_ind_x1(mem_ind_x(CPU::ADC_INDIRECT_X, 0x25, 0x1234, 0x0F, 0x66), 0xDD, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x77, CPU::REG_X, 0x0F)]
#[case::adc_ind_x2(mem_ind_x(CPU::ADC_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x8A), 0x67, CPU::FLAG_OVERFLOW | CPU::FLAG_CARRY, CPU::REG_A, 0xDD, CPU::REG_X, 0xBB)]
#[case::adc_ind_y1(mem_ind_y(CPU::ADC_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0x66), 0xDD, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x77, CPU::REG_Y, 0x0F)]
#[case::adc_ind_y2(mem_ind_y(CPU::ADC_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x8A), 0x67, CPU::FLAG_OVERFLOW | CPU::FLAG_CARRY, CPU::REG_A, 0xDD, CPU::REG_Y, 0xBB)]
// Some tests use the index reg to set the STAT flag
#[case::sbc_imm1(mem_imm(CPU::SBC_IMMEDIATE, 0x66), 0x10, CPU::FLAG_CARRY, CPU::REG_A, 0x77, CPU::REG_STAT, 0)]
#[case::sbc_imm2(mem_imm(CPU::SBC_IMMEDIATE, 0x8A), 0x86, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x10, CPU::REG_STAT, CPU::FLAG_CARRY)]
#[case::sbc_zero1(mem_zero(CPU::SBC_ZERO, 0x1, 0x66), 0x10, CPU::FLAG_CARRY, CPU::REG_A, 0x77, CPU::REG_STAT, 0)]
#[case::sbc_zero2(mem_zero(CPU::SBC_ZERO, 0x2, 0x8A), 0x86, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x10, CPU::REG_STAT, CPU::FLAG_CARRY)]
#[case::sbc_zero_x1(mem_zero_index(CPU::SBC_ZERO_X, 0x80, 0x0F, 0x66), 0x10, CPU::FLAG_CARRY, CPU::REG_A, 0x77, CPU::REG_X, 0x0F)]
#[case::sbc_zero_x2(mem_zero_index(CPU::SBC_ZERO_X, 0x80, 0xFF, 0x8A), 0x85, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x10, CPU::REG_X, 0xFF)]
#[case::sbc_abs1(mem_abs(CPU::SBC_ABSOLUTE, 0x1234, 0x66), 0x10, CPU::FLAG_CARRY, CPU::REG_A, 0x77, 10, 0)]
#[case::sbc_abs_x1(mem_abs_index(CPU::SBC_ABSOLUTE_X, 0x1225, 0x0F, 0x66), 0x10, CPU::FLAG_CARRY, CPU::REG_A, 0x77, CPU::REG_X, 0x0F)]
#[case::sbc_abs_x2(mem_abs_index(CPU::SBC_ABSOLUTE_X, 0x12AA, 0xBB, 0x8A), 0x85, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x10, CPU::REG_X, 0xBB)]
#[case::sbc_abs_y1(mem_abs_index(CPU::SBC_ABSOLUTE_Y, 0x1225, 0x0F, 0x66), 0x10, CPU::FLAG_CARRY, CPU::REG_A, 0x77, CPU::REG_Y, 0x0F)]
#[case::sbc_abs_y2(mem_abs_index(CPU::SBC_ABSOLUTE_Y, 0x12AA, 0xBB, 0x8A), 0x85, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x10, CPU::REG_Y, 0xBB)]
#[case::sbc_ind_x1(mem_ind_x(CPU::SBC_INDIRECT_X, 0x25, 0x1234, 0x0F, 0x66), 0x10, CPU::FLAG_CARRY, CPU::REG_A, 0x77, CPU::REG_X, 0x0F)]
#[case::sbc_ind_x2(mem_ind_x(CPU::SBC_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x8A), 0x85, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x10, CPU::REG_X, 0xBB)]
#[case::sbc_ind_y1(mem_ind_y(CPU::SBC_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0x66), 0x10, CPU::FLAG_CARRY, CPU::REG_A, 0x77, CPU::REG_Y, 0x0F)]
#[case::sbc_ind_y2(mem_ind_y(CPU::SBC_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x8A), 0x85, CPU::FLAG_OVERFLOW | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x10, CPU::REG_Y, 0xBB)]
#[case::bit_test_zero1(mem_zero(CPU::BIT_TEST_ZERO, 0x1, 0xCA), 0xB, CPU::FLAG_NEGATIVE | CPU::FLAG_OVERFLOW, CPU::REG_A, 0xB, 10, 0)]
#[case::bit_test_zero2(mem_zero(CPU::BIT_TEST_ZERO, 0x2, 0x10), 0xB, CPU::FLAG_ZERO, CPU::REG_A, 0xB, 10, 0)]
#[case::bit_test_abs1(mem_abs(CPU::BIT_TEST_ABSOLUTE, 0x1234, 0xAB), 0x54, CPU::FLAG_ZERO | CPU::FLAG_NEGATIVE, CPU::REG_A, 0x54, 10, 0)]
fn load_tests(
    #[case] mut op: Operation,
    #[case] expected_result: u8,
    #[case] expected_stat: u8,
    #[case] to_register: usize,
    #[case] register_init_val: u8,
    #[case] index_register: usize,
    #[case] index_register_init_val: u8,
) {
    let mut cpu = CPU::new(&mut op.mem);
    cpu.reset();
    cpu.regs[to_register] = register_init_val;
    if index_register <= CPU::REG_STAT {
        cpu.regs[index_register] = index_register_init_val;
    }
    cpu.process(op.cycles);
    assert_eq!(RESET_EXEC_ADDRESS + op.bytes, cpu.pc);
    assert_eq!(expected_result, cpu.regs[to_register]);
    assert_eq!(expected_stat, cpu.regs[CPU::REG_STAT]);
    assert_eq!(op.cycles, cpu.cycles_run);
}

#[rstest]
#[case::sta_zero1(mem_zero(CPU::STA_ZERO, 0xCA, 0), 0xFE, 0, CPU::REG_A, 0xFE, 10, 0)]
#[case::sta_zero2(mem_zero(CPU::STA_ZERO, 0xCB, 0), 0x12, 0, CPU::REG_A, 0x12, 10, 0)]
#[case::sta_zero_x1(mem_zero_index(CPU::STA_ZERO_X, 0x80, 0x0F, 0), 0xFE, 0, CPU::REG_A, 0xFE, CPU::REG_X, 0x0F)]
#[case::sta_zero_x2(mem_zero_index(CPU::STA_ZERO_X, 0x80, 0xFF, 0), 0xA, 0, CPU::REG_A, 0xA, CPU::REG_X, 0xFF)]
#[case::sta_abs1(mem_abs(CPU::STA_ABSOLUTE, 0x1225, 0), 0xFE, 0, CPU::REG_A, 0xFE, 10, 0)]
#[case::sta_abs2(mem_abs(CPU::STA_ABSOLUTE, 0x12AA, 0), 0x12, 0, CPU::REG_A, 0x12, 10, 0)]
#[case::sta_abs_x1(mem_abs_index_store(CPU::STA_ABSOLUTE_X, 0x1225, 0x0F), 0xAB, 0, CPU::REG_A, 0xAB, CPU::REG_X, 0x0F)]
#[case::sta_abs_x2(mem_abs_index_store(CPU::STA_ABSOLUTE_X, 0x12AA, 0xBB), 0x11, 0, CPU::REG_A, 0x11, CPU::REG_X, 0xBB)]
#[case::sta_abs_y1(mem_abs_index_store(CPU::STA_ABSOLUTE_Y, 0x1225, 0x0F), 0xAB, 0, CPU::REG_A, 0xAB, CPU::REG_Y, 0x0F)]
#[case::sta_abs_y2(mem_abs_index_store(CPU::STA_ABSOLUTE_Y, 0x12AA, 0xBB), 0x11, 0, CPU::REG_A, 0x11, CPU::REG_Y, 0xBB)]
#[case::sta_ind_x1(mem_ind_x_store(CPU::STA_INDIRECT_X, 0x25, 0x1234, 0x0F), 0xAB, 0, CPU::REG_A, 0xAB, CPU::REG_X, 0x0F)]
#[case::sta_ind_x2(mem_ind_x_store(CPU::STA_INDIRECT_X, 0xAA, 0x1365, 0xBB), 0x11, 0, CPU::REG_A, 0x11, CPU::REG_X, 0xBB)]
#[case::sta_ind_y1(mem_ind_y_store(CPU::STA_INDIRECT_Y, 0x25, 0x1225, 0x0F), 0xAB, 0, CPU::REG_A, 0xAB, CPU::REG_Y, 0x0F)]
#[case::sta_ind_y2(mem_ind_y_store(CPU::STA_INDIRECT_Y, 0xAA, 0x12AA, 0xBB), 0x11, 0, CPU::REG_A, 0x11, CPU::REG_Y, 0xBB)]
#[case::stx_zero1(mem_zero(CPU::STX_ZERO, 0xCA, 0), 0xFE, 0, CPU::REG_X, 0xFE, 10, 0)]
#[case::stx_zero2(mem_zero(CPU::STX_ZERO, 0xCB, 0), 0x12, 0, CPU::REG_X, 0x12, 10, 0)]
#[case::stx_zero_y1(mem_zero_index(CPU::STX_ZERO_Y, 0x80, 0x0F, 0), 0xFE, 0, CPU::REG_X, 0xFE, CPU::REG_Y, 0x0F)]
#[case::stx_zero_y2(mem_zero_index(CPU::STX_ZERO_Y, 0x80, 0xFF, 0), 0xA, 0, CPU::REG_X, 0xA, CPU::REG_Y, 0xFF)]
#[case::stx_abs1(mem_abs(CPU::STX_ABSOLUTE, 0x1225, 0), 0xFE, 0, CPU::REG_X, 0xFE, 10, 0)]
#[case::stx_abs2(mem_abs(CPU::STX_ABSOLUTE, 0x12AA, 0), 0x12, 0, CPU::REG_X, 0x12, 10, 0)]
#[case::stx_zero1(mem_zero(CPU::STX_ZERO, 0xCA, 0), 0xFE, 0, CPU::REG_X, 0xFE, 10, 0)]
#[case::stx_zero2(mem_zero(CPU::STX_ZERO, 0xCB, 0), 0x12, 0, CPU::REG_X, 0x12, 10, 0)]
#[case::stx_zero_y1(mem_zero_index(CPU::STX_ZERO_Y, 0x80, 0x0F, 0), 0xFE, 0, CPU::REG_X, 0xFE, CPU::REG_Y, 0x0F)]
#[case::stx_zero_y2(mem_zero_index(CPU::STX_ZERO_Y, 0x80, 0xFF, 0), 0xA, 0, CPU::REG_X, 0xA, CPU::REG_Y, 0xFF)]
#[case::stx_abs1(mem_abs(CPU::STX_ABSOLUTE, 0x1225, 0), 0xFE, 0, CPU::REG_X, 0xFE, 10, 0)]
#[case::stx_abs2(mem_abs(CPU::STX_ABSOLUTE, 0x12AA, 0), 0x12, 0, CPU::REG_X, 0x12, 10, 0)]
fn store_tests(
    #[case] mut op: Operation,
    #[case] expected_result: u8,
    #[case] expected_stat: u8,
    #[case] from_register: usize,
    #[case] register_init_val: u8,
    #[case] index_register: usize,
    #[case] index_register_init_val: u8,
) {
    let mut cpu = CPU::new(&mut op.mem);
    cpu.reset();
    cpu.regs[from_register] = register_init_val;
    if index_register <= CPU::REG_STAT {
        cpu.regs[index_register] = index_register_init_val;
    }
    cpu.process(op.cycles);
    assert_eq!(RESET_EXEC_ADDRESS + op.bytes, cpu.pc);
    assert_eq!(expected_stat, cpu.regs[CPU::REG_STAT]);
    assert_eq!(op.cycles, cpu.cycles_run);
    assert_eq!(op.mem.read8(op.addr as usize), expected_result);
}

#[test]
fn test_push_a_to_sp() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::PUSH_A_TO_SP, CPU::PUSH_A_TO_SP]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.regs[CPU::REG_A] = 0xFE;
    cpu.regs[CPU::REG_STAT] = CPU::FLAG_ZERO | CPU::FLAG_CARRY;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(CPU::FLAG_ZERO | CPU::FLAG_CARRY, cpu.regs[CPU::REG_STAT]);
    assert_eq!(STACK_OFFSET_START - 1, cpu.regs[CPU::REG_SP]);
    assert_eq!(3, cpu.cycles_run);
    cpu.regs[CPU::REG_A] = 0x12;
    cpu.regs[CPU::REG_STAT] = 0;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0, cpu.regs[CPU::REG_STAT]);
    assert_eq!(STACK_OFFSET_START - 2, cpu.regs[CPU::REG_SP]);
    assert_eq!(6, cpu.cycles_run);
    assert_eq!(mem.read8(STACK_REAL_START), 0xFE);
    assert_eq!(mem.read8(STACK_REAL_START - 1), 0x12);
}

#[test]
fn test_push_stat_to_sp() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::PUSH_STAT_TO_SP, CPU::PUSH_STAT_TO_SP]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.regs[CPU::REG_STAT] = CPU::FLAG_ZERO | CPU::FLAG_CARRY;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(CPU::FLAG_ZERO | CPU::FLAG_CARRY, cpu.regs[CPU::REG_STAT]);
    assert_eq!(STACK_OFFSET_START - 1, cpu.regs[CPU::REG_SP]);
    assert_eq!(3, cpu.cycles_run);
    cpu.regs[CPU::REG_STAT] = CPU::FLAG_INTERRUPT;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(CPU::FLAG_INTERRUPT, cpu.regs[CPU::REG_STAT]);
    assert_eq!(STACK_OFFSET_START - 2, cpu.regs[CPU::REG_SP]);
    assert_eq!(6, cpu.cycles_run);
    assert_eq!(mem.read8(STACK_REAL_START), CPU::FLAG_ZERO | CPU::FLAG_CARRY);
    assert_eq!(mem.read8(STACK_REAL_START - 1), CPU::FLAG_INTERRUPT);
}

#[test]
fn test_pull_sp_to_a() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::PULL_SP_TO_A, CPU::PULL_SP_TO_A]);
    mem.write8(STACK_REAL_START, 0xFE);
    mem.write8(STACK_REAL_START - 1, 0x12);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.regs[CPU::REG_SP] = STACK_OFFSET_START - 2;
    cpu.regs[CPU::REG_STAT] = CPU::FLAG_INTERRUPT;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(CPU::FLAG_INTERRUPT, cpu.regs[CPU::REG_STAT]);
    assert_eq!(STACK_OFFSET_START - 1, cpu.regs[CPU::REG_SP]);
    assert_eq!(0x12, cpu.regs[CPU::REG_A]);
    assert_eq!(4, cpu.cycles_run);
    cpu.regs[CPU::REG_A] = 0;
    cpu.regs[CPU::REG_STAT] = 0;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.regs[CPU::REG_STAT]);
    assert_eq!(STACK_OFFSET_START, cpu.regs[CPU::REG_SP]);
    assert_eq!(8, cpu.cycles_run);
    assert_eq!(0xFE, cpu.regs[CPU::REG_A]);
}

#[test]
fn test_cpu_reset_vector() {
    let mut mem = MEM::new();
    mem.reset();
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    assert_eq!(RESET_EXEC_ADDRESS, cpu.pc)
}

#[test]
fn test_mem_read_limits_ok() {
    let mut mem = MEM::new();
    mem.reset();
    assert_eq!(0, mem.read8(0));
    assert_eq!(0, mem.read8(65535));
}

#[test]
#[should_panic(expected = "memory access out ouf bounds")]
fn test_mem_read_limits_nok() {
    let mut mem = MEM::new();
    mem.reset();
    assert_eq!(0, mem.read8(65536));
}

#[test]
#[should_panic(expected = "memory access out ouf bounds")]
fn test_mem_write_limits_nok() {
    let mut mem = MEM::new();
    mem.reset();
    mem.write8(65536, 1)
}

#[test]
fn test_mem_read_reset_vector_ok() {
    let mut mem = MEM::new();
    mem.reset();
    assert_eq!(0xFC, mem.read8(RESET_VECTOR_ADDR));
    assert_eq!(0xE2, mem.read8(RESET_VECTOR_ADDR + 1));
    assert_eq!(0, mem.read8(667));
}

#[test]
fn test_mem_write_read_ok() {
    let mut mem = MEM::new();
    mem.reset();
    mem.write8(666, 200);
    assert_eq!(200, mem.read8(666));
    assert_eq!(0, mem.read8(665));
    assert_eq!(0, mem.read8(667));
}
