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

pub trait ILexer {
    fn new(input: Vec<char>) -> Self;
    fn run(&mut self) -> Vec<Token>;
}