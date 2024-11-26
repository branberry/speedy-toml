mod toml;
pub mod utils;
use toml::scanner::{Scanner, TomlScanner};

pub fn main() {
    let str_test: String = "[]".to_string();

    let scanner: TomlScanner = TomlScanner::new("[]".to_string());
    scanner.scan_tokens();
    for c in str_test.chars() {
        println!("{:?}", c);
    }
}
