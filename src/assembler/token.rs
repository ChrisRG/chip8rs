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
