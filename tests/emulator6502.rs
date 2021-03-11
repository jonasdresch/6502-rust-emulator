use emulator6502::*;

#[test]
fn test_cpu_reset_vector() {
    let mut mem = MEM::new();
    mem.reset();
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    assert_eq!(RESET_EXEC_ADDRESS, cpu.pc)
}

#[test]
fn test_cpu_lda_immediate() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDA_IMMEDIATE, 0xCA, CPU::LDA_IMMEDIATE, 0x0]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xCA, cpu.a);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(2, cpu.cycles_run);
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x0, cpu.a);
    assert_eq!(CPU::FLAG_ZERO, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_zero_page() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDA_ZERO, 0xCA, CPU::LDA_ZERO, 0x0]);
    mem.write8(0xCA, 0xFE);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xFE, cpu.a);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(3, cpu.cycles_run);
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x0, cpu.a);
    assert_eq!(CPU::FLAG_ZERO, cpu.status);
    assert_eq!(6, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_zero_page_x() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDA_ZERO_X, 0x80, CPU::LDA_ZERO_X, 0x80]);
    mem.write8(0x8F, 0xFE);
    mem.write8(0x7F, 0xA);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0x0F;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xFE, cpu.a);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0xFF;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0xA, cpu.a);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_absolute() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDA_ABSOLUTE, 0x34, 0x12]);
    mem.write8(0x1234, 0xAB);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0xAB, cpu.a);
    assert_eq!(1, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_absolute_x() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDA_ABSOLUTE_X, 0x25, 0x12, CPU::LDA_ABSOLUTE_X, 0xAA, 0x12]);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xF;
    // Flags should not affect the instruction
    cpu.status = CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0xAB, cpu.a);
    assert_eq!(CPU::FLAG_NEGATIVE | CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0xBB;
    cpu.status = 0;
    cpu.process(5);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0x11, cpu.a);
    assert_eq!(0, cpu.status);
    assert_eq!(9, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_absolute_y() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDA_ABSOLUTE_Y, 0x25, 0x12, CPU::LDA_ABSOLUTE_Y, 0xAA, 0x12]);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0xF;
    // TODO: Flags should not affect the instruction
    cpu.status = CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0xAB, cpu.a);
    assert_eq!(CPU::FLAG_NEGATIVE | CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.y = 0xBB;
    cpu.status = 0;
    cpu.process(5);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0x11, cpu.a);
    assert_eq!(0, cpu.status);
    assert_eq!(9, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_indirect_x() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDA_INDIRECT_X, 0x25, CPU::LDA_INDIRECT_X, 0xAA]);
    mem.write16(0x34, 0x1234);
    mem.write16(0x65, 0x1365);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xF;
    // TODO: Flags should not affect the instruction
    cpu.status = CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW;
    cpu.process(6);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xAB, cpu.a);
    assert_eq!(CPU::FLAG_NEGATIVE | CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW, cpu.status);
    assert_eq!(6, cpu.cycles_run);
    // this will cause a wrap around as the addres will be higher than 255
    cpu.x = 0xBB;
    cpu.status = 0;
    cpu.process(6);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x11, cpu.a);
    assert_eq!(0, cpu.status);
    assert_eq!(12, cpu.cycles_run);
}

#[test]
fn test_cpu_lda_indirect_y() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDA_INDIRECT_Y, 0x25, CPU::LDA_INDIRECT_Y, 0xAA]);
    mem.write16(0x25, 0x1225);
    mem.write16(0xAA, 0x12AA);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0xF;
    // TODO: Flags should not affect the instruction
    cpu.status = CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW;
    cpu.process(5);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xAB, cpu.a);
    assert_eq!(CPU::FLAG_NEGATIVE | CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW, cpu.status);
    assert_eq!(5, cpu.cycles_run);
    cpu.y = 0xBB;
    cpu.status = 0;
    cpu.process(6);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x11, cpu.a);
    assert_eq!(0, cpu.status);
    assert_eq!(11, cpu.cycles_run);
}

#[test]
fn test_cpu_ldx_immediate() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDX_IMMEDIATE, 0xCA, CPU::LDX_IMMEDIATE, 0x0]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xCA, cpu.x);
    assert_eq!(1, cpu.status);
    assert_eq!(2, cpu.cycles_run);
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x0, cpu.x);
    assert_eq!(0b0010_0000, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_ldx_zero_page() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDX_ZERO, 0xCA, CPU::LDX_ZERO, 0xCB]);
    mem.write8(0xCA, 0xFE);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xFE, cpu.x);
    assert_eq!(1, cpu.status);
    assert_eq!(3, cpu.cycles_run);
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x0, cpu.x);
    assert_eq!(0b0010_0000, cpu.status);
    assert_eq!(6, cpu.cycles_run);
}

