use emulator6502::*;

#[test]
fn test_jmp_absolute() {
    let mut mem = Mem::new();
    mem.reset();
    let target_addr: u16 = 0x1234;
    // JMP $1234
    mem.load_programm(&[Cpu::JMP_ABSOLUTE, (target_addr & 0x00FF) as u8, ((target_addr & 0xFF00) >> 8) as u8]);
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    cpu.regs[Cpu::REG_STAT] = Cpu::FLAG_ZERO | Cpu::FLAG_CARRY;
    cpu.process(3);

    assert_eq!(target_addr, cpu.pc, "PC should be set to target address");
    assert_eq!(3, cpu.cycles_run, "JMP absolute should take 3 cycles");
    assert_eq!(Cpu::FLAG_ZERO | Cpu::FLAG_CARRY, cpu.regs[Cpu::REG_STAT], "Flags must not be affected");
    assert_eq!(STACK_OFFSET_START, cpu.regs[Cpu::REG_SP], "SP must not be affected");
}

#[test]
fn test_jmp_absolute_execution_continues() {
    let mut mem = Mem::new();
    mem.reset();
    let target_addr: u16 = 0x2000;
    mem.load_programm(&[Cpu::JMP_ABSOLUTE, 0x00, 0x20]);
    mem.load_programm_at(target_addr, &[Cpu::NOP_IMPLIED]);
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();

    // JMP (3 cycles) + NOP (2 cycles) = 5 cycles
    cpu.process(5);
    assert_eq!(target_addr + 1, cpu.pc);
    assert_eq!(5, cpu.cycles_run);
}

#[test]
fn test_jmp_indirect() {
    let mut mem = Mem::new();
    mem.reset();
    let ptr_addr: u16 = 0x2050;
    let target_addr: u16 = 0xABCD;

    // Pointer at $2050 holds $ABCD
    mem.write8(ptr_addr as usize, (target_addr & 0x00FF) as u8);
    mem.write8((ptr_addr + 1) as usize, ((target_addr & 0xFF00) >> 8) as u8);

    // JMP ($2050)
    mem.load_programm(&[Cpu::JMP_INDIRECT, (ptr_addr & 0x00FF) as u8, ((ptr_addr & 0xFF00) >> 8) as u8]);
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    cpu.regs[Cpu::REG_STAT] = Cpu::FLAG_NEGATIVE | Cpu::FLAG_OVERFLOW;
    cpu.process(5);

    assert_eq!(target_addr, cpu.pc, "PC should be loaded from indirect pointer");
    assert_eq!(5, cpu.cycles_run, "JMP indirect should take 5 cycles");
    assert_eq!(Cpu::FLAG_NEGATIVE | Cpu::FLAG_OVERFLOW, cpu.regs[Cpu::REG_STAT], "Flags must not be affected");
    assert_eq!(STACK_OFFSET_START, cpu.regs[Cpu::REG_SP], "SP must not be affected");
}

#[test]
fn test_jmp_indirect_page_boundary_bug() {
    let mut mem = Mem::new();
    mem.reset();

    // The indirect pointer is at $10FF (page boundary).
    // In NMOS 6502 hardware, the low byte is read from $10FF,
    // but the high byte wraps to $1000 instead of reading from $1100.
    let ptr_addr: u16 = 0x10FF;
    let expected_low: u8 = 0x34;
    let expected_high: u8 = 0x12;

    mem.write8(ptr_addr as usize, expected_low);
    mem.write8(0x1000, expected_high); // wrapped address
    mem.write8(0x1100, 0x99); // buggy non-wrapped address

    // JMP ($10FF)
    mem.load_programm(&[Cpu::JMP_INDIRECT, 0xFF, 0x10]);
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    cpu.process(5);

    assert_eq!(0x1234, cpu.pc, "High byte must wrap to start of page ($1000) instead of crossing to $1100");
    assert_eq!(5, cpu.cycles_run);
}

