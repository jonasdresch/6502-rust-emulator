use emulator6502::*;

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
