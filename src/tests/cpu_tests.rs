use super::*;

fn build_cpu() -> Cpu {
    let mut cpu = Cpu::new();
    cpu.pc = 0x200;
    cpu
}
#[test]
fn test_initialize() {
    let cpu = build_cpu();
    assert_eq!(cpu.pc, 0x200);
    assert_eq!(cpu.sp, 0);
    assert_eq!(cpu.stack, [0; 16]);
}

#[test]
fn test_load_rom() {
    let mut cpu = build_cpu();
    let test_rom = vec![0xCC; 64];
    // Test load 64 bytes of data
    cpu.load_rom(test_rom);
    assert_eq!(cpu.ram.read_byte(512), 0xCC);
    assert_eq!(cpu.ram.read_byte(575), 0xCC);
}
