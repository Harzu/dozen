use std::ops::Fn;
use regex::Regex;
use crate::types::{ ITokenizer, Token };

macro_rules! regex {
    ($re:expr) => {
        Regex::new($re).unwrap()
    };
}

pub struct Tokenizer {
    matching_pairs: Vec<(String, Box<dyn Fn(String) -> Token>)>,
}

impl ITokenizer for Tokenizer {
    fn new() -> Self {
        let matching_pairs: Vec<(String, Box<dyn Fn(String) -> Token>)> = vec![
            (String::from(r"if"), Box::new(|_| -> Token { Token::IF })),
            (String::from(r"return"), Box::new(|_| -> Token { Token::RETURN })),
            (String::from(r"def"), Box::new(|_| -> Token { Token::FUNCTION })),

            (String::from(r"\{"), Box::new(|_| -> Token { Token::RBrace })),
            (String::from(r"\}"), Box::new(|_| -> Token { Token::LBrace })),
            (String::from(r"\("), Box::new(|_| -> Token { Token::LParen })),
            (String::from(r"\)"), Box::new(|_| -> Token { Token::RParen })),
            (String::from(r";"), Box::new(|_| -> Token { Token::SEMICOIN })),
                    
            (String::from(r"[a-zA-Z_][a-zA-Z0-9_]*"), Box::new(|item| -> Token { Token::WORD(item.to_string()) })),
            (String::from(r"[0-9]+"), Box::new(|item| -> Token {
                match item.parse() {
                    Ok(v) => Token::NUMBER(v),
                    Err(_) => Token::WORD(item.to_string())
                }
            })),

            (String::from(r"[ \t\r\n]"), Box::new(|_| -> Token { Token::WHITESPACE }))
        ];

        Tokenizer {
            matching_pairs
        }
    }

    fn next_token(&self, item: &str) -> Token {
        let mut res: Token = Token::UNKNOW;

        for pair in &self.matching_pairs {
            let (template, token) = pair;
            let regexp = regex!(&template);
            if let Some(_) = regexp.find(&item) {
                res = token(item.to_string());
                break;
            }
        }
        
        res
    }
}
