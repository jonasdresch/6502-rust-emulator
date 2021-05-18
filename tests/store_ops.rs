use emulator6502::*;
use rstest::*;

mod fixtures;
use fixtures::*;

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
// ASL
#[case::asl_zero1(mem_zero_read_store(Cpu::ASL_ZERO, 0xCA, 0xFF), 0xFE, Cpu::FLAG_NEGATIVE | Cpu::FLAG_CARRY, Cpu::REG_A, 0, 10, 0)]
#[case::asl_zero2(mem_zero_read_store(Cpu::ASL_ZERO, 0xCB, 0x80), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, 10, 0)]
#[case::asl_zero_x1(mem_zero_index_read_store(Cpu::ASL_ZERO_X, 0x80, 0x0F, 0xFF), 0xFE, Cpu::FLAG_NEGATIVE | Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::asl_zero_x2(mem_zero_index_read_store(Cpu::ASL_ZERO_X, 0x80, 0xFF, 0x80), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_X, 0xFF)]
#[case::asl_abs1(mem_abs_read_store(Cpu::ASL_ABSOLUTE, 0x1225, 0xFF), 0xFE, Cpu::FLAG_NEGATIVE | Cpu::FLAG_CARRY, Cpu::REG_A, 0xFE, 10, 0)]
#[case::asl_abs2(mem_abs_read_store(Cpu::ASL_ABSOLUTE, 0x12AA, 0x80), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x12, 10, 0)]
#[case::asl_abs_x1(mem_abs_index_read_store(Cpu::ASL_ABSOLUTE_X, 0x1225, 0xF, 0xFF), 0xFE, Cpu::FLAG_NEGATIVE | Cpu::FLAG_CARRY, Cpu::REG_A, 0xAB, Cpu::REG_X, 0x0F)]
#[case::asl_abs_x2(mem_abs_index_read_store(Cpu::ASL_ABSOLUTE_X, 0x12AA, 0xBB, 0x80), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x11, Cpu::REG_X, 0xBB)]
// LSR
#[case::lsr_zero1(mem_zero_read_store(Cpu::LSR_ZERO, 0xCA, 0x2), 0x1, 0, Cpu::REG_A, 0, 10, 0)]
#[case::lsr_zero2(mem_zero_read_store(Cpu::LSR_ZERO, 0xCB, 0x1), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, 10, 0)]
#[case::lsr_zero_x1(mem_zero_index_read_store(Cpu::LSR_ZERO_X, 0x80, 0x0F, 0x2), 0x1, 0, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::lsr_zero_x2(mem_zero_index_read_store(Cpu::LSR_ZERO_X, 0x80, 0xFF, 0x1), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_X, 0xFF)]
#[case::lsr_abs1(mem_abs_read_store(Cpu::LSR_ABSOLUTE, 0x1225, 0x2), 0x1, 0, Cpu::REG_A, 0xFE, 10, 0)]
#[case::lsr_abs2(mem_abs_read_store(Cpu::LSR_ABSOLUTE, 0x12AA, 0x1), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x12, 10, 0)]
#[case::lsr_abs_x1(mem_abs_index_read_store(Cpu::LSR_ABSOLUTE_X, 0x1225, 0xF, 0x2), 0x1, 0, Cpu::REG_A, 0xAB, Cpu::REG_X, 0x0F)]
#[case::lsr_abs_x2(mem_abs_index_read_store(Cpu::LSR_ABSOLUTE_X, 0x12AA, 0xBB, 0x1), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0x11, Cpu::REG_X, 0xBB)]
// ROL
#[case::rol_zero1(mem_zero_read_store(Cpu::ROL_ZERO, 0xCA, 0xFF), 0xFF, Cpu::FLAG_NEGATIVE | Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::rol_zero2(mem_zero_read_store(Cpu::ROL_ZERO, 0xCB, 0x80), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, 10, 0)]
#[case::rol_zero_x1(mem_zero_index_read_store(Cpu::ROL_ZERO_X, 0x80, 0x0F, 0xFF), 0xFE, Cpu::FLAG_NEGATIVE | Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::rol_zero_x2(mem_zero_index_read_store(Cpu::ROL_ZERO_X, 0x80, 0xFF, 0x80), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_X, 0xFF)]
#[case::rol_abs1(mem_abs_read_store(Cpu::ROL_ABSOLUTE, 0x1225, 0xFF), 0xFF, Cpu::FLAG_NEGATIVE | Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::rol_abs2(mem_abs_read_store(Cpu::ROL_ABSOLUTE, 0x12AA, 0x80), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, 10, 0)]
#[case::rol_abs_x1(mem_abs_index_read_store(Cpu::ROL_ABSOLUTE_X, 0x1225, 0xF, 0xFF), 0xFE, Cpu::FLAG_NEGATIVE | Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::rol_abs_x2(mem_abs_index_read_store(Cpu::ROL_ABSOLUTE_X, 0x12AA, 0xBB, 0x80), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_X, 0xBB)]
// ROR
#[case::ror_zero1(mem_zero_read_store(Cpu::ROR_ZERO, 0xCA, 0x1), 0x80, Cpu::FLAG_CARRY | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::ror_zero2(mem_zero_read_store(Cpu::ROR_ZERO, 0xCB, 0x1), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, 10, 0)]
#[case::ror_zero_x1(mem_zero_index_read_store(Cpu::ROR_ZERO_X, 0x80, 0x0F, 0xFF), 0x7F, Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::ror_zero_x2(mem_zero_index_read_store(Cpu::ROR_ZERO_X, 0x80, 0xFF, 0x1), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_X, 0xFF)]
#[case::ror_abs1(mem_abs_read_store(Cpu::ROR_ABSOLUTE, 0x1225, 0x1), 0x80, Cpu::FLAG_CARRY | Cpu::FLAG_NEGATIVE, Cpu::REG_A, 0, Cpu::REG_STAT, Cpu::FLAG_CARRY)]
#[case::ror_abs2(mem_abs_read_store(Cpu::ROR_ABSOLUTE, 0x12AA, 0x1), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, 10, 0)]
#[case::ror_abs_x1(mem_abs_index_read_store(Cpu::ROR_ABSOLUTE_X, 0x1225, 0xF, 0xFF), 0x7F, Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_X, 0x0F)]
#[case::ror_abs_x2(mem_abs_index_read_store(Cpu::ROR_ABSOLUTE_X, 0x12AA, 0xBB, 0x1), 0, Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, Cpu::REG_A, 0, Cpu::REG_X, 0xBB)]
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
    assert_eq!(RESET_EXEC_ADDRESS as i32 + op.bytes, cpu.pc as i32);
    assert_eq!(expected_stat, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(op.cycles, cpu.cycles_run);
    assert_eq!(op.mem.read8(op.addr as usize), expected_result);
}
