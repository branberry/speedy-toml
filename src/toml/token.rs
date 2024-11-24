use std::any::Any;

pub enum TokenType {
    // single char token
    Comma,
    Dot,
    LeftBrace,
    RightBrace,
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Box<dyn Any>,
    line: i32,
}
