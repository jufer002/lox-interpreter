use std::io::{stdin, BufRead, BufReader, Write};

pub fn run_repl() {
    loop {
        print_prompt();
        let line = read_line();
        if line == "exit" {
            break;
        }

        exec_line(line);
    }
}

// Print a prompt to the user and flush stdout
fn print_prompt() {
    print!("[🦀 lox] > ");
    std::io::stdout().flush().unwrap();
}

// Read a line and return it
fn read_line() -> String {
    let mut reader = BufReader::new(stdin());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    // Strip CRLF or LF
    buffer
        .strip_suffix("\r\n")
        .or(buffer.strip_suffix("\n"))
        .unwrap_or(buffer.as_str())
        .to_string()
}

// Execute a line of lox
fn exec_line(line: String) {
    // Get a list of tokens from the line
    let tokens = line.split_whitespace();
    for token in tokens {
        println!("token: {}", token);
    }
}

fn reportError(line: u32, err_loc: &str, err_msg: &str) {
    println!("[line {}] Error {}: {}", line, err_loc, err_msg);
}
