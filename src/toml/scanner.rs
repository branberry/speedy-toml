use std::{any::Any, convert::TryInto};

use crate::utils::get_substring;

use super::token::{Token, TokenType};

pub trait Scanner {
    fn scan_tokens(&self) -> Vec<Token>;
    fn scan_token(&self, symbol: char) -> Token;
    fn create_token(&self, token_type: TokenType) -> Token;
    fn add_token_literal(&self, token_type: TokenType, literal: Box<dyn Any>);
    fn is_at_end(&self) -> bool;
}

pub struct TomlScanner {
    source: String,
    start: usize,
    current: usize,
    tokens: Vec<Token>,
}

impl Scanner for TomlScanner {
    fn scan_tokens(&self) -> Vec<Token> {
        let tokens: Vec<Token> = Vec::new();
        for ch in self.source.chars() {
            self.scan_token(ch);
        }

        return tokens;
    }

    fn scan_token(&self, symbol: char) -> Token {
        match symbol {
            '[' => self.create_token(TokenType::LeftBrace),
            _ => todo!(),
        };
        todo!()
    }

    fn create_token(&self, token_type: TokenType) -> Token {
        let text = get_substring(&self.source, self.start, self.current);
        let token = Token::new(token_type, text, 0, None);
        token
    }

    fn is_at_end(&self) -> bool {
        self.current < self.source.len().try_into().unwrap()
    }

    fn add_token_literal(&self, token_type: TokenType, literal: Box<dyn Any>) {
        todo!()
    }
}
