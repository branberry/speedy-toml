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
    literal: Option<Box<dyn Any>>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line: usize,
        literal: Option<Box<dyn Any>>,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            literal,
        }
    }
}
