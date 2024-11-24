use super::token::Token;

pub trait Scanner {
    fn scanTokens(&self, source: String) -> Vec<Token>;
}
pub struct TomlScanner {
    source: String,
}

impl Scanner for TomlScanner {
    fn scanTokens(&self, source: String) -> Vec<Token> {
        todo!()
    }
}
