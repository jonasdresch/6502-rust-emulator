use emulator6502::*;

#[test]
fn test_cpu_reset_vector() {
    let mut mem = Mem::new();
    mem.reset();
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    assert_eq!(RESET_EXEC_ADDRESS, cpu.pc)
}

#[test]
fn test_mem_read_limits_ok() {
    let mut mem = Mem::new();
    mem.reset();
    assert_eq!(0, mem.read8(0));
    assert_eq!(0, mem.read8(65535));
}

#[test]
#[should_panic(expected = "memory access out ouf bounds")]
fn test_mem_read_limits_nok() {
    let mut mem = Mem::new();
    mem.reset();
    assert_eq!(0, mem.read8(65536));
}

#[test]
#[should_panic(expected = "memory access out ouf bounds")]
fn test_mem_write_limits_nok() {
    let mut mem = Mem::new();
    mem.reset();
    mem.write8(65536, 1)
}

#[test]
fn test_mem_read_reset_vector_ok() {
    let mut mem = Mem::new();
    mem.reset();
    assert_eq!(0xFC, mem.read8(RESET_VECTOR_ADDR));
    assert_eq!(0xE2, mem.read8(RESET_VECTOR_ADDR + 1));
    assert_eq!(0, mem.read8(667));
}

#[test]
fn test_mem_write_read_ok() {
    let mut mem = Mem::new();
    mem.reset();
    mem.write8(666, 200);
    assert_eq!(200, mem.read8(666));
    assert_eq!(0, mem.read8(665));
    assert_eq!(0, mem.read8(667));
}