#[test]
fn test_cpu_ldx_zero_page_y() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDX_ZERO_Y, 0x80, CPU::LDX_ZERO_Y, 0x80]);
    mem.write8(0x8F, 0xFE);
    mem.write8(0x7F, 0xA);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0x0F;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xFE, cpu.x);
    assert_eq!(1, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.y = 0xFF;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0xA, cpu.x);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
}

#[test]
fn test_cpu_ldx_absolute() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDX_ABSOLUTE, 0x34, 0x12]);
    mem.write8(0x1234, 0xAB);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0xAB, cpu.x);
    assert_eq!(1, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_ldx_absolute_y() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDX_ABSOLUTE_Y, 0x25, 0x12, CPU::LDX_ABSOLUTE_Y, 0xAA, 0x12]);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0xF;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0xAB, cpu.x);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.y = 0xBB;
    cpu.process(5);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0x11, cpu.x);
    assert_eq!(0, cpu.status);
    assert_eq!(9, cpu.cycles_run);
}

#[test]
fn test_cpu_ldy_immediate() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDY_IMMEDIATE, 0xCA, CPU::LDY_IMMEDIATE, 0x0]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xCA, cpu.y);
    assert_eq!(1, cpu.status);
    assert_eq!(2, cpu.cycles_run);
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x0, cpu.y);
    assert_eq!(0b0010_0000, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_ldy_zero_page() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDY_ZERO, 0xCA, CPU::LDY_ZERO, 0xCB]);
    mem.write8(0xCA, 0xFE);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xFE, cpu.y);
    assert_eq!(1, cpu.status);
    assert_eq!(3, cpu.cycles_run);
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x0, cpu.y);
    assert_eq!(0b0010_0000, cpu.status);
    assert_eq!(6, cpu.cycles_run);
}

#[test]
fn test_cpu_ldy_zero_page_x() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDY_ZERO_X, 0x80, CPU::LDY_ZERO_X, 0x80]);
    mem.write8(0x8F, 0xFE);
    mem.write8(0x7F, 0xA);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0x0F;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xFE, cpu.y);
    assert_eq!(1, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0xFF;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0xA, cpu.y);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
}

#[test]
fn test_cpu_ldy_absolute() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDY_ABSOLUTE, 0x34, 0x12]);
    mem.write8(0x1234, 0xAB);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0xAB, cpu.y);
    assert_eq!(1, cpu.status);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_cpu_ldy_absolute_x() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDY_ABSOLUTE_X, 0x25, 0x12, CPU::LDY_ABSOLUTE_X, 0xAA, 0x12]);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xF;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0xAB, cpu.y);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0xBB;
    cpu.process(5);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0x11, cpu.y);
    assert_eq!(0, cpu.status);
    assert_eq!(9, cpu.cycles_run);
}

#[test]
fn test_cpu_sta_zero_page() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STA_ZERO, 0xCA, CPU::STA_ZERO, 0xCB]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.a = 0xFE;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(3, cpu.cycles_run);
    cpu.a = 0x12;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(6, cpu.cycles_run);
    assert_eq!(mem.read8(0xCA), 0xFE);
    assert_eq!(mem.read8(0xCB), 0x12);
}

#[test]
fn test_cpu_sta_zero_page_x() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STA_ZERO_X, 0x80, CPU::STA_ZERO_X, 0x80]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0x0F;
    cpu.a = 0xFE;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0xFF;
    cpu.a = 0xA;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
    assert_eq!(mem.read8(0x8F), 0xFE);
    assert_eq!(mem.read8(0x7F), 0xA);
}

#[test]
fn test_cpu_sta_absolute() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STA_ABSOLUTE, 0x25, 0x12, CPU::STA_ABSOLUTE, 0xAA, 0x12]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.a = 0xFE;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.a = 0x12;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
    assert_eq!(mem.read8(0x1225), 0xFE);
    assert_eq!(mem.read8(0x12AA), 0x12);
}

