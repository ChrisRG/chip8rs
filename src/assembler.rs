use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use hex;

const START_ROM: usize = 512; // 0x200

struct ParseError {
    line: usize,
    msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Line {}] {}", self.line, self.msg)
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: String,
    bytes: Vec<u8>,
    address: u16,
}

impl Instruction {
    pub fn new(opcode: String, bytes: Vec<u8>, address: u16) -> Self {
        Self {
            opcode,
            bytes,
            address,
        }
    }
}

pub struct Assembler {
    source: String,
    instructions: Vec<Instruction>,
    line: usize,
    address: usize,
}

impl Assembler {
    pub fn new(source_file: String) -> Self {
        Self {
            source: source_file,
            instructions: Vec::new(),
            line: 1,
            address: 0x200,
        }
    }

    pub fn run(&mut self) {
        println!("Running assembler");
        self.parse_lines();
        self.write_file().unwrap();
    }

    fn parse_lines(&mut self) {
        for line in self.source.lines() {
            if let Ok(opcode) = self.parse_instruction(line) {
                self.instructions.push(opcode);
                self.line += 1;
                self.address += 2;
            }
        }
    }

    fn parse_instruction(&self, line: &str) -> Result<Instruction, ParseError> {
        let words: Vec<&str> = line
            .split(&[' ', ','][..])
            .filter(|&elem| !elem.is_empty())
            .collect();

        let opcode = match words[0] {
            "JP" if words.len() > 1 => self.parse_jp(&words[1..])?,
            "CALL" if words.len() > 1 => self.parse_call(&words[1..])?,
            "RET" if words.len() == 1 => format!("00EE"),
            "CLS" if words.len() == 1 => format!("00E0"),
            "SE" if words.len() == 3 => self.parse_se(&words[1..])?,
            "SNE" if words.len() == 3 => self.parse_sne(&words[1..])?,
            "LD" if words.len() > 1 => self.parse_ld(&words[1..])?,
            "ADD" if words.len() > 1 => self.parse_add(&words[1..])?,
            "OR" if words.len() == 3 => self.parse_or(&words[1..])?,
            "AND" if words.len() == 3 => self.parse_and(&words[1..])?,
            "XOR" if words.len() == 3 => self.parse_xor(&words[1..])?,
            "SUB" if words.len() == 3 => self.parse_sub(&words[1..])?,
            "SHR" if words.len() == 3 => self.parse_shr(&words[1..])?,
            "SUBN" if words.len() == 3 => self.parse_subn(&words[1..])?,
            "SHL" if words.len() == 3 => self.parse_shl(&words[1..])?,
            "RND" if words.len() == 3 => self.parse_rnd(&words[1..])?,
            "DRW" if words.len() == 4 => self.parse_drw(&words[1..])?,
            "SKP" if words.len() == 2 => self.parse_skp(&words[1..])?,
            "SKNP" if words.len() == 2 => self.parse_sknp(&words[1..])?,
            _ => format!("0000"),
        };
        Ok(self.build_instruction(opcode, self.line))
    }

    fn build_instruction(&self, opcode: String, line: usize) -> Instruction {
        // println!("[{}] {}", self.line, opcode);
        let mut bytes = [0u8; 2];
        hex::decode_to_slice(&opcode, &mut bytes as &mut [u8])
            .expect("Failed to encode instruction");
        let address = line + START_ROM - 1;
        Instruction::new(opcode, bytes.to_vec(), address as u16)
    }

    fn parse_digit(&self, word: &str) -> Option<u16> {
        match word.parse::<u16>() {
            Ok(num) => Some(num),
            _ => None,
        }
    }

    fn parse_register(&self, word: &str) -> Option<u16> {
        let chars: Vec<char> = word.chars().collect();
        match chars[0] {
            // If first char is 'V', parse the rest of the word as a digit
            'V' if chars.len() > 1 => self.parse_digit(&word[1..]),
            _ => None,
        }
    }

