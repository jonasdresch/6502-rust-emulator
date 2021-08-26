use emulator6502::*;
use rstest::*;

mod fixtures;
use fixtures::*;

const NOT_NEG_ZERO_FLAG: u8 = Cpu::FLAG_CARRY | Cpu::FLAG_OVERFLOW | Cpu::FLAG_BREAK | Cpu::FLAG_DECIMAL | Cpu::FLAG_INTERRUPT;
const ALL_FLAGS: u8 = Cpu::FLAG_CARRY | Cpu::FLAG_NEGATIVE | Cpu::FLAG_ZERO | Cpu::FLAG_OVERFLOW | Cpu::FLAG_BREAK | Cpu::FLAG_DECIMAL | Cpu::FLAG_INTERRUPT;

// TODO for all
// test if status flags are not zeroed
// test if status are not affected at all for specific results
#[rstest]
// LDA
#[case::lda_imm_only_negative_flag(mem_imm(Cpu::LDA_IMMEDIATE, 0xCA), 0xCA, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, 10, 0)]
#[case::lda_imm_only_zero_flag(mem_imm(Cpu::LDA_IMMEDIATE, 0x0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x1, 10, 0)]
#[case::lda_imm_dont_change_other_flags(mem_imm(Cpu::LDA_IMMEDIATE, 0xB), 0xB, NOT_NEG_ZERO_FLAG, Cpu::REG_A, 0, Cpu::REG_STAT, NOT_NEG_ZERO_FLAG)]
#[case::lda_zero_only_negative_flag(mem_zero(Cpu::LDA_ZERO, 0xCA, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, 10, 0)]
#[case::lda_zero_only_zero_flag(mem_zero(Cpu::LDA_ZERO, 0, 0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x1, 10, 0)]
#[case::lda_zero_dont_change_other_flags(mem_zero(Cpu::LDA_ZERO, 0, 0xB), 0xB, NOT_NEG_ZERO_FLAG, Cpu::REG_A, 0, Cpu::REG_STAT, NOT_NEG_ZERO_FLAG)]
#[case::lda_zero_x_only_negative_flag(mem_zero_index(Cpu::LDA_ZERO_X, 0x80, 0x0F, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::lda_zero_x_only_zero_flag(mem_zero_index(Cpu::LDA_ZERO_X, 0x80, 0xFF, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x1, Cpu::REG_X, 0xFF)]
#[case::lda_abs_only_negative_flag(mem_abs(Cpu::LDA_ABSOLUTE, 0x1234, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, 10, 0)]
#[case::lda_abs_only_zero_flag(mem_abs(Cpu::LDA_ABSOLUTE, 0x1234, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x1, 10, 0)]
#[case::lda_abs_dont_change_other_flags(mem_abs(Cpu::LDA_ABSOLUTE, 0x1234, 0xB), 0xB, NOT_NEG_ZERO_FLAG, Cpu::REG_A, 0, Cpu::REG_STAT, NOT_NEG_ZERO_FLAG)]
#[case::lda_abs_x_only_negative_flag(mem_abs_index(Cpu::LDA_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::lda_abs_x_only_zero_flag(mem_abs_index(Cpu::LDA_ABSOLUTE_X, 0x12AA, 0xBB, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x1, Cpu::REG_X, 0xBB)]
#[case::lda_abs_y_only_negative_flag(mem_abs_index(Cpu::LDA_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_Y, 0x0F)]
#[case::lda_abs_y_only_zero_flag(mem_abs_index(Cpu::LDA_ABSOLUTE_Y, 0x12AA, 0xBB, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x1, Cpu::REG_Y, 0xBB)]
#[case::lda_ind_x_only_negative_flag(mem_ind_x(Cpu::LDA_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::lda_ind_x_only_zero_flag(mem_ind_x(Cpu::LDA_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x1, Cpu::REG_X, 0xBB)]
#[case::lda_ind_y_only_negative_flag(mem_ind_y(Cpu::LDA_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_Y, 0x0F)]
#[case::lda_ind_y_only_zero_flag(mem_ind_y(Cpu::LDA_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x1, Cpu::REG_Y, 0xBB)]
// LDX
#[case::ldx_imm_only_negative_flag(mem_imm(Cpu::LDX_IMMEDIATE, 0xCA), 0xCA, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, 10, 0)]
#[case::ldx_imm_only_zero_flag(mem_imm(Cpu::LDX_IMMEDIATE, 0x0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_X, 0x1, 10, 0)]
#[case::ldx_imm_dont_change_other_flags(mem_imm(Cpu::LDX_IMMEDIATE, 0xB), 0xB, NOT_NEG_ZERO_FLAG, Cpu::REG_X, 0, Cpu::REG_STAT, NOT_NEG_ZERO_FLAG)]
#[case::ldx_zero_only_negative_flag(mem_zero(Cpu::LDX_ZERO, 0xCA, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, 10, 0)]
#[case::ldx_zero_only_zero_flag(mem_zero(Cpu::LDX_ZERO, 0xCB, 0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_X, 0x1, 10, 0)]
#[case::ldx_zero_dont_change_other_flags(mem_zero(Cpu::LDX_ZERO, 0xCB, 0xB), 0xB, NOT_NEG_ZERO_FLAG, Cpu::REG_X, 0, Cpu::REG_STAT, NOT_NEG_ZERO_FLAG)]
#[case::ldx_zero_y_only_negative_flag(mem_zero_index(Cpu::LDX_ZERO_Y, 0x80, 0x0F, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, Cpu::REG_Y, 0x0F)]
#[case::ldx_zero_y_only_zero_flag(mem_zero_index(Cpu::LDX_ZERO_Y, 0x80, 0xFF, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_X, 0x1, Cpu::REG_Y, 0xFF)]
#[case::ldx_abs_only_negative_flag(mem_abs(Cpu::LDX_ABSOLUTE, 0x1234, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, 10, 0)]
#[case::ldx_abs_only_zero_flag(mem_abs(Cpu::LDX_ABSOLUTE, 0x1234, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_X, 0x1, 10, 0)]
#[case::ldx_abs_dont_change_other_flags(mem_abs(Cpu::LDX_ABSOLUTE, 0x1234, 0xB), 0xB, NOT_NEG_ZERO_FLAG, Cpu::REG_X, 0, Cpu::REG_STAT, NOT_NEG_ZERO_FLAG)]
#[case::ldx_abs_y_only_negative_flag(mem_abs_index(Cpu::LDX_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, Cpu::REG_Y, 0x0F)]
#[case::ldx_abs_y_only_zero_flag(mem_abs_index(Cpu::LDX_ABSOLUTE_Y, 0x12AA, 0xBB, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_X, 0x1, Cpu::REG_Y, 0xBB)]
// LDY
#[case::ldy_imm_only_negative_flag(mem_imm(Cpu::LDY_IMMEDIATE, 0xCA), 0xCA, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, 10, 0)]
#[case::ldy_imm_only_zero_flag(mem_imm(Cpu::LDY_IMMEDIATE, 0x0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0x1, 10, 0)]
#[case::ldy_imm_dont_change_other_flags(mem_imm(Cpu::LDY_IMMEDIATE, 0xB), 0xB, NOT_NEG_ZERO_FLAG, Cpu::REG_Y, 0, Cpu::REG_STAT, NOT_NEG_ZERO_FLAG)]
#[case::ldy_zero_only_negative_flag(mem_zero(Cpu::LDY_ZERO, 0xCA, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, 10, 0)]
#[case::ldy_zero_only_zero_flag(mem_zero(Cpu::LDY_ZERO, 0xCB, 0), 0x0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0x1, 10, 0)]
#[case::ldy_zero_dont_change_other_flags(mem_zero(Cpu::LDY_ZERO, 0xCB, 0xB), 0xB, NOT_NEG_ZERO_FLAG, Cpu::REG_Y, 0, Cpu::REG_STAT, NOT_NEG_ZERO_FLAG)]
#[case::ldy_zero_y_only_negative_flag(mem_zero_index(Cpu::LDY_ZERO_X, 0x80, 0x0F, 0xFE), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, Cpu::REG_X, 0x0F)]
#[case::ldy_zero_y_only_zero_flag(mem_zero_index(Cpu::LDY_ZERO_X, 0x80, 0xFF, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0x1, Cpu::REG_X, 0xFF)]
#[case::ldy_abs_only_negative_flag(mem_abs(Cpu::LDY_ABSOLUTE, 0x1234, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, 10, 0)]
#[case::ldy_abs_only_zero_flag(mem_abs(Cpu::LDY_ABSOLUTE, 0x1234, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0x1, 10, 0)]
#[case::ldy_abs_dont_change_other_flags(mem_abs(Cpu::LDY_ABSOLUTE, 0x1234, 0xB), 0xB, NOT_NEG_ZERO_FLAG, Cpu::REG_Y, 0, Cpu::REG_STAT, NOT_NEG_ZERO_FLAG)]
#[case::ldy_abs_y_only_negative_flag(mem_abs_index(Cpu::LDY_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xAB, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, Cpu::REG_X, 0x0F)]
#[case::ldy_abs_y_only_zero_flag(mem_abs_index(Cpu::LDY_ABSOLUTE_X, 0x12AA, 0xBB, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0x1, Cpu::REG_X, 0xBB)]
// REG TRANSFERS
#[case::trans_a_to_x_only_negative_flag(mem_trans(Cpu::TRANS_A_TO_X), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, Cpu::REG_A, 0xFE)]
#[case::trans_a_to_x_only_zero_flag(mem_trans(Cpu::TRANS_A_TO_X), 0, Cpu::FLAG_ZERO, Cpu::REG_X, 0x1, Cpu::REG_A, 0)]
#[case::trans_a_to_y_only_negative_flag(mem_trans(Cpu::TRANS_A_TO_Y), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, Cpu::REG_A, 0xFE)]
#[case::trans_a_to_y_only_zero_flag(mem_trans(Cpu::TRANS_A_TO_Y), 0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0x1, Cpu::REG_A, 0)]
#[case::trans_x_to_a_only_negative_flag1(mem_trans(Cpu::TRANS_X_TO_A), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_X, 0xFE)]
#[case::trans_x_to_a_only_zero_flag(mem_trans(Cpu::TRANS_X_TO_A), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x1, Cpu::REG_X, 0)]
#[case::trans_y_to_a_only_negative_flag(mem_trans(Cpu::TRANS_Y_TO_A), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_Y, 0xFE)]
#[case::trans_y_to_a_only_zero_flag(mem_trans(Cpu::TRANS_Y_TO_A), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0x1, Cpu::REG_Y, 0)]
#[case::trans_sp_to_x_only_negative_flag(mem_trans(Cpu::TRANS_SP_TO_X), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, Cpu::REG_SP, 0xFE)]
#[case::trans_sp_to_x_only_zero_flag(mem_trans(Cpu::TRANS_SP_TO_X), 0, Cpu::FLAG_ZERO, Cpu::REG_X, 0x1, Cpu::REG_SP, 0)]
#[case::trans_x_to_sp_value_dont_set_flags(mem_trans(Cpu::TRANS_X_TO_SP), 0xFE, 0, Cpu::REG_SP, 0, Cpu::REG_X, 0xFE)]
#[case::trans_x_to_sp_zero_value_dont_set_flags(mem_trans(Cpu::TRANS_X_TO_SP), 0, 0, Cpu::REG_SP, 0x1, Cpu::REG_X, 0)]
// AND
#[case::and_imm_only_negative_flag(mem_imm(Cpu::AND_IMMEDIATE, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, 10, 0)]
#[case::and_imm_only_zero_flag(mem_imm(Cpu::AND_IMMEDIATE, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xA1, 10, 0)]
#[case::and_zero_only_negative_flag(mem_zero(Cpu::AND_ZERO, 0x1, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, 10, 0)]
#[case::and_zero_only_zero_flag(mem_zero(Cpu::AND_ZERO, 0x2, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xA1, 10, 0)]
#[case::and_zero_x_only_negative_flag(mem_zero_index(Cpu::AND_ZERO_X, 0x80, 0x0F, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::and_zero_x_only_zero_flag(mem_zero_index(Cpu::AND_ZERO_X, 0x80, 0xFF, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xA1, Cpu::REG_X, 0xFF)]
#[case::and_abs_only_negative_flag(mem_abs(Cpu::AND_ABSOLUTE, 0x1234, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, 10, 0)]
#[case::and_abs_only_zero_flag(mem_abs(Cpu::AND_ABSOLUTE, 0x1234, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xA1, 10, 0)]
#[case::and_abs_x_only_negative_flag(mem_abs_index(Cpu::AND_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::and_abs_x_only_zero_flag(mem_abs_index(Cpu::AND_ABSOLUTE_X, 0x12AA, 0xBB, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xA1, Cpu::REG_X, 0xBB)]
#[case::and_abs_y_only_negative_flag(mem_abs_index(Cpu::AND_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::and_abs_y_only_zero_flag(mem_abs_index(Cpu::AND_ABSOLUTE_Y, 0x12AA, 0xBB, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xA1, Cpu::REG_Y, 0xBB)]
#[case::and_ind_x_only_negative_flag(mem_ind_x(Cpu::AND_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::and_ind_x_only_zero_flag(mem_ind_x(Cpu::AND_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_X, 0xBB)]
#[case::and_ind_y_only_negative_flag(mem_ind_y(Cpu::AND_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0xA1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::and_ind_y_only_zero_flag(mem_ind_y(Cpu::AND_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x5E), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_Y, 0xBB)]
// EOR
#[case::eor_imm_only_negative_flag(mem_imm(Cpu::EOR_IMMEDIATE, 0xCA), 0xC1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, 10, 0)]
#[case::eor_imm_only_zero_flag(mem_imm(Cpu::EOR_IMMEDIATE, 0xCA), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xCA, 10, 0)]
#[case::eor_zero_only_negative_flag(mem_zero(Cpu::EOR_ZERO, 0x1, 0xCA), 0xC1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, 10, 0)]
#[case::eor_zero_only_zero_flag(mem_zero(Cpu::EOR_ZERO, 0x2, 0xCA), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xCA, 10, 0)]
#[case::eor_zero_x_only_negative_flag(mem_zero_index(Cpu::EOR_ZERO_X, 0x80, 0x0F, 0xCA), 0xC1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, Cpu::REG_X, 0x0F)]
#[case::eor_zero_x_only_zero_flag(mem_zero_index(Cpu::EOR_ZERO_X, 0x80, 0xFF, 0xCA), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xCA, Cpu::REG_X, 0xFF)]
#[case::eor_abs_only_negative_flag(mem_abs(Cpu::EOR_ABSOLUTE, 0x1234, 0xCA), 0xC1, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, 10, 0)]
#[case::eor_abs_only_zero_flag(mem_abs(Cpu::EOR_ABSOLUTE, 0x1234, 0xCA), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xCA, 10, 0)]
#[case::eor_abs_x_only_negative_flag(mem_abs_index(Cpu::EOR_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0x1A, 0, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::eor_abs_x_only_zero_flag(mem_abs_index(Cpu::EOR_ABSOLUTE_X, 0x12AA, 0xBB, 0xCA), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xCA, Cpu::REG_X, 0xBB)]
#[case::eor_abs_y_only_negative_flag(mem_abs_index(Cpu::EOR_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0x1A, 0, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::eor_abs_y_only_zero_flag(mem_abs_index(Cpu::EOR_ABSOLUTE_Y, 0x12AA, 0xBB, 0xCA), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xCA, Cpu::REG_Y, 0xBB)]
#[case::eor_ind_x_only_negative_flag1(mem_ind_x(Cpu::EOR_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0x1A, 0, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::eor_ind_x_only_zero_flag(mem_ind_x(Cpu::EOR_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0xCA), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xCA, Cpu::REG_X, 0xBB)]
#[case::eor_ind_y_only_negative_flag(mem_ind_y(Cpu::EOR_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0x1A, 0, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::eor_ind_y_only_zero_flag(mem_ind_y(Cpu::EOR_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0xCA), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0xCA, Cpu::REG_Y, 0xBB)]
// ORA
#[case::ora_imm_only_negative_flag(mem_imm(Cpu::ORA_IMMEDIATE, 0xCA), 0xCB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, 10, 0)]
#[case::ora_imm_only_zero_flag(mem_imm(Cpu::ORA_IMMEDIATE, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, 10, 0)]
#[case::ora_zero_only_negative_flag(mem_zero(Cpu::ORA_ZERO, 0x1, 0xCA), 0xCB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, 10, 0)]
#[case::ora_zero_only_zero_flag(mem_zero(Cpu::ORA_ZERO, 0x2, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, 10, 0)]
#[case::ora_zero_x_only_negative_flag(mem_zero_index(Cpu::ORA_ZERO_X, 0x80, 0x0F, 0xCA), 0xCB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB, Cpu::REG_X, 0x0F)]
#[case::ora_zero_x_only_zero_flag(mem_zero_index(Cpu::ORA_ZERO_X, 0x80, 0xFF, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_X, 0xFF)]
#[case::ora_abs_only_negative_flag(mem_abs(Cpu::ORA_ABSOLUTE, 0x1234, 0xAB), 0xBB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, 10, 0)]
#[case::ora_abs_only_zero_flag(mem_abs(Cpu::ORA_ABSOLUTE, 0x1234, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, 10, 0)]
#[case::ora_abs_x_only_negative_flag(mem_abs_index(Cpu::ORA_ABSOLUTE_X, 0x1225, 0x0F, 0xAB), 0xBB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::ora_abs_x_only_zero_flag(mem_abs_index(Cpu::ORA_ABSOLUTE_X, 0x12AA, 0xBB, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_X, 0xBB)]
#[case::ora_abs_y_only_negative_flag(mem_abs_index(Cpu::ORA_ABSOLUTE_Y, 0x1225, 0x0F, 0xAB), 0xBB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::ora_abs_y_only_zero_flag(mem_abs_index(Cpu::ORA_ABSOLUTE_Y, 0x12AA, 0xBB, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_Y, 0xBB)]
#[case::ora_ind_x_only_negative_flag(mem_ind_x(Cpu::ORA_INDIRECT_X, 0x25, 0x1234, 0x0F, 0xAB), 0xBB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_X, 0x0F)]
#[case::ora_ind_x_only_zero_flag(mem_ind_x(Cpu::ORA_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_X, 0xBB)]
#[case::ora_ind_y_only_negative_flag(mem_ind_y(Cpu::ORA_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0xAB), 0xBB, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0xB1, Cpu::REG_Y, 0x0F)]
#[case::ora_ind_y_only_zero_flag(mem_ind_y(Cpu::ORA_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, Cpu::REG_Y, 0xBB)]
// ADC
// TODO test input carry flag
// NOTE: before adding we must clear the CARRY with CLC: https://retro64.altervista.org/blog/an-introduction-to-6502-math-addiction-subtraction-and-more/
#[case::adc_overflow_and_negative(mem_imm(Cpu::ADC_IMMEDIATE, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_STAT, 0)]
#[case::adc_overflow_and_carry(mem_imm(Cpu::ADC_IMMEDIATE, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_STAT, 0)]
#[case::adc_zero_overflow_and_negative(mem_zero(Cpu::ADC_ZERO, 0x1, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_STAT, 0)]
#[case::adc_zero_overflow_and_carry(mem_zero(Cpu::ADC_ZERO, 0x2, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_STAT, 0)]
#[case::adc_zero_x_overflow_and_negative(mem_zero_index(Cpu::ADC_ZERO_X, 0x80, 0x0F, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::adc_zero_x_overflow_and_carry(mem_zero_index(Cpu::ADC_ZERO_X, 0x80, 0xFF, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_X, 0xFF)]
#[case::adc_abs_overflow_and_negative(mem_abs(Cpu::ADC_ABSOLUTE, 0x1234, 0x66), 0xDD, Cpu::FLAG_OVERFLOW |Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, 10, 0)]
#[case::adc_abs_x_overflow_and_negative(mem_abs_index(Cpu::ADC_ABSOLUTE_X, 0x1225, 0x0F, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::adc_abs_x_overflow_and_carry(mem_abs_index(Cpu::ADC_ABSOLUTE_X, 0x12AA, 0xBB, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_X, 0xBB)]
#[case::adc_abs_y_overflow_and_negative(mem_abs_index(Cpu::ADC_ABSOLUTE_Y, 0x1225, 0x0F, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_Y, 0x0F)]
#[case::adc_abs_y_overflow_and_carry(mem_abs_index(Cpu::ADC_ABSOLUTE_Y, 0x12AA, 0xBB, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_Y, 0xBB)]
#[case::adc_ind_x_overflow_and_negative(mem_ind_x(Cpu::ADC_INDIRECT_X, 0x25, 0x1234, 0x0F, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::adc_ind_x_overflow_and_carry(mem_ind_x(Cpu::ADC_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_X, 0xBB)]
#[case::adc_ind_y_overflow_and_negative(mem_ind_y(Cpu::ADC_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0x66), 0xDD, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x77, Cpu::REG_Y, 0x0F)]
#[case::adc_ind_y_overflow_and_carry(mem_ind_y(Cpu::ADC_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x8A), 0x67, Cpu::FLAG_OVERFLOW | Cpu::FLAG_CARRY, Cpu::REG_A, 0xDD, Cpu::REG_Y, 0xBB)]
// SBC
// There is not a “borrow flag” in the 6502 CPU. The carry flag is not a borrow either,
// but it acts as a reverse borrow. So, before performing a subtraction,
// we should clear the borrow, or, as it is done in practice, we must SET the carry.
// https://retro64.altervista.org/blog/an-introduction-to-6502-math-addiction-subtraction-and-more/
// http://forum.6502.org/viewtopic.php?t=18
#[case::sbc_imm_set_negative(mem_imm(Cpu::SBC_IMMEDIATE, 0x41), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x3F, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::sbc_imm_dont_set_any_flags(mem_imm(Cpu::SBC_IMMEDIATE, 0xFF), 0x02, 0, Cpu::REG_A, 0x1, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::sbc_zero_set_carry(mem_zero(Cpu::SBC_ZERO, 0x1, 0x66), 0x11, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::sbc_zero_overflow_and_negative(mem_zero(Cpu::SBC_ZERO, 0x2, 0x8A), 0x87, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x11, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
// When we do not set the carry flag we assume we are borrowing one and the result will be -1
#[case::sbc_zero_x_set_carry(mem_zero_index(Cpu::SBC_ZERO_X, 0x80, 0x0F, 0x66), 0x10, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::sbc_zero_x_overflow_and_negative(mem_zero_index(Cpu::SBC_ZERO_X, 0x80, 0xFF, 0x8A), 0x85, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x10, Cpu::REG_X, 0xFF)]
#[case::sbc_abs_set_carry(mem_abs(Cpu::SBC_ABSOLUTE, 0x1234, 0x66), 0x11, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::sbc_abs_x_set_carry(mem_abs_index(Cpu::SBC_ABSOLUTE_X, 0x1225, 0x0F, 0x66), 0x10, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::sbc_abs_x_overflow_and_negative(mem_abs_index(Cpu::SBC_ABSOLUTE_X, 0x12AA, 0xBB, 0x8A), 0x85, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x10, Cpu::REG_X, 0xBB)]
#[case::sbc_abs_y_set_carry(mem_abs_index(Cpu::SBC_ABSOLUTE_Y, 0x1225, 0x0F, 0x66), 0x10, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_Y, 0x0F)]
#[case::sbc_abs_y_overflow_and_negative(mem_abs_index(Cpu::SBC_ABSOLUTE_Y, 0x12AA, 0xBB, 0x8A), 0x85, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x10, Cpu::REG_Y, 0xBB)]
#[case::sbc_ind_x_set_carry(mem_ind_x(Cpu::SBC_INDIRECT_X, 0x25, 0x1234, 0x0F, 0x66), 0x10, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_X, 0x0F)]
#[case::sbc_ind_x_overflow_and_negative(mem_ind_x(Cpu::SBC_INDIRECT_X, 0xAA, 0x1365, 0xBB, 0x8A), 0x85, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x10, Cpu::REG_X, 0xBB)]
#[case::sbc_ind_y_set_carry(mem_ind_y(Cpu::SBC_INDIRECT_Y, 0x25, 0x1225, 0x0F, 0x66), 0x10, Cpu::FLAG_CARRY, Cpu::REG_A, 0x77, Cpu::REG_Y, 0x0F)]
#[case::sbc_ind_y_overflow_and_negative(mem_ind_y(Cpu::SBC_INDIRECT_Y, 0xAA, 0x12AA, 0xBB, 0x8A), 0x85, Cpu::FLAG_OVERFLOW | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x10, Cpu::REG_Y, 0xBB)]
// BIT test
#[case::bit_test_zero_overflow_and_negative(mem_zero(Cpu::BIT_TEST_ZERO, 0x1, 0xCA), 0xB, Cpu::FLAG_NEGATIVE | Cpu::FLAG_OVERFLOW, Cpu::REG_A, 0xB, 10, 0)]
#[case::bit_test_zero_set_zero(mem_zero(Cpu::BIT_TEST_ZERO, 0x2, 0x10), 0xB, Cpu::FLAG_ZERO, Cpu::REG_A, 0xB, 10, 0)]
#[case::bit_test_zero_and_negative(mem_abs(Cpu::BIT_TEST_ABSOLUTE, 0x1234, 0xAB), 0x54, Cpu::FLAG_ZERO | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x54, 10, 0)]
// CMP http://www.6502.org/tutorials/compare_beyond.html
#[case::cmp_imm_less_than_dont_set_flags(mem_imm(Cpu::CMP_IMMEDIATE, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_STAT, 0)]
#[case::cmp_imm_equal_set_carry_zero(mem_imm(Cpu::CMP_IMMEDIATE, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x1, Cpu::REG_STAT, 0)]
#[case::cmp_imm_bigger_than_set_carry(mem_imm(Cpu::CMP_IMMEDIATE, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_A, 0x2, Cpu::REG_STAT, 0)]
#[case::cmp_imm_set_negative_flag(mem_imm(Cpu::CMP_IMMEDIATE, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_STAT, 0)]
#[case::cmp_zero_less_than_dont_set_flags(mem_zero(Cpu::CMP_ZERO, 0x1, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_STAT, 0)]
#[case::cmp_zero_equal_set_carry_zero(mem_zero(Cpu::CMP_ZERO, 0x1, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x1, Cpu::REG_STAT, 0)]
#[case::cmp_zero_bigger_than_set_carry(mem_zero(Cpu::CMP_ZERO, 0x1, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_A, 0x2, Cpu::REG_STAT, 0)]
#[case::cmp_zero_set_negative_flag(mem_zero(Cpu::CMP_ZERO, 0x2, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_STAT, 0)]
#[case::cmp_zero_x_less_than_dont_set_flags(mem_zero_index(Cpu::CMP_ZERO_X, 0x70, 0xF, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_zero_x_equal_set_carry_zero(mem_zero_index(Cpu::CMP_ZERO_X, 0x70, 0xF, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_zero_x_bigger_than_set_carry(mem_zero_index(Cpu::CMP_ZERO_X, 0x70, 0xF, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_A, 0x2, Cpu::REG_X, 0xF)]
#[case::cmp_zero_x_set_negative_flag(mem_zero_index(Cpu::CMP_ZERO_X, 0x70, 0xFF, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_X, 0xFF)]
#[case::cmp_abs_less_than_dont_set_flags(mem_abs(Cpu::CMP_ABSOLUTE, 0x1234, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_abs_equal_set_carry_zero(mem_abs(Cpu::CMP_ABSOLUTE, 0x1234, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_abs_bigger_than_set_carry(mem_abs(Cpu::CMP_ABSOLUTE, 0x1234, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_A, 0x2, Cpu::REG_X, 0xF)]
#[case::cmp_abs_set_negative_flag(mem_abs(Cpu::CMP_ABSOLUTE, 0x1234, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_X, 0xFF)]
#[case::cmp_abs_x_less_than_dont_set_flags(mem_abs_index(Cpu::CMP_ABSOLUTE_X, 0x1234, 0xF, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_abs_x_equal_set_carry_zero(mem_abs_index(Cpu::CMP_ABSOLUTE_X, 0x1234, 0xF, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_abs_x_bigger_than_set_carry(mem_abs_index(Cpu::CMP_ABSOLUTE_X, 0x1234, 0xF, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_A, 0x2, Cpu::REG_X, 0xF)]
#[case::cmp_abs_x_set_negative_flag(mem_abs_index(Cpu::CMP_ABSOLUTE_X, 0x1234, 0xBB, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_X, 0xBB)]
#[case::cmp_abs_y_less_than_dont_set_flags(mem_abs_index(Cpu::CMP_ABSOLUTE_Y, 0x1234, 0xF, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_Y, 0xF)]
#[case::cmp_abs_y_equal_set_carry_zero(mem_abs_index(Cpu::CMP_ABSOLUTE_Y, 0x1234, 0xF, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x1, Cpu::REG_Y, 0xF)]
#[case::cmp_abs_y_bigger_than_set_carry(mem_abs_index(Cpu::CMP_ABSOLUTE_Y, 0x1234, 0xF, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_A, 0x2, Cpu::REG_Y, 0xF)]
#[case::cmp_abs_y_set_negative_flag(mem_abs_index(Cpu::CMP_ABSOLUTE_Y, 0x1234, 0xBB, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_Y, 0xBB)]
#[case::cmp_ind_x_less_than_dont_set_flags(mem_ind_x(Cpu::CMP_INDIRECT_X, 0x25, 0x1234, 0xF, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_ind_x_equal_set_carry_zero(mem_ind_x(Cpu::CMP_INDIRECT_X, 0x25, 0x1234, 0xF, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x1, Cpu::REG_X, 0xF)]
#[case::cmp_ind_x_bigger_than_set_carry(mem_ind_x(Cpu::CMP_INDIRECT_X, 0x25, 0x1234, 0xF, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_A, 0x2, Cpu::REG_X, 0xF)]
#[case::cmp_ind_x_set_negative_flag(mem_ind_x(Cpu::CMP_INDIRECT_X, 0xAA, 0x1234, 0xBB, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_X, 0xBB)]
#[case::cmp_ind_y_less_than_dont_set_flags(mem_ind_y(Cpu::CMP_INDIRECT_Y, 0x25, 0x1234, 0xF, 0xFF), 0x1, 0, Cpu::REG_A, 0x1, Cpu::REG_Y, 0xF)]
#[case::cmp_ind_y_equal_set_carry_zero(mem_ind_y(Cpu::CMP_INDIRECT_Y, 0x25, 0x1234, 0xF, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x1, Cpu::REG_Y, 0xF)]
#[case::cmp_ind_y_bigger_than_set_carry(mem_ind_y(Cpu::CMP_INDIRECT_Y, 0x25, 0x1234, 0xF, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_A, 0x2, Cpu::REG_Y, 0xF)]
#[case::cmp_ind_y_set_negative_flag(mem_ind_y(Cpu::CMP_INDIRECT_Y, 0xAA, 0x1234, 0xBB, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x7F, Cpu::REG_Y, 0xBB)]
// CPX
#[case::cpx_imm_less_than_dont_set_flags(mem_imm(Cpu::CPX_IMMEDIATE, 0xFF), 0x1, 0, Cpu::REG_X, 0x1, Cpu::REG_STAT, 0)]
#[case::cpx_imm_equal_set_carry_zero(mem_imm(Cpu::CPX_IMMEDIATE, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_X, 0x1, Cpu::REG_STAT, 0)]
#[case::cpx_imm_bigger_than_set_carry(mem_imm(Cpu::CPX_IMMEDIATE, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_X, 0x2, Cpu::REG_STAT, 0)]
#[case::cpx_imm_set_negative_flag(mem_imm(Cpu::CPX_IMMEDIATE, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0x7F, Cpu::REG_STAT, 0)]
#[case::cpx_zero_less_than_dont_set_flags(mem_zero(Cpu::CPX_ZERO, 0x1, 0xFF), 0x1, 0, Cpu::REG_X, 0x1, Cpu::REG_STAT, 0)]
#[case::cpx_zero_equal_set_carry_zero(mem_zero(Cpu::CPX_ZERO, 0x2, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_X, 0x1, Cpu::REG_STAT, 0)]
#[case::cpx_zero_bigger_than_set_carry(mem_zero(Cpu::CPX_ZERO, 0x2, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_X, 0x2, Cpu::REG_STAT, 0)]
#[case::cpx_zero_set_negative_flag(mem_zero(Cpu::CPX_ZERO, 0x2, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0x7F, Cpu::REG_STAT, 0)]
#[case::cpx_abs_less_than_dont_set_flags(mem_abs(Cpu::CPX_ABSOLUTE, 0x1234, 0xFF), 0x1, 0, Cpu::REG_X, 0x1, Cpu::REG_STAT, 0)]
#[case::cpx_abs_equal_set_carry_zero(mem_abs(Cpu::CPX_ABSOLUTE, 0x1234, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_X, 0x1, Cpu::REG_STAT, 0)]
#[case::cpx_abs_bigger_than_set_carry(mem_abs(Cpu::CPX_ABSOLUTE, 0x1234, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_X, 0x2, Cpu::REG_STAT, 0)]
#[case::cpx_abs_set_negative_flag(mem_abs(Cpu::CPX_ABSOLUTE, 0x1234, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0x7F, Cpu::REG_STAT, 0)]
// CPY
#[case::cpy_imm_less_than_dont_set_flags(mem_imm(Cpu::CPY_IMMEDIATE, 0xFF), 0x1, 0, Cpu::REG_Y, 0x1, Cpu::REG_STAT, 0)]
#[case::cpy_imm_equal_set_carry_zero(mem_imm(Cpu::CPY_IMMEDIATE, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_Y, 0x1, Cpu::REG_STAT, 0)]
#[case::cpy_imm_bigger_than_set_carry(mem_imm(Cpu::CPY_IMMEDIATE, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_Y, 0x2, Cpu::REG_STAT, 0)]
#[case::cpy_imm_set_negative_flag(mem_imm(Cpu::CPY_IMMEDIATE, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0x7F, Cpu::REG_STAT, 0)]
#[case::cpy_zero_less_than_dont_set_flags(mem_zero(Cpu::CPY_ZERO, 0x1, 0xFF), 0x1, 0, Cpu::REG_Y, 0x1, Cpu::REG_STAT, 0)]
#[case::cpy_zero_equal_set_carry_zero(mem_zero(Cpu::CPY_ZERO, 0x2, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_Y, 0x1, Cpu::REG_STAT, 0)]
#[case::cpy_zero_bigger_than_set_carry(mem_zero(Cpu::CPY_ZERO, 0x2, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_Y, 0x2, Cpu::REG_STAT, 0)]
#[case::cpy_zero_set_negative_flag(mem_zero(Cpu::CPY_ZERO, 0x2, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0x7F, Cpu::REG_STAT, 0)]
#[case::cpy_abs_less_than_dont_set_flags(mem_abs(Cpu::CPY_ABSOLUTE, 0x1234, 0xFF), 0x1, 0, Cpu::REG_Y, 0x1, Cpu::REG_STAT, 0)]
#[case::cpy_abs_equal_set_carry_zero(mem_abs(Cpu::CPY_ABSOLUTE, 0x1234, 0x1), 0x1, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_Y, 0x1, Cpu::REG_STAT, 0)]
#[case::cpy_abs_bigger_than_set_carry(mem_abs(Cpu::CPY_ABSOLUTE, 0x1234, 0x1), 0x2, Cpu::FLAG_CARRY, Cpu::REG_Y, 0x2, Cpu::REG_STAT, 0)]
#[case::cpy_abs_set_negative_flag(mem_abs(Cpu::CPY_ABSOLUTE, 0x1234, 0x80), 0x7F, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0x7F, Cpu::REG_STAT, 0)]
// INX
#[case::inx_wrap_around_and_set_zero(mem_implied(Cpu::INX_IMPLIED), 0, Cpu::FLAG_ZERO, Cpu::REG_X, 0xFF, 10, 0)]
#[case::inx_set_negative_flag(mem_implied(Cpu::INX_IMPLIED), 0x80, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0x7F, 10, 0)]
#[case::inx_dont_set_any_flags(mem_implied(Cpu::INX_IMPLIED), 0x2, 0, Cpu::REG_X, 0x1, 10, 0)]
// INY
#[case::iny_wrap_around_and_set_zero(mem_implied(Cpu::INY_IMPLIED), 0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0xFF, 10, 0)]
#[case::iny_set_negative_flag(mem_implied(Cpu::INY_IMPLIED), 0x80, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0x7F, 10, 0)]
#[case::iny_dont_set_any_flags(mem_implied(Cpu::INY_IMPLIED), 0x2, 0, Cpu::REG_Y, 0x1, 10, 0)]
// DEX
#[case::dex_wrap_around(mem_implied(Cpu::DEX_IMPLIED), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0, 10, 0)]
#[case::dex_set_negative_flag(mem_implied(Cpu::DEX_IMPLIED), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_X, 0xFF, 10, 0)]
#[case::dex_set_zero_flag(mem_implied(Cpu::DEX_IMPLIED), 0, Cpu::FLAG_ZERO, Cpu::REG_X, 0x1, 10, 0)]
#[case::dex_dont_set_any_flags(mem_implied(Cpu::DEX_IMPLIED), 0x1, 0, Cpu::REG_X, 0x2, 10, 0)]
// DEY
#[case::dey_wrap_around(mem_implied(Cpu::DEY_IMPLIED), 0xFF, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0, 10, 0)]
#[case::dey_set_negative_flag(mem_implied(Cpu::DEY_IMPLIED), 0xFE, Cpu::FLAG_NEGATIVE, Cpu::REG_Y, 0xFF, 10, 0)]
#[case::dey_set_zero_flag(mem_implied(Cpu::DEY_IMPLIED), 0, Cpu::FLAG_ZERO, Cpu::REG_Y, 0x1, 10, 0)]
#[case::dey_dont_set_any_flags(mem_implied(Cpu::DEY_IMPLIED), 0x1, 0, Cpu::REG_Y, 0x2, 10, 0)]
// ASL
#[case::asl_set_negative_and_carry(mem_implied(Cpu::ASL_IMPLIED), 0xFE, Cpu::FLAG_NEGATIVE | Cpu::FLAG_CARRY, Cpu::REG_A, 0xFF, 10, 0)]
#[case::asl_set_carry_and_zero(mem_implied(Cpu::ASL_IMPLIED), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x80, 10, 0)]
#[case::asl_dont_set_any_flags(mem_implied(Cpu::ASL_IMPLIED), 0x40, 0, Cpu::REG_A, 0x20, 10, 0)]
#[case::asl_carry_must_not_affect(mem_implied(Cpu::ASL_IMPLIED), 0x40, 0, Cpu::REG_A, 0x20, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
// LSR
#[case::lsr_set_carry_and_zero(mem_implied(Cpu::LSR_IMPLIED), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x1, 10, 0)]
#[case::lsr_set_zero(mem_implied(Cpu::LSR_IMPLIED), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, 10, 0)]
#[case::lsr_dont_set_any_flags(mem_implied(Cpu::LSR_IMPLIED), 0x1, 0, Cpu::REG_A, 0x2, 10, 0)]
// ROL
#[case::rol_set_negative(mem_implied(Cpu::ROL_IMPLIED), 0x80, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x40, 10, 0)]
#[case::rol_set_bit0_from_carry(mem_implied(Cpu::ROL_IMPLIED), 0x1, 0, Cpu::REG_A, 0, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::rol_set_bit0_from_carry_and_set_carry(mem_implied(Cpu::ROL_IMPLIED), 0x1, Cpu::FLAG_CARRY, Cpu::REG_A, 0x80, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::rol_set_carry_and_zero(mem_implied(Cpu::ROL_IMPLIED), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x80, 10, 0)]
#[case::rol_set_zero(mem_implied(Cpu::ROL_IMPLIED), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, 10, 0)]
#[case::rol_dont_set_any_flags(mem_implied(Cpu::ROL_IMPLIED), 0x40, 0, Cpu::REG_A, 0x20, 10, 0)]
// ROR
#[case::ror_set_bit7_from_carry(mem_implied(Cpu::ROR_IMPLIED), 0x80, Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::ror_set_bit7_from_carry_and_set_carry(mem_implied(Cpu::ROR_IMPLIED), 0x80, Cpu::FLAG_CARRY | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0x1, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::ror_set_carry_and_zero(mem_implied(Cpu::ROR_IMPLIED), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x1, 10, 0)]
#[case::ror_set_zero(mem_implied(Cpu::ROR_IMPLIED), 0, Cpu::FLAG_ZERO, Cpu::REG_A, 0, 10, 0)]
#[case::ror_dont_set_any_flags(mem_implied(Cpu::ROR_IMPLIED), 0x20, 0, Cpu::REG_A, 0x40, 10, 0)]
// CLC
#[case::clc_clear_only_carry(mem_implied(Cpu::CLC_IMPLIED), 0, ALL_FLAGS & !Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_STAT, ALL_FLAGS)]
// CLD
#[case::cld_clear_only_decimal(mem_implied(Cpu::CLD_IMPLIED), 0, ALL_FLAGS & !Cpu::FLAG_DECIMAL, Cpu::REG_A, 0, Cpu::REG_STAT, ALL_FLAGS)]
// CLI
#[case::cli_clear_only_interrupt(mem_implied(Cpu::CLI_IMPLIED), 0, ALL_FLAGS & !Cpu::FLAG_INTERRUPT, Cpu::REG_A, 0, Cpu::REG_STAT, ALL_FLAGS)]
// CLV
#[case::clv_clear_only_overflow(mem_implied(Cpu::CLV_IMPLIED), 0, ALL_FLAGS & !Cpu::FLAG_OVERFLOW, Cpu::REG_A, 0, Cpu::REG_STAT, ALL_FLAGS)]
// SEC
#[case::sec_set_only_carry(mem_implied(Cpu::SEC_IMPLIED), 0, Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_STAT, 0)]
// SED
#[case::sed_set_only_decimal(mem_implied(Cpu::SED_IMPLIED), 0, Cpu::FLAG_DECIMAL, Cpu::REG_A, 0, Cpu::REG_STAT, 0)]
// SEI
#[case::sei_set_only_interrupt(mem_implied(Cpu::SEI_IMPLIED), 0, Cpu::FLAG_INTERRUPT, Cpu::REG_A, 0, Cpu::REG_STAT, 0)]
// NOP
#[case::nop(mem_implied(Cpu::NOP_IMPLIED), 0, 0, Cpu::REG_A, 0, Cpu::REG_STAT, 0)]
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
    assert_eq!(RESET_EXEC_ADDRESS as i32 + op.bytes, cpu.pc as i32, "PC not expected");
    assert_eq!(expected_result, cpu.regs[to_register], "Result not expected");
    assert_eq!(expected_stat, cpu.regs[Cpu::REG_STAT], "Stat reg expected");
    assert_eq!(op.cycles, cpu.cycles_run, "Cycles run not expected");
}
