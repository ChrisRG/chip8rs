#[allow(dead_code)]
mod assembler;
mod bus;
mod chip8;
mod cpu;
mod disassembler;
mod display;
mod font;
mod ram;
use crate::{assembler::Assembler, chip8::Chip8};
use crate::disassembler::Disassembler;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let source_file = matches
        .value_of("INPUT")
        .expect("Unable to read file.")
        .to_string();

    if matches.is_present("disassemble") {
        let disassembler = Disassembler::new(source_file);
        disassembler.run();
    } else if matches.is_present("assemble") {
        let assembler = Assembler::new(source_file);
        assembler.run();
    } else {
        let mut chip8 = Chip8::new(source_file);
        chip8.run();
    };
}
