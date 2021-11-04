use super::token::{Token, TokenType};

pub(crate) struct Lexer {
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
        while self.current != self.source.len() {
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

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            ' ' | '\t' | '\r' | ',' => {}
            '\n' => self.line += 1,
            ';' => self.comment(),
            '.' => self.directive(),
            'v' | 'V' if self.peek().is_digit(10) => self.register_v(),
            ':' => self.add_token(TokenType::Assign),
            x if x.is_digit(10) => self.number(),
            x if x.is_ascii_alphabetic() => self.symbol(),
            _ => self.add_token(TokenType::Error(format!("Unrecognized token `{}` line {}", c, self.line))),
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
        self.source[self.current]
    }

    fn comment(&mut self) {
        while self.peek() != '\n' {
            self.advance();
        }
    }

    fn directive(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let directive_str: String = self.source[self.start + 1 .. self.current].iter().collect();

        self.add_token(TokenType::Directive(directive_str));
    }

    fn scan_number(&mut self, start: usize) -> u16 {
        while self.peek().is_digit(10) {
            self.advance();
        }

        let num_string: String = self.source[start..self.current].iter().collect();
        num_string.parse().unwrap()
    }

    fn register_v(&mut self) {
        let parsed_num = self.scan_number(self.start + 1);

        self.add_token(TokenType::RegV(parsed_num as u8));
    }

    fn number(&mut self) {
        let parsed_num = self.scan_number(self.start);

        self.add_token(TokenType::Number(parsed_num));
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
            _ => TokenType::Label(text),
        };
        self.add_token(token);
    }
}
