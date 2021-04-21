use criterion::{criterion_group, criterion_main, Criterion};
use emulator6502::*;

fn test_cpu_lda_immediate(cpu: &mut Cpu) {
    cpu.reset();
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xCA, cpu.regs[Cpu::REG_A]);
    assert_eq!(Cpu::FLAG_NEGATIVE, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(2, cpu.cycles_run);
    cpu.process(2);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x0, cpu.regs[Cpu::REG_A]);
    assert_eq!(Cpu::FLAG_ZERO, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(4, cpu.cycles_run);
}

fn _test_cpu_lda_immediate2(cpu: &mut Cpu) {
    cpu.reset();
    cpu.process2(2);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xCA, cpu.regs[Cpu::REG_A]);
    assert_eq!(Cpu::FLAG_NEGATIVE, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(2, cpu.cycles_run);
    cpu.process2(2);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0x0, cpu.regs[Cpu::REG_A]);
    assert_eq!(Cpu::FLAG_ZERO, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(4, cpu.cycles_run);
}

fn test_cpu_ldy_absolute_x(cpu: &mut Cpu) {
    cpu.reset();
    cpu.regs[Cpu::REG_X] = 0xF;
    cpu.process(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0xAB, cpu.regs[Cpu::REG_Y]);
    assert_eq!(Cpu::FLAG_NEGATIVE, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(4, cpu.cycles_run);
    cpu.regs[Cpu::REG_X] = 0xBB;
    cpu.process(5);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0x11, cpu.regs[Cpu::REG_Y]);
    assert_eq!(0, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(9, cpu.cycles_run);
}

fn _test_cpu_ldy_absolute_x2(cpu: &mut Cpu) {
    cpu.reset();
    cpu.regs[Cpu::REG_X] = 0xF;
    cpu.process2(4);
    assert_eq!(RESET_EXEC_ADDRESS + 3, cpu.pc);
    assert_eq!(0xAB, cpu.regs[Cpu::REG_Y]);
    assert_eq!(Cpu::FLAG_NEGATIVE, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(4, cpu.cycles_run);
    cpu.regs[Cpu::REG_X] = 0xBB;
    cpu.process2(5);
    assert_eq!(RESET_EXEC_ADDRESS + 6, cpu.pc);
    assert_eq!(0x11, cpu.regs[Cpu::REG_Y]);
    assert_eq!(0, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(9, cpu.cycles_run);
}

fn test_cpu_ora_zero_page(cpu: &mut Cpu) {
    cpu.reset();
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xFE, cpu.regs[Cpu::REG_A]);
    assert_eq!(Cpu::FLAG_NEGATIVE, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(3, cpu.cycles_run);
    cpu.process(3);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0xFF, cpu.regs[Cpu::REG_A]);
    assert_eq!(Cpu::FLAG_NEGATIVE, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(6, cpu.cycles_run);
}

fn test_cpu_ora_zero_page2(cpu: &mut Cpu) {
    cpu.reset();
    cpu.process2(3);
    assert_eq!(RESET_EXEC_ADDRESS + 2, cpu.pc);
    assert_eq!(0xFE, cpu.regs[Cpu::REG_A]);
    assert_eq!(Cpu::FLAG_NEGATIVE, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(3, cpu.cycles_run);
    cpu.process2(3);
    assert_eq!(RESET_EXEC_ADDRESS + 4, cpu.pc);
    assert_eq!(0xFF, cpu.regs[Cpu::REG_A]);
    assert_eq!(Cpu::FLAG_NEGATIVE, cpu.regs[Cpu::REG_STAT]);
    assert_eq!(6, cpu.cycles_run);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[Cpu::LDA_IMMEDIATE, 0xCA, Cpu::LDA_IMMEDIATE, 0x0]);
    let mut cpu = Cpu::new(&mut mem);
    let ref_cpu = &mut cpu;
    c.bench_function("test_Cpu_lda_immediate", |b| b.iter(|| test_cpu_lda_immediate(ref_cpu)));
    // c.bench_function("test_Cpu_lda_immediate2", |b| b.iter(|| test_Cpu_lda_immediate2(ref_cpu)));
}

fn criterion_benchmark2(c: &mut Criterion) {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[Cpu::LDY_ABSOLUTE_X, 0x25, 0x12, Cpu::LDY_ABSOLUTE_X, 0xAA, 0x12]);
    mem.write8(0x1234, 0xAB);
    mem.write8(0x1365, 0x11);
    let mut cpu = Cpu::new(&mut mem);
    let ref_cpu = &mut cpu;
    c.bench_function("test_Cpu_ldy_absolute_x", |b| b.iter(|| test_cpu_ldy_absolute_x(ref_cpu)));
    // c.bench_function("test_Cpu_ldy_absolute_x2", |b| b.iter(|| test_Cpu_ldy_absolute_x2(ref_cpu)));
}

fn criterion_benchmark3(c: &mut Criterion) {
    let mut mem = Mem::new();
    mem.reset();
    mem.load_programm(&[Cpu::ORA_ZERO, 0xCA, Cpu::ORA_ZERO, 0xCB]);
    mem.write8(0xCA, 0xFE);
    mem.write8(0xCB, 0x1);
    let mut cpu = Cpu::new(&mut mem);
    let ref_cpu = &mut cpu;
    c.bench_function("test_Cpu_ora_zero_page", |b| b.iter(|| test_cpu_ora_zero_page(ref_cpu)));
    c.bench_function("test_Cpu_ora_zero_page2", |b| b.iter(|| test_cpu_ora_zero_page2(ref_cpu)));
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark2, criterion_benchmark3);
criterion_main!(benches);
