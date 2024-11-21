use chumsky::{prelude::*, Parser};

#[derive(Clone, Debug``)]
pub enum Toml {
    Num(f64),
}

pub fn parser() -> impl Parser<char, Toml, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_digit()).map(|c| Toml::Num(c.to_digit(10).unwrap() as f64))
}
