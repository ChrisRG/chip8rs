# CHIP-8rs
A CHIP-8 virtual machine written in Rust.

## Installation
* Install Rust using the [official installation guide](https://www.rust-lang.org/learn/get-started), which will also install the `cargo` build system.
* Clone the GitHub repository and build the emulator:

```
$ git clone https://github.com/ChrisRG/chip8rs
$ cd chip8rs
$ cargo build --release
```
The binary can then be found in `./target/release`.

## Usage

#### Emulator 
Running a ROM in the emulator
    
```$ ./target/release/chip8rs /roms/pong.ch8```

#### Disassembler
Disassembling a ROM: a new `.chasm` file will be created in the same directory.

```$ ./target/release/chip8rs /roms/pong.ch8 -d```

#### Assembler
Assembling a `.chasm` file: a new file will be created in the same directory named `<file_name>_a.ch8`

```$ ./target/release/chip8rs /roms/pong.chasm -a```


## Additional resources
* [How to write an emulator (CHIP-8 interpreter)](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/) 
* [CHIP-8 Technical Reference](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference)
* [CHIP-8 Instruction Set](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set)
