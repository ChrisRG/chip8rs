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
