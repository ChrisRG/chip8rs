use std::fs::File;
use std::io::Write;
use std::path::Path;

use Instruction::*;

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    line: usize,
    address: usize,
}

#[derive(Debug)]
enum TokenType {
    Instruction(Instruction),
    VRegister(u8),
    IRegister,
    DelayTimer,
    SoundTimer,
    Key,
    FSprite,
    NumLiteral(u16),
    Invalid,
}

#[derive(Debug)]
enum Instruction {
    Clear,
    Return,
    Jump,
    Call,
    SkipEqual,
    SkipNotEqual,
    Load,
    Add,
    Or,
    And,
    Xor,
    Sub,
    SubReverse,
    ShiftRight,
    ShiftLeft,
    Rand,
    Draw,
    SkipKeyPress,
    SkipNotKeyPress,
    BinCodedDecimal,
}

pub struct Assembler {
    source: String,
    tokens: Vec<Vec<Token>>,
    line: usize,
    address: usize,
}

impl Assembler {
    pub fn new(source_file: String) -> Self {
        Self {
            source: source_file,
            tokens: Vec::new(),
            line: 1,
            address: 0x200,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        println!("Running assembler");
        self.scan_lines();
        self.write_file().unwrap();
        Ok(())
    }

    fn scan_lines(&mut self) {
        for line in self.source.lines() {
            let line_tokens = self.scan_line(line.to_string());
            self.tokens.push(line_tokens);
            self.line += 1;
            self.address += 2;
        }
    }

    fn scan_line(&self, line: String) -> Vec<Token> {
        let words: Vec<&str> = line
            .split(&[' ', ','][..])
            .filter(|&elem| !elem.is_empty())
            .collect();
        let mut tokens = Vec::new();

        for word in words {
            if let Ok(digit) = word.parse::<u16>() {
                tokens.push(Token {
                    token_type: TokenType::NumLiteral(digit),
                    line: self.line,
                    address: self.address,
                });
            } else {
                let token_type = match word {
                    "JP" => TokenType::Instruction(Jump),
                    "CALL" => TokenType::Instruction(Call),
                    "I" => TokenType::IRegister,
                    "RET" => TokenType::Instruction(Return),
                    "CLS" => TokenType::Instruction(Clear),
                    "SE" => TokenType::Instruction(SkipEqual),
                    "SNE" => TokenType::Instruction(SkipNotEqual),
                    "ADD" => TokenType::Instruction(Add),
                    "LD" => TokenType::Instruction(Load),
                    "AND" => TokenType::Instruction(And),
                    "OR" => TokenType::Instruction(Or),
                    "XOR" => TokenType::Instruction(Xor),
                    "SUB" => TokenType::Instruction(Sub),
                    "SUBN" => TokenType::Instruction(SubReverse),
                    "SHR" => TokenType::Instruction(ShiftRight),
                    "SHL" => TokenType::Instruction(ShiftLeft),
                    "RND" => TokenType::Instruction(Rand),
                    "SKP" => TokenType::Instruction(SkipKeyPress),
                    "SKNP" => TokenType::Instruction(SkipNotKeyPress),
                    "DRW" => TokenType::Instruction(Draw),
                    "DT" => TokenType::DelayTimer,
                    "K" => TokenType::Key,
                    "ST" => TokenType::SoundTimer,
                    "BCD" => TokenType::Instruction(BinCodedDecimal),
                    "F" => TokenType::FSprite,
                    word => {
                        if word.chars().nth(0).unwrap() == 'V' && !word[1..].is_empty() {
                            TokenType::VRegister(5)
                        } else {
                            TokenType::Invalid
                        }
                    }
                };
                tokens.push(Token {
                    token_type,
                    line: self.line,
                    address: self.address,
                });
            }
        }
        tokens
    }

    // fn parse_opcodes(&self) {
    //     for instruction in self.tokens {}
    // }

    fn write_file(&self) -> std::io::Result<()> {
        let path = Path::new("./src/roms/test.ch8");
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(e) => panic!("Couldn't create {}: {}", display, e),
            Ok(file) => file,
        };

        writeln!(file, "{:?}", self.tokens)?;
        Ok(())
    }
}
