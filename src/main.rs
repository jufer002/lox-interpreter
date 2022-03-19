mod util;
use util::*;

fn main() {
    let hello_lox = read_file("hello.lox");

    println!("{}", hello_lox);
}
