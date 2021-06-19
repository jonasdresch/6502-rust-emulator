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

#[test]
fn test_break_to_interrupt_and_return_without_affecting_flags() {
    let interrupt_addr: u16 = 0x1234;
    let mut mem = Mem::new();
    mem.reset();
    // Go to interrupt handler and execute a NOP after returning
    mem.load_programm(&[Cpu::BRK_IMPLIED, Cpu::NOP_IMPLIED]);
    // The cpu will continue from interrupt_addr as the interrupt handler address
    mem.write16(Cpu::IRQ_INTERRUPT_VECTOR_ADDR as usize, interrupt_addr);
    // Execute a NOP before returning from the interrupt handler
    mem.load_programm_at(interrupt_addr, &[Cpu::NOP_IMPLIED, Cpu::RTI_IMPLIED]);
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    let oldpc = cpu.pc;
    cpu.regs[Cpu::REG_STAT] = Cpu::FLAG_CARRY | Cpu::FLAG_NEGATIVE;
    cpu.process(7);
    assert_eq!(interrupt_addr, cpu.pc);
    assert_eq!(Cpu::FLAG_CARRY | Cpu::FLAG_NEGATIVE, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(STACK_OFFSET_START - 3, cpu.regs[Cpu::REG_SP]);
    assert_eq!(7, cpu.cycles_run);
    cpu.process(2); // NOP
    assert_eq!(interrupt_addr + 1, cpu.pc);
    assert_eq!(7 + 2, cpu.cycles_run);
    cpu.process(6); // RTI
    assert_eq!(oldpc + 1, cpu.pc); // +1 from reading the OPCODE from BRK
    assert_eq!(Cpu::FLAG_CARRY | Cpu::FLAG_NEGATIVE | Cpu::FLAG_BREAK, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(STACK_OFFSET_START, cpu.regs[Cpu::REG_SP]);
    assert_eq!(7 + 2 + 6, cpu.cycles_run);
    cpu.process(2); // NOP
    assert_eq!(oldpc + 2, cpu.pc);
    assert_eq!(7 + 2 + 6 + 2, cpu.cycles_run);
}
