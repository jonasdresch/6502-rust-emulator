use emulator6502::*;
use rstest::*;

mod fixtures;
use fixtures::{mem_branch, Operation};

#[rstest]
// BCC
#[case::bcc_branch_forward_if_carry_is_clear(mem_branch(Cpu::BCC_RELATIVE, 0x8, true, false, RESET_EXEC_ADDRESS), 0)]
#[case::bcc_branch_forward_page_if_carry_is_clear(mem_branch(Cpu::BCC_RELATIVE, 0x20, true, true, RESET_EXEC_ADDRESS), 0)]
#[case::bcc_dont_branch_if_carry_not_clear(mem_branch(Cpu::BCC_RELATIVE, 0x8, false, false, RESET_EXEC_ADDRESS), Cpu::FLAG_CARRY)]
#[case::bcc_branch_backward_if_carry_is_clear(mem_branch(Cpu::BCC_RELATIVE, -0x8, true, false, RESET_EXEC_ADDRESS), 0)]
#[case::bcc_branch_backward_page_if_carry_is_clear(mem_branch(Cpu::BCC_RELATIVE, -0x7F, true, true, 0xFC7C), 0)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bcc_dont_wrap_beyond_mem_limit(mem_branch(Cpu::BCC_RELATIVE, 0x7F, true, true, 0xFFE2), 0)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bcc_dont_wrap_beyond_lower_mem_limit(mem_branch(Cpu::BCC_RELATIVE, -0x7F, true, true, 0x7C), 0)]
// BCS
#[case::bcs_branch_forward_if_carry_is_set(mem_branch(Cpu::BCS_RELATIVE, 0x8, true, false, RESET_EXEC_ADDRESS), Cpu::FLAG_CARRY)]
#[case::bcs_branch_forward_page_if_carry_is_set(mem_branch(Cpu::BCS_RELATIVE, 0x20, true, true, RESET_EXEC_ADDRESS), Cpu::FLAG_CARRY)]
#[case::bcs_dont_branch_if_carry_not_set(mem_branch(Cpu::BCS_RELATIVE, 0x8, false, false, RESET_EXEC_ADDRESS), 0)]
#[case::bcs_branch_backward_if_carry_is_set(mem_branch(Cpu::BCS_RELATIVE, -0x8, true, false, RESET_EXEC_ADDRESS), Cpu::FLAG_CARRY)]
#[case::bcs_branch_backward_page_if_carry_is_set(mem_branch(Cpu::BCS_RELATIVE, -0x7F, true, true, 0xFC7C), Cpu::FLAG_CARRY)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bcs_dont_wrap_beyond_mem_limit(mem_branch(Cpu::BCS_RELATIVE, 0x7F, true, true, 0xFFE2), Cpu::FLAG_CARRY)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bcs_dont_wrap_beyond_lower_mem_limit(mem_branch(Cpu::BCS_RELATIVE, -0x7F, true, true, 0x7C), Cpu::FLAG_CARRY)]
// BEQ
#[case::beq_branch_forward_if_zero_is_set(mem_branch(Cpu::BEQ_RELATIVE, 0x8, true, false, RESET_EXEC_ADDRESS), Cpu::FLAG_ZERO)]
#[case::beq_branch_forward_page_if_zero_is_set(mem_branch(Cpu::BEQ_RELATIVE, 0x20, true, true, RESET_EXEC_ADDRESS), Cpu::FLAG_ZERO)]
#[case::beq_dont_branch_if_zero_not_set(mem_branch(Cpu::BEQ_RELATIVE, 0x8, false, false, RESET_EXEC_ADDRESS), 0)]
#[case::beq_branch_backward_if_zero_is_set(mem_branch(Cpu::BEQ_RELATIVE, -0x8, true, false, RESET_EXEC_ADDRESS), Cpu::FLAG_ZERO)]
#[case::beq_branch_backward_page_if_zero_is_set(mem_branch(Cpu::BEQ_RELATIVE, -0x7F, true, true, 0xFC7C), Cpu::FLAG_ZERO)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::beq_dont_wrap_beyond_mem_limit(mem_branch(Cpu::BEQ_RELATIVE, 0x7F, true, true, 0xFFE2), Cpu::FLAG_ZERO)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::beq_dont_wrap_beyond_lower_mem_limit(mem_branch(Cpu::BEQ_RELATIVE, -0x7F, true, true, 0x7C), Cpu::FLAG_ZERO)]
// BMI
#[case::bmi_branch_forward_if_negative_is_set(mem_branch(Cpu::BMI_RELATIVE, 0x8, true, false, RESET_EXEC_ADDRESS), Cpu::FLAG_NEGATIVE)]
#[case::bmi_branch_forward_page_if_negative_is_set(mem_branch(Cpu::BMI_RELATIVE, 0x20, true, true, RESET_EXEC_ADDRESS), Cpu::FLAG_NEGATIVE)]
#[case::bmi_dont_branch_if_negative_not_set(mem_branch(Cpu::BMI_RELATIVE, 0x8, false, false, RESET_EXEC_ADDRESS), 0)]
#[case::bmi_branch_backward_if_negative_is_set(mem_branch(Cpu::BMI_RELATIVE, -0x8, true, false, RESET_EXEC_ADDRESS), Cpu::FLAG_NEGATIVE)]
#[case::bmi_branch_backward_page_if_negative_is_set(mem_branch(Cpu::BMI_RELATIVE, -0x7F, true, true, 0xFC7C), Cpu::FLAG_NEGATIVE)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bmi_dont_wrap_beyond_mem_limit(mem_branch(Cpu::BMI_RELATIVE, 0x7F, true, true, 0xFFE2), Cpu::FLAG_NEGATIVE)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bmi_dont_wrap_beyond_lower_mem_limit(mem_branch(Cpu::BMI_RELATIVE, -0x7F, true, true, 0x7C), Cpu::FLAG_NEGATIVE)]
// BNE
#[case::bne_branch_forward_if_zero_is_clear(mem_branch(Cpu::BNE_RELATIVE, 0x8, true, false, RESET_EXEC_ADDRESS), 0)]
#[case::bne_branch_forward_page_if_zero_is_clear(mem_branch(Cpu::BNE_RELATIVE, 0x20, true, true, RESET_EXEC_ADDRESS), 0)]
#[case::bne_dont_branch_if_zero_not_clear(mem_branch(Cpu::BNE_RELATIVE, 0x8, false, false, RESET_EXEC_ADDRESS), Cpu::FLAG_ZERO)]
#[case::bne_branch_backward_if_zero_is_clear(mem_branch(Cpu::BNE_RELATIVE, -0x8, true, false, RESET_EXEC_ADDRESS), 0)]
#[case::bne_branch_backward_page_if_zero_is_clear(mem_branch(Cpu::BNE_RELATIVE, -0x7F, true, true, 0xFC7C), 0)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bne_dont_wrap_beyond_mem_limit(mem_branch(Cpu::BNE_RELATIVE, 0x7F, true, true, 0xFFE2), 0)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bne_dont_wrap_beyond_lower_mem_limit(mem_branch(Cpu::BNE_RELATIVE, -0x7F, true, true, 0x7C), 0)]
// BPL
#[case::bpl_branch_forward_if_zero_is_clear(mem_branch(Cpu::BPL_RELATIVE, 0x8, true, false, RESET_EXEC_ADDRESS), 0)]
#[case::bpl_branch_forward_page_if_zero_is_clear(mem_branch(Cpu::BPL_RELATIVE, 0x20, true, true, RESET_EXEC_ADDRESS), 0)]
#[case::bpl_dont_branch_if_zero_not_clear(mem_branch(Cpu::BPL_RELATIVE, 0x8, false, false, RESET_EXEC_ADDRESS), Cpu::FLAG_NEGATIVE)]
#[case::bpl_branch_backward_if_zero_is_clear(mem_branch(Cpu::BPL_RELATIVE, -0x8, true, false, RESET_EXEC_ADDRESS), 0)]
#[case::bpl_branch_backward_page_if_zero_is_clear(mem_branch(Cpu::BPL_RELATIVE, -0x7F, true, true, 0xFC7C), 0)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bpl_dont_wrap_beyond_mem_limit(mem_branch(Cpu::BPL_RELATIVE, 0x7F, true, true, 0xFFE2), 0)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bpl_dont_wrap_beyond_lower_mem_limit(mem_branch(Cpu::BPL_RELATIVE, -0x7F, true, true, 0x7C), 0)]
// BVS
#[case::bvs_branch_forward_if_overflow_is_set(mem_branch(Cpu::BVS_RELATIVE, 0x8, true, false, RESET_EXEC_ADDRESS), Cpu::FLAG_OVERFLOW)]
#[case::bvs_branch_forward_page_if_overflow_is_set(mem_branch(Cpu::BVS_RELATIVE, 0x20, true, true, RESET_EXEC_ADDRESS), Cpu::FLAG_OVERFLOW)]
#[case::bvs_dont_branch_if_overflow_not_set(mem_branch(Cpu::BVS_RELATIVE, 0x8, false, false, RESET_EXEC_ADDRESS), 0)]
#[case::bvs_branch_backward_if_overflow_is_set(mem_branch(Cpu::BVS_RELATIVE, -0x8, true, false, RESET_EXEC_ADDRESS), Cpu::FLAG_OVERFLOW)]
#[case::bvs_branch_backward_page_if_overflow_is_set(mem_branch(Cpu::BVS_RELATIVE, -0x7F, true, true, 0xFC7C), Cpu::FLAG_OVERFLOW)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bvs_dont_wrap_beyond_mem_limit(mem_branch(Cpu::BVS_RELATIVE, 0x7F, true, true, 0xFFE2), Cpu::FLAG_OVERFLOW)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bvs_dont_wrap_beyond_lower_mem_limit(mem_branch(Cpu::BVS_RELATIVE, -0x7F, true, true, 0x7C), Cpu::FLAG_OVERFLOW)]
// BVC
#[case::bvc_branch_forward_if_overflow_is_clear(mem_branch(Cpu::BVC_RELATIVE, 0x8, true, false, RESET_EXEC_ADDRESS), 0)]
#[case::bvc_branch_forward_page_if_overflow_is_clear(mem_branch(Cpu::BVC_RELATIVE, 0x20, true, true, RESET_EXEC_ADDRESS), 0)]
#[case::bvc_dont_branch_if_overflow_not_clear(mem_branch(Cpu::BVC_RELATIVE, 0x8, false, false, RESET_EXEC_ADDRESS), Cpu::FLAG_OVERFLOW)]
#[case::bvc_branch_backward_if_overflow_is_clear(mem_branch(Cpu::BVC_RELATIVE, -0x8, true, false, RESET_EXEC_ADDRESS), 0)]
#[case::bvc_branch_backward_page_if_overflow_is_clear(mem_branch(Cpu::BVC_RELATIVE, -0x7F, true, true, 0xFC7C), 0)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bvc_dont_wrap_beyond_mem_limit(mem_branch(Cpu::BVC_RELATIVE, 0x7F, true, true, 0xFFE2), 0)]
#[should_panic(expected = "Trying to branch beyond memory limits")]
#[case::bvc_dont_wrap_beyond_lower_mem_limit(mem_branch(Cpu::BVC_RELATIVE, -0x7F, true, true, 0x7C), 0)]
fn branch_tests(#[case] mut op: Operation, #[case] flag_value: u8) {
    let mut cpu = Cpu::new(&mut op.mem);
    cpu.reset();
    // Set the conditional value that the instruction will check
    cpu.regs[Cpu::REG_STAT] = flag_value;
    // To test backwards page change we must set the start memory address
    // because we can't branch 0xE2 (226) bytes from the default RESET_EXEC_ADDRESS (0xFCE2).
    // The branch offset is signed, so the pc can change -127 or +127 max.
    cpu.pc = op.addr;
    cpu.process(op.cycles);
    assert_eq!((op.addr as i32 + op.bytes) as u16, cpu.pc, "PC not expected");
    assert_eq!(flag_value as u8, cpu.regs[Cpu::REG_STAT], "Stat reg expected");
    assert_eq!(op.cycles, cpu.cycles_run, "Cycles run not expected");
}
