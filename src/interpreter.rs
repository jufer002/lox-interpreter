use crate::lex::Lexer;

// Execute a line of lox
pub fn exec_line(line: String) -> Result<(), String> {
    let mut lexer = Lexer::new(line);

    let tokens = lexer.lex_tokens()?;

    dbg!(tokens);

    Ok(())
}

// Report to the user that an error has occurred
pub fn report_err(line_no: u32, err_msg: &str) {
    println!("[line {}] Error: {}", line_no, err_msg);
}
