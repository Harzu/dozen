#[derive(Debug, Clone)]
pub enum Token {
    WORD(String),
    FUNCTION,
    RETURN,
    NUMBER(i32),
    IF,
    LParen,
    RParen,
    LBrace,
    RBrace,
    WHITESPACE,
    SEMICOIN,
    UNKNOW,
}

pub trait ITokenizer {
    fn new() -> Self;
    fn next_token(&self, item: &str) -> Token;
}