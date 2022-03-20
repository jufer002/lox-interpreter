use crate::interpreter::{exec_line, report_err};
use std::io::{stdin, BufRead, BufReader, Read, Write};

// Start a Lox REPL that will continually interpret lines until it receives the 'exit/quit' command
pub fn run_repl() {
    let mut line_no: u32 = 1;
    loop {
        print_prompt(line_no);
        let line = read_line(stdin());
        if line == "exit" || line == "quit" {
            break;
        } else {
            exec_line(line).unwrap_or_else(|e| {
                report_err(line_no, e.as_str());
            });
        }

        line_no += 1;
    }
}

// Print a prompt to the user and flush stdout
fn print_prompt(line_no: u32) {
    print!("[🦀 lox] [{}] > ", line_no);
    std::io::stdout().flush().unwrap();
}

// Read a line and return it
fn read_line(read_src: impl Read) -> String {
    let mut reader = BufReader::new(read_src);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    // Strip CRLF or LF
    buffer
        .strip_suffix("\r\n")
        .or_else(|| buffer.strip_suffix('\n'))
        .unwrap_or(buffer.as_str())
        .to_string()
}

#[cfg(test)]
mod test_repl {
    use super::*;

    #[test]
    fn test_read_line() {
        let inputs = vec!["abc\r\n", "abc\n", "abc"];

        for input in inputs {
            let abc = read_line(input.as_bytes());
            assert_eq!("abc", abc);
        }

        // Test that reading an empty line is ok
        read_line("".as_bytes());
    }
}
