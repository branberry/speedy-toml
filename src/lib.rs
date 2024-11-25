mod toml;
pub mod utils;

use std::fs;
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
    let contents =
        fs::read_to_string("cargo.toml").expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
