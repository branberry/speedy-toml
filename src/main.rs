use chumsky::primitive::Container;
use speedy_toml::{parse, utils::get_substring};

pub fn main() {
    let str_test: String = "hello".to_string();

    let sub_string = get_substring(&str_test, 0, 3);
    println!("{sub_string}");
    for c in str_test.chars() {
        println!("{:?}", c);
    }
}
