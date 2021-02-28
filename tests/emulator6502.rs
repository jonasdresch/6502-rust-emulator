#[test]
fn test_cpu_reset_vector() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS, cpu.pc)
}

#[test]
fn test_cpu_lda_immediate() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDA_IMMEDIATE);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0xCA);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::LDA_IMMEDIATE);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0x0);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.process(2);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0xCA, cpu.a);
    assert_eq!(emulator6502::CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(2, cpu.cycles_run);
    cpu.process(2);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0x0, cpu.a);
    assert_eq!(emulator6502::CPU::FLAG_ZERO, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_zero_page() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDA_ZERO);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0xCA);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::LDA_ZERO);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0xCB);
    mem.write8(0xCA, 0xFE);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.process(3);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0xFE, cpu.a);
    assert_eq!(emulator6502::CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(3, cpu.cycles_run);
    cpu.process(3);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0x0, cpu.a);
    assert_eq!(emulator6502::CPU::FLAG_ZERO, cpu.status);
    assert_eq!(6, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_zero_page_x() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDA_ZERO_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x80);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::LDA_ZERO_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0x80);
    mem.write8(0x8F, 0xFE);
    mem.write8(0x7F, 0xA);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0x0F;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0xFE, cpu.a);
    assert_eq!(emulator6502::CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0xFF;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0xA, cpu.a);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_absolute() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDA_ABSOLUTE);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x34);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, 0x12);
    mem.write8(0x1234, 0xAB);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+3, cpu.pc);
    assert_eq!(0xAB, cpu.a);
    assert_eq!(1, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_absolute_x() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load program in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDA_ABSOLUTE_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x25);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, 0x12);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, emulator6502::CPU::LDA_ABSOLUTE_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+4) as usize, 0xAA);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+5) as usize, 0x12);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xF;
    // Flags should not affect the instruction
    cpu.status = emulator6502::CPU::FLAG_CARRY | emulator6502::CPU::FLAG_OVERFLOW;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+3, cpu.pc);
    assert_eq!(0xAB, cpu.a);
    assert_eq!(emulator6502::CPU::FLAG_NEGATIVE | emulator6502::CPU::FLAG_CARRY | emulator6502::CPU::FLAG_OVERFLOW, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0xBB;
    cpu.status = 0;
    cpu.process(5);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+6, cpu.pc);
    assert_eq!(0x11, cpu.a);
    assert_eq!(0, cpu.status);
    assert_eq!(9, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_absolute_y() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load program in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDA_ABSOLUTE_Y);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x25);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, 0x12);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, emulator6502::CPU::LDA_ABSOLUTE_Y);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+4) as usize, 0xAA);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+5) as usize, 0x12);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0xF;
    // TODO: Flags should not affect the instruction
    cpu.status = emulator6502::CPU::FLAG_CARRY | emulator6502::CPU::FLAG_OVERFLOW;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+3, cpu.pc);
    assert_eq!(0xAB, cpu.a);
    assert_eq!(emulator6502::CPU::FLAG_NEGATIVE | emulator6502::CPU::FLAG_CARRY | emulator6502::CPU::FLAG_OVERFLOW, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.y = 0xBB;
    cpu.status = 0;
    cpu.process(5);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+6, cpu.pc);
    assert_eq!(0x11, cpu.a);
    assert_eq!(0, cpu.status);
    assert_eq!(9, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_indirect_x() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load program in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDA_INDIRECT_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x25);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::LDA_INDIRECT_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0xAA);
    mem.write16(0x34, 0x1234);
    mem.write16(0x65, 0x1365);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xF;
    // TODO: Flags should not affect the instruction
    cpu.status = emulator6502::CPU::FLAG_CARRY | emulator6502::CPU::FLAG_OVERFLOW;
    cpu.process(6);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0xAB, cpu.a);
    assert_eq!(emulator6502::CPU::FLAG_NEGATIVE | emulator6502::CPU::FLAG_CARRY | emulator6502::CPU::FLAG_OVERFLOW, cpu.status);
    assert_eq!(6, cpu.cycles_run);
    // this will cause a wrap around as the addres will be higher than 255
    cpu.x = 0xBB;
    cpu.status = 0;
    cpu.process(6);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0x11, cpu.a);
    assert_eq!(0, cpu.status);
    assert_eq!(12, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_indirect_y() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load program in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDA_INDIRECT_Y);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x25);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::LDA_INDIRECT_Y);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0xAA);
    mem.write16(0x25, 0x1225);
    mem.write16(0xAA, 0x12AA);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0xF;
    // TODO: Flags should not affect the instruction
    cpu.status = emulator6502::CPU::FLAG_CARRY | emulator6502::CPU::FLAG_OVERFLOW;
    cpu.process(5);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0xAB, cpu.a);
    assert_eq!(emulator6502::CPU::FLAG_NEGATIVE | emulator6502::CPU::FLAG_CARRY | emulator6502::CPU::FLAG_OVERFLOW, cpu.status);
    assert_eq!(5, cpu.cycles_run);
    cpu.y = 0xBB;
    cpu.status = 0;
    cpu.process(6);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0x11, cpu.a);
    assert_eq!(0, cpu.status);
    assert_eq!(11, cpu.cycles_run);
}

