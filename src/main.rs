use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Missing input file");
    let hello_lox = fs::read_to_string(input_file)
        .expect(format!("Failed to read input file [{}]", input_file).as_str());

    println!("{}", hello_lox);
}
