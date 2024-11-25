mod toml;
pub mod utils;
extern crate console_error_panic_hook;
use js_sys::Promise;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::{Request, RequestInit};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetch)]
    fn fetch_with_request_and_init(input: &Request, init: &RequestInit) -> Promise;
}

#[wasm_bindgen]
pub fn parse(test_str: String) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    test_str
}