#[test]
fn test_cpu_ldx_immediate() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDX_IMMEDIATE);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0xCA);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::LDX_IMMEDIATE);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0x0);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.process(2);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0xCA, cpu.x);
    assert_eq!(1, cpu.status);
    assert_eq!(2, cpu.cycles_run);
    cpu.process(2);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0x0, cpu.x);
    assert_eq!(0b0010_0000, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_ldx_zero_page() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDX_ZERO);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0xCA);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::LDX_ZERO);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0xCB);
    mem.write8(0xCA, 0xFE);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.process(3);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0xFE, cpu.x);
    assert_eq!(1, cpu.status);
    assert_eq!(3, cpu.cycles_run);
    cpu.process(3);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0x0, cpu.x);
    assert_eq!(0b0010_0000, cpu.status);
    assert_eq!(6, cpu.cycles_run);
}

#[test]
fn test_cpu_ldx_zero_page_y() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDX_ZERO_Y);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x80);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::LDX_ZERO_Y);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0x80);
    mem.write8(0x8F, 0xFE);
    mem.write8(0x7F, 0xA);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0x0F;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0xFE, cpu.x);
    assert_eq!(1, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.y = 0xFF;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0xA, cpu.x);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
}

#[test]
fn test_cpu_ldx_absolute() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDX_ABSOLUTE);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x34);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, 0x12);
    mem.write8(0x1234, 0xAB);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+3, cpu.pc);
    assert_eq!(0xAB, cpu.x);
    assert_eq!(1, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_ldx_absolute_y() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load program in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDX_ABSOLUTE_Y);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x25);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, 0x12);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, emulator6502::CPU::LDX_ABSOLUTE_Y);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+4) as usize, 0xAA);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+5) as usize, 0x12);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0xF;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+3, cpu.pc);
    assert_eq!(0xAB, cpu.x);
    assert_eq!(emulator6502::CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.y = 0xBB;
    cpu.process(5);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+6, cpu.pc);
    assert_eq!(0x11, cpu.x);
    assert_eq!(0, cpu.status);
    assert_eq!(9, cpu.cycles_run);
}

#[test]
fn test_cpu_ldy_immediate() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDY_IMMEDIATE);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0xCA);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::LDY_IMMEDIATE);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0x0);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.process(2);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0xCA, cpu.y);
    assert_eq!(1, cpu.status);
    assert_eq!(2, cpu.cycles_run);
    cpu.process(2);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0x0, cpu.y);
    assert_eq!(0b0010_0000, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_ldy_zero_page() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDY_ZERO);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0xCA);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::LDY_ZERO);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0xCB);
    mem.write8(0xCA, 0xFE);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.process(3);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0xFE, cpu.y);
    assert_eq!(1, cpu.status);
    assert_eq!(3, cpu.cycles_run);
    cpu.process(3);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0x0, cpu.y);
    assert_eq!(0b0010_0000, cpu.status);
    assert_eq!(6, cpu.cycles_run);
}

