use chumsky::primitive::Container;
use speedy_toml::parse;
pub fn main() {
    let str_test: String = "hello".to_string();

    for c in str_test.chars() {
        println!("{:?}", c);
    }
}
