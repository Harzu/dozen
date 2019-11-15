use crate::types::*;
use crate::tokenizer::{ Tokenizer };

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    tokenizer: Tokenizer,
    length: usize,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        let length = input.len();
        let tokenizer = Tokenizer::new();
        
        Lexer {
            input,
            position: 0,
            tokenizer,
            length
        }
    }

    pub fn run(&mut self) -> Vec<Token> {
        let mut res: Vec<Token> = Vec::new();

        while self.position != self.length {
            let current_char = self.peek(0);
            let token = self.tokenizer.next_token(&current_char.to_string());
            
            match token {
                Token::WHITESPACE => { /*...*/ },
                Token::UNKNOW => { panic!("Unknow token") },
                Token::WORD(head_char) => {
                    let word_token = self.tokenize_word(&head_char);
                    res.push(word_token);
                },
                Token::NUMBER(head_int) => {
                    let number_token = self.tokenize_number(head_int);
                    res.push(number_token);
                },
                _ => {
                    res.push(token);
                }
            }
            
            self.next_position();
        }

        res
    }

    fn tokenize_word(&mut self, head: &str) -> Token {
        let mut token = Token::WORD(head.to_string());
        let mut buffer = String::new();
        buffer.push_str(head);

        loop {
            let target_token = self.tokenizer.next_token(&buffer);
            
            match target_token {
                Token::WORD(_) => {
                    self.next_position();
                    let current_char = self.peek(0);
                    buffer.push(current_char);
                    token = target_token;
                    continue;
                },
                // TODO: fix for more reused format
                | Token::FUNCTION
                | Token::RETURN
                | Token::IF => {
                    token = target_token;
                    break;
                },
                _ => {
                    self.prev_position();
                    break;
                }
            }
        }

        token
    }

    fn tokenize_number(&mut self, head_num: i32) -> Token {
        let mut token = Token::NUMBER(head_num);
        let mut buffer = String::new();
        buffer.push_str(&head_num.to_string());
        self.next_position();

        loop {
            let current_char = self.peek(0);
            buffer.push_str(&current_char.to_string());
            let target_token = self.tokenizer.next_token(&buffer);
            
            match target_token {
                Token::NUMBER(_) => {
                    token = target_token;
                    self.next_position();
                    continue;
                },
                _ => {
                    self.prev_position();
                    break;
                }
            }
        }

        token
    }

    fn peek(&self, pos_offset: usize) -> char {
        let target_position = self.position + pos_offset;
        if target_position == self.length {
            return '\0'
        }
        
        self.input[target_position]
    }

    fn next_position(&mut self) {
        self.position += 1;
    }

    fn prev_position(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }
}