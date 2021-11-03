mod lexer;
mod parser;
mod token;
use std::fs;

use crate::assembler::lexer::Lexer;

pub struct Assembler {
    source_path: String,
    source_code: String,
}

impl Assembler {
    pub fn new(source_path: String) -> Self {
        let source_code = fs::read_to_string(&source_path).expect("Unable to read file.");

        Self {
            source_code,
            source_path,
        }
    }

    pub fn run(&self) {
        println!("Running assembler");
        let mut lexer = Lexer::new(&self.source_code);
        lexer.scan_tokens();
        let tokens = lexer.tokens();
        dbg!(tokens);
    }
}
