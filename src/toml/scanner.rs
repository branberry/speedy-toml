use std::{any::Any, convert::TryInto};

use crate::utils::get_substring;

use super::token::{Token, TokenType};

pub trait Scanner {
    fn scan_tokens(&self) -> Vec<Token>;
    fn scan_token(&self, symbol: char, start: &mut usize, current: &mut usize) -> Token;
    fn create_token(&self, token_type: TokenType, start: usize, end: usize) -> Token;
    fn add_token_literal(&self, token_type: TokenType, literal: Box<dyn Any>);
}

pub struct TomlScanner {
    source: String,
}

impl Scanner for TomlScanner {
    fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut start: usize = 0;
        let mut current: usize = 0;
        for ch in self.source.chars() {
            let token = self.scan_token(ch, &mut start, &mut current);
            tokens.push(token);
            start = current;
        }
        println!("{:?}", tokens);
        tokens
    }

    fn scan_token(&self, symbol: char, start: &mut usize, current: &mut usize) -> Token {
        *current += 1;

        match symbol {
            '[' => {
                let token = self.create_token(TokenType::LeftBrace, start.clone(), current.clone());
                return token;
            }
            ']' => {
                let token =
                    self.create_token(TokenType::RightBrace, start.clone(), current.clone());
                return token;
            }
            _ => panic!("Invalid token found {}", symbol),
        };
    }

    fn create_token(&self, token_type: TokenType, start: usize, end: usize) -> Token {
        let text = get_substring(&self.source, start, end);
        let token = Token::new(token_type, text, 0, None);
        token
    }

    fn add_token_literal(&self, token_type: TokenType, literal: Box<dyn Any>) {
        todo!()
    }
}

impl TomlScanner {
    pub fn new(source: String) -> Self {
        Self { source }
    }
}
