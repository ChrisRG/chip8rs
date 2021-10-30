use std::fs;

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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    RegV(u8),
    Number(u16),
    Clear,
    Ret,
    Jump,
    Call,
    Load,
    SkipEq,
    SkipNotEq,
    SkipPress,
    SkipNotPress,
    Add,
    Or,
    And,
    Xor,
    Sub,
    ShiftRight,
    SubNotBorrow,
    ShiftLeft,
    Random,
    Draw,
    DelayTimer,
    SoundTimer,
    Key,
    RegI,
    Sprite,
    Bcd,
    Unrecognized(String),
    Eof,
}

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            line: self.line,
        });
    }

    pub fn tokens(self) -> Vec<Token> {
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            ' ' | '\t' | '\r' | ',' => {}
            '\n' => self.line += 1,
            'v' | 'V' if self.peek().is_digit(10) => self.register_v(),
            x if x.is_ascii_alphabetic() => self.symbol(),
            x if x.is_digit(10) => self.number(),
            _ => println!("Unrecognized character {}", c),
        }
    }

    fn advance(&mut self) -> char {
        let cursor = self.source[self.current];
        self.current += 1;
        cursor
    }

    fn add_token(&mut self, token_type: TokenType) {
        let token = Token {
            token_type,
            line: self.line,
        };
        self.tokens.push(token);
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn symbol(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let token = match text.to_uppercase().as_str() {
            "CLS" => TokenType::Clear,
            "RET" => TokenType::Ret,
            "JP" => TokenType::Jump,
            "CALL" => TokenType::Call,
            "SE" => TokenType::SkipEq,
            "SNE" => TokenType::SkipNotEq,
            "LD" => TokenType::Load,
            "ADD" => TokenType::Add,
            "OR" => TokenType::Or,
            "XOR" => TokenType::Xor,
            "SUB" => TokenType::Sub,
            "SHR" => TokenType::ShiftRight,
            "SUBN" => TokenType::SubNotBorrow,
            "SHL" => TokenType::ShiftLeft,
            "RND" => TokenType::Random,
            "DRW" => TokenType::Draw,
            "SKP" => TokenType::SkipPress,
            "SKNP" => TokenType::SkipNotPress,
            "I" => TokenType::RegI,
            "K" => TokenType::Key,
            "ST" => TokenType::SoundTimer,
            "DT" => TokenType::DelayTimer,
            "B" => TokenType::Bcd,
            "F" => TokenType::Sprite,
            _ => TokenType::Unrecognized(text),
        };
        self.add_token(token);
    }

    // TODO: Essentially the same function as number, simplify
    fn register_v(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        let num_string: String = self.source[self.start + 1..self.current].iter().collect();
        let parsed_num = num_string.parse().unwrap();

        self.add_token(TokenType::RegV(parsed_num));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        let num_string: String = self.source[self.start..self.current].iter().collect();
        let parsed_num = num_string.parse().unwrap();

        self.add_token(TokenType::Number(parsed_num));
    }
}
