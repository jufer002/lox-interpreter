mod interpreter;
mod lex;
mod repl;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_script(args.get(1).unwrap());
    } else {
        repl::run_repl();
    }
}

fn run_script(script_path: &str) {
    let script_str = fs::read_to_string(script_path)
        .unwrap_or_else(|e| {
            panic!("Failed to read input file {}: {}", script_path, e)
        });

    println!("{}", script_str);
}
