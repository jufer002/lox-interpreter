use crate::lex::lex_tokens;

// Execute a line of lox
pub fn exec_line(line: String) -> Result<(), &'static str> {
    let tokens = lex_tokens(line)?;

    Ok(())
}

// Report to the user that an error has occurred
pub fn reportError(line: u32, err_loc: &str, err_msg: &str) {
    println!("[line {}] Error {}: {}", line, err_loc, err_msg);
}

