mod cpu;
mod display;
mod ram;

use cpu::Cpu;

fn main() {
    println!("Starting CHIP-8 emulator...");
    // Setup graphics - TODO
    // Setup CPU, initialize
    let mut cpu = Cpu::new();
    // Load game into memory
    let test_rom = vec![0xA2, 0xF0, 0x20, 0x08, 0xFF, 0xFF];
    cpu.load_rom(test_rom);
    // Start emulation loop
    cpu.execute_cycle();
    cpu.execute_cycle();
}
