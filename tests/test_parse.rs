use lox_interpreter::util::read_file;


#[test]
fn test_parse() {
    let hello_lox = read_file("hello.lox");

    println!("{}", hello_lox);
}
