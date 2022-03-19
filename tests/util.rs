use std::{fs, path::Path};

pub fn read_file(file_name: &str) -> String {
    let file_path = Path::new("tests").join("data").join(file_name);
    fs::read_to_string(file_path).expect(format!("Failed to read file {}", file_name).as_str())
}

#[test]
fn test_read_file() {
    let hello_lox = read_file("1_hello.lox");

    assert_ne!(0, hello_lox.len());
}
