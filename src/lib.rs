mod file;
mod lexer;
mod tokenizer;
pub mod types;

use lexer::Lexer;
use types::{ ILexer, Token };

pub fn from_file(file_path: &str) -> Vec<Token> {
    let file_content = file::get_file_content(&file_path);
    let mut lexer = Lexer::new(file_content);
    let tokens = lexer.run();

    tokens
}

pub fn from_str(input: &str) -> Vec<Token> {
    let input_chars = input.chars().collect();
    let mut lexer = Lexer::new(input_chars);
    let tokens = lexer.run();

    tokens
}