#[test]
fn test_cpu_ldy_zero_page_x() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDY_ZERO_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x80);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::LDY_ZERO_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0x80);
    mem.write8(0x8F, 0xFE);
    mem.write8(0x7F, 0xA);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0x0F;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0xFE, cpu.y);
    assert_eq!(1, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0xFF;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0xA, cpu.y);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
}

#[test]
fn test_cpu_ldy_absolute() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDY_ABSOLUTE);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x34);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, 0x12);
    mem.write8(0x1234, 0xAB);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+3, cpu.pc);
    assert_eq!(0xAB, cpu.y);
    assert_eq!(1, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_ldy_absolute_x() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load program in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::LDY_ABSOLUTE_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x25);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, 0x12);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, emulator6502::CPU::LDY_ABSOLUTE_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+4) as usize, 0xAA);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+5) as usize, 0x12);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xF;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+3, cpu.pc);
    assert_eq!(0xAB, cpu.y);
    assert_eq!(emulator6502::CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0xBB;
    cpu.process(5);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+6, cpu.pc);
    assert_eq!(0x11, cpu.y);
    assert_eq!(0, cpu.status);
    assert_eq!(9, cpu.cycles_run);
}

#[test]
fn test_cpu_sta_zero_page() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::STA_ZERO);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0xCA);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::STA_ZERO);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0xCB);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.a = 0xFE;
    cpu.process(3);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(3, cpu.cycles_run);
    cpu.a = 0x12;
    cpu.process(3);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(6, cpu.cycles_run);
    assert_eq!(mem.read8(0xCA), 0xFE);
    assert_eq!(mem.read8(0xCB), 0x12);
}

#[test]
fn test_cpu_sta_zero_page_x() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    // Load programm in memory
    mem.write8(emulator6502::RESET_EXEC_ADDRESS as usize, emulator6502::CPU::STA_ZERO_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+1) as usize, 0x80);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+2) as usize, emulator6502::CPU::STA_ZERO_X);
    mem.write8((emulator6502::RESET_EXEC_ADDRESS+3) as usize, 0x80);
    let mut cpu = emulator6502::CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0x0F;
    cpu.a = 0xFE;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+2, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0xFF;
    cpu.a = 0xA;
    cpu.process(4);
    assert_eq!(emulator6502::RESET_EXEC_ADDRESS+4, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
    assert_eq!(mem.read8(0x8F), 0xFE);
    assert_eq!(mem.read8(0x7F), 0xA);
}


#[test]
fn test_mem_read_limits_ok() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    assert_eq!(0, mem.read8(0));
    assert_eq!(0, mem.read8(65535));
}

#[test]
#[should_panic(expected = "memory access out ouf bounds")]
fn test_mem_read_limits_nok() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    assert_eq!(0, mem.read8(65536));
}

#[test]
#[should_panic(expected = "memory access out ouf bounds")]
fn test_mem_write_limits_nok() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    mem.write8(65536, 1)
}

#[test]
fn test_mem_read_reset_vector_ok() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    assert_eq!(0xFC, mem.read8(emulator6502::RESET_VECTOR_ADDR));
    assert_eq!(0xE2, mem.read8(emulator6502::RESET_VECTOR_ADDR+1));
    assert_eq!(0, mem.read8(667));
}

#[test]
fn test_mem_write_read_ok() {
    let mut mem = emulator6502::MEM::new();
    mem.reset();
    mem.write8(666, 200);
    assert_eq!(200, mem.read8(666));
    assert_eq!(0, mem.read8(665));
    assert_eq!(0, mem.read8(667));
}
