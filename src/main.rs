mod assembler;
mod bus;
mod chip8;
mod cpu;
mod disassembler;
mod display;
mod font;
mod ram;
use crate::chip8::Chip8;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let rom_file = matches.value_of("INPUT").unwrap().to_string();
    println!("[Starting CHIP-8 emulator]");
    let mut chip8 = Chip8::new(rom_file);

    if matches.is_present("disassemble") {
        chip8.disassemble();
    } else if matches.is_present("assemble") {
        chip8.assemble();
    } else {
        chip8.run();
    }
}
