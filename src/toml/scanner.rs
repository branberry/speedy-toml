use std::{any::Any, convert::TryInto};

use super::token::{Token, TokenType};

pub trait Scanner {
    fn scan_tokens(&self) -> Vec<Token>;
    fn scan_token(&self, symbol: char) -> Token;
    fn add_token(&self, token_type: TokenType);
    fn add_token_literal(&self, token_type: TokenType, literal: Box<dyn Any>);
    fn is_at_end(&self) -> bool;
}

pub struct TomlScanner {
    source: String,
    current: i64,
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
            '[' => self.add_token(TokenType::LeftBrace),
            _ => (),
        }
        todo!()
    }

    fn add_token(&self, token_type: TokenType) {
        todo!()
    }

    fn is_at_end(&self) -> bool {
        self.current < self.source.len().try_into().unwrap()
    }

    fn add_token_literal(&self, token_type: TokenType, literal: Box<dyn Any>) {
        todo!()
    }
}
