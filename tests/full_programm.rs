use emulator6502::*;
use std::fs;

/// The binary used here was assembled from the code in
/// https://github.com/Klaus2m5/6502_65C02_functional_tests
/// with the following parameters: -l -m -w -h0
/// and decimal mode disabled
///
/// As it stays in a loop even on success and must be run with a
/// debugger, do not run it in normal testing mode
#[test]
#[ignore]
fn test_full_programm_all_opcodes() {
    let mut mem = Mem::new();
    mem.reset();
    let programm = fs::read("tests/6502_functional_test.bin").expect("Unable to read file");
    // The programm must be written from address 0xA
    mem.load_programm_at_from_vec(0xA, programm);
    let mut cpu = Cpu::new(&mut mem);
    cpu.reset();
    // The PC must start at 0x400
    cpu.pc = 0x400;
    cpu.process(0);
}