#[test]
fn test_cpu_sta_absolute_x() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STA_ABSOLUTE_X, 0x25, 0x12, CPU::STA_ABSOLUTE_X, 0xAA, 0x12]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xF;
    cpu.a = 0xAB;
    cpu.process(5);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(5, cpu.cycles_run);
    cpu.x = 0xBB;
    cpu.a = 0x11;
    cpu.process(5);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(10, cpu.cycles_run);
    assert_eq!(mem.read8(0x1234), 0xAB);
    assert_eq!(mem.read8(0x1365), 0x11);
}

#[test]
fn test_cpu_sta_absolute_y() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STA_ABSOLUTE_Y, 0x25, 0x12, CPU::STA_ABSOLUTE_Y, 0xAA, 0x12]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0xF;
    cpu.a = 0xAB;
    cpu.process(5);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(5, cpu.cycles_run);
    cpu.y = 0xBB;
    cpu.a = 0x11;
    cpu.process(5);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(10, cpu.cycles_run);
    assert_eq!(mem.read8(0x1234), 0xAB);
    assert_eq!(mem.read8(0x1365), 0x11);
}

#[test]
fn test_cpu_sta_indirect_x() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STA_INDIRECT_X, 0x25, CPU::STA_INDIRECT_X, 0xAA]);
    mem.write16(0x34, 0x1234);
    mem.write16(0x65, 0x1365);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xF;
    cpu.a = 0xAB;
    // Flags should not affect the instruction
    cpu.status = CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW;
    cpu.process(6);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW, cpu.status);
    assert_eq!(6, cpu.cycles_run);
    // this will cause a wrap around as the addres will be higher than 255
    cpu.x = 0xBB;
    cpu.a = 0x11;
    cpu.status = 0;
    cpu.process(6);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(12, cpu.cycles_run);
    assert_eq!(mem.read8(0x1234), 0xAB);
    assert_eq!(mem.read8(0x1365), 0x11);
}

#[test]
fn test_cpu_sta_indirect_y() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STA_INDIRECT_Y, 0x25, CPU::STA_INDIRECT_Y, 0xAA]);
    mem.write16(0x25, 0x1225);
    mem.write16(0xAA, 0x12AA);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0xF;
    cpu.a = 0xAB;
    // Flags should not affect the instruction
    cpu.status = CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW;
    cpu.process(6);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(CPU::FLAG_CARRY | CPU::FLAG_OVERFLOW, cpu.status);
    assert_eq!(6, cpu.cycles_run);
    cpu.y = 0xBB;
    cpu.a = 0x11;
    cpu.status = 0;
    cpu.process(6);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(12, cpu.cycles_run);
    assert_eq!(mem.read8(0x1234), 0xAB);
    assert_eq!(mem.read8(0x1365), 0x11);
}

#[test]
fn test_cpu_stx_zero_page() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STX_ZERO, 0xCA, CPU::STX_ZERO, 0xCB]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xFE;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(3, cpu.cycles_run);
    cpu.x = 0x12;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(6, cpu.cycles_run);
    assert_eq!(mem.read8(0xCA), 0xFE);
    assert_eq!(mem.read8(0xCB), 0x12);
}

#[test]
fn test_cpu_stx_zero_page_y() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STX_ZERO_Y, 0x80, CPU::STX_ZERO_Y, 0x80]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0x0F;
    cpu.x = 0xFE;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.y = 0xFF;
    cpu.x = 0xA;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
    assert_eq!(mem.read8(0x8F), 0xFE);
    assert_eq!(mem.read8(0x7F), 0xA);
}

#[test]
fn test_cpu_stx_absolute() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STX_ABSOLUTE, 0x25, 0x12, CPU::STX_ABSOLUTE, 0xAA, 0x12]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xFE;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0x12;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
    assert_eq!(mem.read8(0x1225), 0xFE);
    assert_eq!(mem.read8(0x12AA), 0x12);
}

#[test]
fn test_cpu_sty_zero_page() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STY_ZERO, 0xCA, CPU::STY_ZERO, 0xCB]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0xFE;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(3, cpu.cycles_run);
    cpu.y = 0x12;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(6, cpu.cycles_run);
    assert_eq!(mem.read8(0xCA), 0xFE);
    assert_eq!(mem.read8(0xCB), 0x12);
}

#[test]
fn test_cpu_sty_zero_page_x() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STY_ZERO_X, 0x80, CPU::STY_ZERO_X, 0x80]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0x0F;
    cpu.y = 0xFE;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.x = 0xFF;
    cpu.y = 0xA;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
    assert_eq!(mem.read8(0x8F), 0xFE);
    assert_eq!(mem.read8(0x7F), 0xA);
}

