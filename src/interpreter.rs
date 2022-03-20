use crate::lex::lex_tokens;

// Execute a line of lox
pub fn exec_line(line: String, line_no: u32) -> Result<(), &'static str> {
    let tokens = lex_tokens(line, line_no)?;

    dbg!(tokens);

    Ok(())
}

// Report to the user that an error has occurred
pub fn report_err(line_no: u32, err_msg: &str) {
    println!("[line {}] Error: {}", line_no, err_msg);
}

