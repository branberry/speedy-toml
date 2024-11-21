mod toml;
mod utils;

use chumsky::Parser;
use toml::parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, toml-js!");
}

#[wasm_bindgen]
pub fn parse() {
    let parser = parser();
    let result = parser.parse("5");

    println!("{:?}", result)
}