#[test]
fn test_jsr_absolute_pc_and_sp() {
    let mut mem = Mem::new();
    mem.reset();
    let jsr_addr: u16 = 0x1000;
    let target_addr: u16 = 0x2580;

    // JSR $2580 at address $1000
    mem.load_programm_at(jsr_addr, &[Cpu::JSR_ABSOLUTE, 0x80, 0x25]);
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    cpu.pc = jsr_addr;
    cpu.regs[Cpu::REG_STAT] = Cpu::FLAG_CARRY;

    cpu.process(6);

    assert_eq!(target_addr, cpu.pc, "PC should jump to target address");
    assert_eq!(6, cpu.cycles_run, "JSR absolute should take 6 cycles");
    assert_eq!(STACK_OFFSET_START - 2, cpu.regs[Cpu::REG_SP], "SP should be decremented by 2");
    assert_eq!(Cpu::FLAG_CARRY, cpu.regs[Cpu::REG_STAT], "Flags must not be affected");

    // The return address pushed to stack is PC + 2 (address of 3rd byte of JSR: $1002)
    // High byte pushed first at STACK_REAL_START, low byte at STACK_REAL_START - 1
    assert_eq!(0x10, mem.read8(STACK_REAL_START), "High byte of return address on stack");
    assert_eq!(0x02, mem.read8(STACK_REAL_START - 1), "Low byte of return address on stack");
}

#[test]
fn test_rts_implied_pc_and_sp() {
    let mut mem = Mem::new();
    mem.reset();
    let rts_addr: u16 = 0x2000;
    let return_addr_on_stack: u16 = 0x1002;

    // Simulate stack state left by JSR:
    // High byte at STACK_REAL_START, low byte at STACK_REAL_START - 1
    mem.write8(STACK_REAL_START, ((return_addr_on_stack & 0xFF00) >> 8) as u8);
    mem.write8(STACK_REAL_START - 1, (return_addr_on_stack & 0x00FF) as u8);

    mem.load_programm_at(rts_addr, &[Cpu::RTS_IMPLIED]);
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    cpu.pc = rts_addr;
    cpu.regs[Cpu::REG_SP] = STACK_OFFSET_START - 2;
    cpu.regs[Cpu::REG_STAT] = Cpu::FLAG_NEGATIVE;

    cpu.process(6);

    // RTS pulls return address and adds 1: $1002 + 1 = $1003
    assert_eq!(return_addr_on_stack + 1, cpu.pc, "PC should be return address + 1");
    assert_eq!(6, cpu.cycles_run, "RTS implied should take 6 cycles");
    assert_eq!(STACK_OFFSET_START, cpu.regs[Cpu::REG_SP], "SP should be restored (+2)");
    assert_eq!(Cpu::FLAG_NEGATIVE, cpu.regs[Cpu::REG_STAT], "Flags must not be affected");
}

#[test]
fn test_jsr_and_rts_subroutine_flow() {
    let mut mem = Mem::new();
    mem.reset();
    let caller_addr: u16 = 0x1000;
    let sub_addr: u16 = 0x2000;

    // Caller code:
    // $1000: JSR $2000 (3 bytes)
    // $1003: NOP       (1 byte)
    mem.load_programm_at(caller_addr, &[Cpu::JSR_ABSOLUTE, (sub_addr & 0x00FF) as u8, ((sub_addr & 0xFF00) >> 8) as u8, Cpu::NOP_IMPLIED]);

    // Subroutine code:
    // $2000: NOP       (1 byte)
    // $2001: RTS       (1 byte)
    mem.load_programm_at(sub_addr, &[Cpu::NOP_IMPLIED, Cpu::RTS_IMPLIED]);

    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    cpu.pc = caller_addr;

    // 1. Execute JSR (6 cycles)
    cpu.process(6);
    assert_eq!(sub_addr, cpu.pc);
    assert_eq!(STACK_OFFSET_START - 2, cpu.regs[Cpu::REG_SP]);
    assert_eq!(6, cpu.cycles_run);

    // 2. Execute NOP in subroutine (2 cycles)
    cpu.process(2);
    assert_eq!(sub_addr + 1, cpu.pc);
    assert_eq!(STACK_OFFSET_START - 2, cpu.regs[Cpu::REG_SP]);
    assert_eq!(8, cpu.cycles_run);

    // 3. Execute RTS in subroutine (6 cycles)
    cpu.process(6);
    assert_eq!(caller_addr + 3, cpu.pc, "PC should return to instruction following JSR");
    assert_eq!(STACK_OFFSET_START, cpu.regs[Cpu::REG_SP], "SP should be fully restored");
    assert_eq!(14, cpu.cycles_run);

    // 4. Execute NOP after returning (2 cycles)
    cpu.process(2);
    assert_eq!(caller_addr + 4, cpu.pc);
    assert_eq!(16, cpu.cycles_run);
}
