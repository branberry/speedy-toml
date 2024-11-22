pub fn error(line: i32, message: String) {
    report(line, "".to_string(), message)
}

pub fn report(line: i32, from: String, message: String) {
    println!("[line {}] Error {}: {}", line, from, message)
}
