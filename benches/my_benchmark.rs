use criterion::{criterion_group, criterion_main, Criterion};
use emulator6502::*;

fn test_cpu_lda_immediate(cpu: &mut CPU) {
    cpu.reset();
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xCA, cpu.regs[CPU::REG_A]);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.regs[CPU::REG_STAT]);
    assert_eq!(2, cpu.cycles_run);
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x0, cpu.regs[CPU::REG_A]);
    assert_eq!(CPU::FLAG_ZERO, cpu.regs[CPU::REG_STAT]);
    assert_eq!(4, cpu.cycles_run);
}

fn test_cpu_lda_immediate2(cpu: &mut CPU) {
    cpu.reset();
    cpu.process2(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xCA, cpu.regs[CPU::REG_A]);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.regs[CPU::REG_STAT]);
    assert_eq!(2, cpu.cycles_run);
    cpu.process2(2);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x0, cpu.regs[CPU::REG_A]);
    assert_eq!(CPU::FLAG_ZERO, cpu.regs[CPU::REG_STAT]);
    assert_eq!(4, cpu.cycles_run);
}

fn test_cpu_ldy_absolute_x(cpu: &mut CPU) {
    cpu.reset();
    cpu.regs[CPU::REG_X] = 0xF;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0xAB, cpu.regs[CPU::REG_Y]);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.regs[CPU::REG_STAT]);
    assert_eq!(4, cpu.cycles_run);
    cpu.regs[CPU::REG_X] = 0xBB;
    cpu.process(5);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0x11, cpu.regs[CPU::REG_Y]);
    assert_eq!(0, cpu.regs[CPU::REG_STAT]);
    assert_eq!(9, cpu.cycles_run);
}

fn test_cpu_ldy_absolute_x2(cpu: &mut CPU) {
    cpu.reset();
    cpu.regs[CPU::REG_X] = 0xF;
    cpu.process2(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0xAB, cpu.regs[CPU::REG_Y]);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.regs[CPU::REG_STAT]);
    assert_eq!(4, cpu.cycles_run);
    cpu.regs[CPU::REG_X] = 0xBB;
    cpu.process2(5);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0x11, cpu.regs[CPU::REG_Y]);
    assert_eq!(0, cpu.regs[CPU::REG_STAT]);
    assert_eq!(9, cpu.cycles_run);
}

fn test_cpu_ora_zero_page(cpu: &mut CPU) {
    cpu.reset();
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xFE, cpu.regs[CPU::REG_A]);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.regs[CPU::REG_STAT]);
    assert_eq!(3, cpu.cycles_run);
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0xFF, cpu.regs[CPU::REG_A]);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.regs[CPU::REG_STAT]);
    assert_eq!(6, cpu.cycles_run);
}

fn test_cpu_ora_zero_page2(cpu: &mut CPU) {
    cpu.reset();
    cpu.process2(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xFE, cpu.regs[CPU::REG_A]);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.regs[CPU::REG_STAT]);
    assert_eq!(3, cpu.cycles_run);
    cpu.process2(3);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0xFF, cpu.regs[CPU::REG_A]);
    assert_eq!(CPU::FLAG_NEGATIVE, cpu.regs[CPU::REG_STAT]);
    assert_eq!(6, cpu.cycles_run);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDA_IMMEDIATE, 0xCA, CPU::LDA_IMMEDIATE, 0x0]);
    let mut cpu = CPU::new(&mut mem);
    let refcpu = &mut cpu;
    c.bench_function("test_cpu_lda_immediate", |b| b.iter(|| test_cpu_lda_immediate(refcpu)));
    // c.bench_function("test_cpu_lda_immediate2", |b| b.iter(|| test_cpu_lda_immediate2(refcpu)));
}

fn criterion_benchmark2(c: &mut Criterion) {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::LDY_ABSOLUTE_X, 0x25, 0x12, CPU::LDY_ABSOLUTE_X, 0xAA, 0x12]);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = CPU::new(&mut mem);
    let refcpu = &mut cpu;
    c.bench_function("test_cpu_ldy_absolute_x", |b| b.iter(|| test_cpu_ldy_absolute_x(refcpu)));
    // c.bench_function("test_cpu_ldy_absolute_x2", |b| b.iter(|| test_cpu_ldy_absolute_x2(refcpu)));
}

fn criterion_benchmark3(c: &mut Criterion) {
    let mut mem = MEM::new();
    mem.reset();
    mem.load_programm(&[CPU::ORA_ZERO, 0xCA, CPU::ORA_ZERO, 0xCB]);
    mem.write8(0xCA, 0xFE);
    mem.write8(0xCB, 0x1);
    let mut cpu = CPU::new(&mut mem);
    let refcpu = &mut cpu;
    c.bench_function("test_cpu_ora_zero_page", |b| b.iter(|| test_cpu_ora_zero_page(refcpu)));
    c.bench_function("test_cpu_ora_zero_page2", |b| b.iter(|| test_cpu_ora_zero_page2(refcpu)));
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark2, criterion_benchmark3);
criterion_main!(benches);