    fn parse_jp(&self, words: &[&str]) -> Result<String, ParseError> {
        match words.len() {
            // 1nnn
            1 => match self.parse_digit(words[0]) {
                Some(nnn) => Ok(format!("1{:x}", nnn)),
                _ => Err(ParseError {
                    line: self.line,
                    msg: format!("Unable parse to parse jump address {}", words[0]),
                }),
            },
            // Bnnn
            2 => match self.parse_digit(words[1]) {
                Some(nnn) => Ok(format!("B{:x}", nnn)),
                _ => Err(ParseError {
                    line: self.line,
                    msg: format!("Unable parse to parse jump address {}", words[0]),
                }),
            },
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable parse to parse jump address {}", words[0]),
            }),
        }
    }

    fn parse_call(&self, words: &[&str]) -> Result<String, ParseError> {
        // 2nnn
        if let Some(nnn) = self.parse_digit(words[0]) {
            Ok(format!("2{:x}", nnn))
        } else {
            Err(ParseError {
                line: self.line,
                msg: format!("Unable parse to parse call instruction {}", words.join(" ")),
            })
        }
    }

    fn parse_sne(&self, words: &[&str]) -> Result<String, ParseError> {
        let regs: Vec<Option<u16>> = words.iter().map(|word| self.parse_register(word)).collect();
        match (regs[0], regs[1]) {
            (Some(x), Some(y)) => Ok(format!("9{:x}{:x}0", x, y)),
            (Some(x), None) => match self.parse_digit(words[1]) {
                Some(kk) => Ok(format!("4{:x}{:02x}", x, kk)),
                _ => Err(ParseError {
                    line: self.line,
                    msg: format!("Unable to parse SNE Vx, kk instruction {}", words.join(" ")),
                }),
            },
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse SNE Vx, Vy instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_se(&self, words: &[&str]) -> Result<String, ParseError> {
        let regs: Vec<Option<u16>> = words.iter().map(|word| self.parse_register(word)).collect();
        match (regs[0], regs[1]) {
            (Some(x), Some(y)) => Ok(format!("5{:x}{:x}0", x, y)),
            (Some(x), None) => match self.parse_digit(words[1]) {
                Some(kk) => Ok(format!("3{:x}{:02x}", x, kk)),
                _ => Err(ParseError {
                    line: self.line,
                    msg: format!("Unable to parse SE Vx, kk instruction {}", words.join(" ")),
                }),
            },
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse SE Vx, Vy instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_ld(&self, words: &[&str]) -> Result<String, ParseError> {
        match words[0] {
            "I" => {
                match self.parse_register(words[1]) {
                    // Fx55 I, Vx
                    Some(x) => return Ok(format!("F{:x}55", x)),
                    None => {
                        match self.parse_digit(words[1]) {
                            // Annn I, addr
                            Some(nnn) => return Ok(format!("A{:x}", nnn)),
                            None => {
                                return Err(ParseError {
                                    line: self.line,
                                    msg: format!(
                                        "Unable to parse LD I instruction {}",
                                        words.join(" ")
                                    ),
                                })
                            }
                        }
                    }
                }
            }
            "DT" => {
                match self.parse_register(words[1]) {
                    // Fx15 Dt, Vx
                    Some(x) => return Ok(format!("F{:x}15", x)),
                    None => {
                        return Err(ParseError {
                            line: self.line,
                            msg: format!("Unable to parse LD DT instruction {}", words.join(" ")),
                        })
                    }
                }
            }
            "ST" => {
                match self.parse_register(words[1]) {
                    // Fx18 ST, Vx
                    Some(x) => return Ok(format!("F{:x}18", x)),
                    None => {
                        return Err(ParseError {
                            line: self.line,
                            msg: format!("Unable to parse LD ST instruction {}", words.join(" ")),
                        })
                    }
                }
            }
            "F" => {
                match self.parse_register(words[1]) {
                    // Fx29 F, Vx
                    Some(x) => return Ok(format!("F{:x}29", x)),
                    None => {
                        return Err(ParseError {
                            line: self.line,
                            msg: format!("Unable to parse LD F instruction {}", words.join(" ")),
                        })
                    }
                }
            }
            "B" => {
                match self.parse_register(words[1]) {
                    // Fx33 B, Vx
                    Some(x) => return Ok(format!("F{:x}33", x)),
                    None => {
                        return Err(ParseError {
                            line: self.line,
                            msg: format!("Unable to parse LD B instruction {}", words.join(" ")),
                        })
                    }
                }
            }
            _ => {
                if let Some(x) = self.parse_register(words[0]) {
                    match words[1] {
                        // Fx07 Vx, Dt
                        "DT" => return Ok(format!("F{:x}07", x)),
                        // Fx0A Vx, K
                        "K" => return Ok(format!("F{:x}0A", x)),
                        // Fx65 Vx, I
                        "I" => return Ok(format!("F{:x}65", x)),
                        _ => match self.parse_register(words[1]) {
                            // 8xy0 Vx, Vy
                            Some(y) => return Ok(format!("8{:x}{:x}0", x, y)),
                            None => {
                                match self.parse_digit(words[1]) {
                                    // 6xkk Vx, byte
                                    Some(kk) => return Ok(format!("6{:x}{:02x}", x, kk)),
                                    _ => {
                                        return Err(ParseError {
                                            line: self.line,
                                            msg: format!(
                                                "Unable to parse LD Vx kk instruction {}",
                                                words.join(" ")
                                            ),
                                        })
                                    }
                                }
                            }
                        },
                    }
                } else {
                    return Err(ParseError {
                        line: self.line,
                        msg: format!("Unable to parse LD Vx instruction {}", words.join(" ")),
                    });
                }
            }
        }
    }

    fn parse_or(&self, words: &[&str]) -> Result<String, ParseError> {
        let regs: Vec<Option<u16>> = words.iter().map(|word| self.parse_register(word)).collect();
        match (regs[0], regs[1]) {
            (Some(x), Some(y)) => Ok(format!("8{:x}{:x}1", x, y)),
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse OR instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_and(&self, words: &[&str]) -> Result<String, ParseError> {
        let regs: Vec<Option<u16>> = words.iter().map(|word| self.parse_register(word)).collect();
        match (regs[0], regs[1]) {
            (Some(x), Some(y)) => Ok(format!("8{:x}{:x}2", x, y)),
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse AND instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_xor(&self, words: &[&str]) -> Result<String, ParseError> {
        let regs: Vec<Option<u16>> = words.iter().map(|word| self.parse_register(word)).collect();
        match (regs[0], regs[1]) {
            (Some(x), Some(y)) => Ok(format!("8{:x}{:x}3", x, y)),
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse XOR instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_rnd(&self, words: &[&str]) -> Result<String, ParseError> {
        let x = self.parse_register(words[0]);
        let kk = self.parse_digit(words[1]);
        match (x, kk) {
            (Some(x), Some(kk)) => Ok(format!("C{:x}{:x}", x, kk)),
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse XOR instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_drw(&self, words: &[&str]) -> Result<String, ParseError> {
        let regs: Vec<Option<u16>> = words[0..=1]
            .iter()
            .map(|word| self.parse_register(word))
            .collect();
        let n = self.parse_digit(words[2]);
        match (regs[0], regs[1], n) {
            (Some(x), Some(y), Some(n)) => Ok(format!("D{:x}{:x}{:x}", x, y, n)),
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse XOR instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_skp(&self, words: &[&str]) -> Result<String, ParseError> {
        let reg: Vec<Option<u16>> = words.iter().map(|word| self.parse_register(word)).collect();
        match reg[0] {
            Some(x) => Ok(format!("E{:x}9E", x)),
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse XOR instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_sknp(&self, words: &[&str]) -> Result<String, ParseError> {
        let reg: Vec<Option<u16>> = words.iter().map(|word| self.parse_register(word)).collect();
        match reg[0] {
            Some(x) => Ok(format!("E{:x}A1", x)),
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse XOR instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_add(&self, words: &[&str]) -> Result<String, ParseError> {
        match words[0] {
            // Fx1E
            "I" => match self.parse_register(words[1]) {
                Some(x) => Ok(format!("F{:x}1E", x)),
                _ => Err(ParseError {
                    line: self.line,
                    msg: format!("Unable to parse ADD I, Vx instruction {}", words.join(" ")),
                }),
            },
            _ => {
                let regs: Vec<Option<u16>> =
                    words.iter().map(|word| self.parse_register(word)).collect();
                match (regs[0], regs[1]) {
                    // 8xy4
                    (Some(x), Some(y)) => Ok(format!("8{:x}{:x}4", x, y)),
                    // 7xkk
                    (Some(x), None) => match self.parse_digit(words[1]) {
                        Some(kk) => Ok(format!("7{:x}{:02x}", x, kk)),
                        _ => Err(ParseError {
                            line: self.line,
                            msg: format!(
                                "Unable to parse ADD Vx, kk instruction {}",
                                words.join(" ")
                            ),
                        }),
                    },
                    _ => Err(ParseError {
                        line: self.line,
                        msg: format!("Unable to parse ADD Vx, Vy instruction {}", words.join(" ")),
                    }),
                }
            }
        }
    }

    fn parse_sub(&self, words: &[&str]) -> Result<String, ParseError> {
        let regs: Vec<Option<u16>> = words.iter().map(|word| self.parse_register(word)).collect();
        match (regs[0], regs[1]) {
            (Some(x), Some(y)) => Ok(format!("8{:x}{:x}5", x, y)),
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse SUB instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_shr(&self, words: &[&str]) -> Result<String, ParseError> {
        let regs: Vec<Option<u16>> = words.iter().map(|word| self.parse_register(word)).collect();
        match (regs[0], regs[1]) {
            (Some(x), Some(y)) => Ok(format!("8{:x}{:x}6", x, y)),
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse SHR instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_subn(&self, words: &[&str]) -> Result<String, ParseError> {
        let regs: Vec<Option<u16>> = words.iter().map(|word| self.parse_register(word)).collect();
        match (regs[0], regs[1]) {
            (Some(x), Some(y)) => Ok(format!("8{:x}{:x}7", x, y)),
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse SUBN instruction {}", words.join(" ")),
            }),
        }
    }

    fn parse_shl(&self, words: &[&str]) -> Result<String, ParseError> {
        let regs: Vec<Option<u16>> = words.iter().map(|word| self.parse_register(word)).collect();
        match (regs[0], regs[1]) {
            (Some(x), Some(y)) => Ok(format!("8{:x}{:x}E", x, y)),
            _ => Err(ParseError {
                line: self.line,
                msg: format!("Unable to parse SHL instruction {}", words.join(" ")),
            }),
        }
    }

    fn write_file(&self) -> std::io::Result<()> {
        let path = Path::new("./src/roms/test.ch8");
        let mut file = match OpenOptions::new().write(true).create(true).open(path) {
            Err(e) => panic!("Couldn't create file {:?}: {}", path, e),
            Ok(file) => file,
        };
        for inst in self.instructions.iter() {
            let bytes = &*inst.bytes;
            file.write(bytes).unwrap();
        }
        Ok(())
    }
}
