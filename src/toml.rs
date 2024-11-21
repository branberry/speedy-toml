use chumsky::{prelude::*, Parser};

#[derive(Clone, Debug)]
pub enum Toml {
    Num(f64),
    Str(String),
}

pub fn parser() -> impl Parser<char, Toml, Error = Simple<char>> {
    recursive(|expr| {
        let int = text::int(10)
            .map(|s: String| Toml::Num(s.parse().unwrap()))
            .padded();

        int
    })
    .then_ignore(end())
}

pub fn eval(toml: &Toml) -> Result<f64, String> {
    match toml {
        Toml::Num(num) => Ok(*num),
        _ => todo!(),
    }
}
