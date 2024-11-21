use chumsky::{prelude::*, Parser};

#[derive(Clone, Debug)]
pub enum Toml {
    Num(f64),
    Var(String),

    Neg(Box<Toml>),
    Add(Box<Toml>, Box<Toml>),
    Sub(Box<Toml>, Box<Toml>),
    Mul(Box<Toml>, Box<Toml>),
    Div(Box<Toml>, Box<Toml>),

    Call(String, Vec<Toml>),
    Let {
        name: String,
        rhs: Box<Toml>,
        then: Box<Toml>,
    },
    Fn {
        name: String,
        args: Vec<String>,
        body: Box<Toml>,
        then: Box<Toml>,
    },
}

pub fn parser() -> impl Parser<char, Toml, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_digit()).map(|c| Toml::Num(c.to_digit(10).unwrap() as f64))
}