#[test]
fn test_cpu_sty_absolute() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::STY_ABSOLUTE, 0x25, 0x12, CPU::STY_ABSOLUTE, 0xAA, 0x12]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0xFE;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(4, cpu.cycles_run);
    cpu.y = 0x12;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(8, cpu.cycles_run);
    assert_eq!(mem.read8(0x1225), 0xFE);
    assert_eq!(mem.read8(0x12AA), 0x12);
}

#[test]
fn test_trans_a_to_x() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::TRANS_A_TO_X, CPU::TRANS_A_TO_X]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.a = 0xFE;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(0xFE, cpu.x);
    assert_eq!(2, cpu.cycles_run);
    cpu.a = 0;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(CPU::FLAG_ZERO, cpu.status);
    assert_eq!(0, cpu.y);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_trans_a_to_y() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::TRANS_A_TO_Y, CPU::TRANS_A_TO_Y]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.a = 0xFE;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(0xFE, cpu.y);
    assert_eq!(2, cpu.cycles_run);
    cpu.a = 0;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(CPU::FLAG_ZERO, cpu.status);
    assert_eq!(0, cpu.y);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_trans_x_to_a() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::TRANS_X_TO_A, CPU::TRANS_X_TO_A]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xFE;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(0xFE, cpu.a);
    assert_eq!(2, cpu.cycles_run);
    cpu.x = 0;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(CPU::FLAG_ZERO, cpu.status);
    assert_eq!(0, cpu.a);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_trans_y_to_a() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::TRANS_Y_TO_A, CPU::TRANS_Y_TO_A]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.y = 0xFE;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(0xFE, cpu.a);
    assert_eq!(2, cpu.cycles_run);
    cpu.y = 0;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(CPU::FLAG_ZERO, cpu.status);
    assert_eq!(0, cpu.a);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_trans_sp_to_x() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::TRANS_SP_TO_X, CPU::TRANS_SP_TO_X]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.sp = 0xFE;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(0xFE, cpu.x);
    assert_eq!(2, cpu.cycles_run);
    cpu.sp = 0;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(CPU::FLAG_ZERO, cpu.status);
    assert_eq!(0, cpu.x);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_trans_x_to_sp() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::TRANS_X_TO_SP, CPU::TRANS_X_TO_SP]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.x = 0xFE;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(0xFE, cpu.sp);
    assert_eq!(2, cpu.cycles_run);
    cpu.x = 0;
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(0, cpu.sp);
    assert_eq!(4, cpu.cycles_run);
}

#[test]
fn test_push_a_to_sp() {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::PUSH_A_TO_SP, CPU::PUSH_A_TO_SP]);
    let mut cpu = CPU::new(&mut mem);
    cpu.reset();
    cpu.a = 0xFE;
    cpu.status = CPU::FLAG_ZERO | CPU::FLAG_CARRY;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(CPU::FLAG_ZERO | CPU::FLAG_CARRY, cpu.status);
    assert_eq!(STACK_OFFSET_START - 1, cpu.sp);
    assert_eq!(3, cpu.cycles_run);
    cpu.a = 0x12;
    cpu.status = 0;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0, cpu.status);
    assert_eq!(STACK_OFFSET_START - 2, cpu.sp);
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
    cpu.status = CPU::FLAG_ZERO | CPU::FLAG_CARRY;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(CPU::FLAG_ZERO | CPU::FLAG_CARRY, cpu.status);
    assert_eq!(STACK_OFFSET_START - 1, cpu.sp);
    assert_eq!(3, cpu.cycles_run);
    cpu.status = CPU::FLAG_INTERRUPT;
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(CPU::FLAG_INTERRUPT, cpu.status);
    assert_eq!(STACK_OFFSET_START - 2, cpu.sp);
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
    cpu.sp = STACK_OFFSET_START - 2;
    cpu.status = CPU::FLAG_INTERRUPT;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 1, cpu.pc);
    assert_eq!(CPU::FLAG_INTERRUPT, cpu.status);
    assert_eq!(STACK_OFFSET_START - 1, cpu.sp);
    assert_eq!(0x12, cpu.a);
    assert_eq!(4, cpu.cycles_run);
    cpu.a = 0;
    cpu.status = 0;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.status);
    assert_eq!(STACK_OFFSET_START, cpu.sp);
    assert_eq!(8, cpu.cycles_run);
    assert_eq!(0xFE, cpu.a);
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
