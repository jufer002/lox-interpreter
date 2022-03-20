use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // EOF
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    identifier: Option<String>,
    line_no: u32,
}

impl Token {
    // Return a new "abstract" token (non-literal)
    fn abstract_token(token_type: TokenType, chr: &'static str, line_no: u32) -> Self {
        Token {
            token_type,
            lexeme: chr.to_string(),
            identifier: None,
            line_no: line_no,
        }
    }
}

// Get a list of tokens from a line of Lox source
pub fn lex_tokens(line: String, line_no: u32) -> Result<Vec<Token>, &'static str> {
    let mut curr_loc = 0;
    let line_length = line.len();
    // Create a vector which we'll fill with tokens
    let mut tokens: Vec<Token> = vec![];
    // Create an iterator over the line's chars
    let mut chars = line.chars().into_iter().peekable();

    while curr_loc < line_length {
        let next_token = lex_token(&mut curr_loc, &mut chars, line_no)?;
        tokens.push(next_token);
    }

    tokens.push(Token::abstract_token(TokenType::EOF, "", line_no));

    Ok(tokens)
}

fn lex_token(
    curr_loc: &mut usize,
    iter: &mut Peekable<impl Iterator<Item = char>>,
    line_no: u32,
) -> Result<Token, &'static str> {
    if let None = iter.peek() {
        return Err("Failed to lex token");
    }

    let next_char = iter.next().unwrap();
    *curr_loc += 1;

    match next_char {
        '(' => Ok(Token::abstract_token(TokenType::LeftParen, "(", line_no)),
        ')' => Ok(Token::abstract_token(TokenType::RightParen, ")", line_no)),
        '{' => Ok(Token::abstract_token(TokenType::LeftBrace, "{", line_no)),
        '}' => Ok(Token::abstract_token(TokenType::RightBrace, "}", line_no)),
        ',' => Ok(Token::abstract_token(TokenType::Comma, ",", line_no)),
        '.' => Ok(Token::abstract_token(TokenType::Dot, ".", line_no)),
        '-' => Ok(Token::abstract_token(TokenType::Minus, "-", line_no)),
        '+' => Ok(Token::abstract_token(TokenType::Plus, "+", line_no)),
        ';' => Ok(Token::abstract_token(TokenType::Semicolon, ";", line_no)),
        '/' => Ok(Token::abstract_token(TokenType::Slash, "/", line_no)),
        '*' => Ok(Token::abstract_token(TokenType::Star, "*", line_no)),
        '!' => {
            // Check for BangEqual
            let maybe_next = iter.peek();
            if maybe_next.is_none() {
                return Ok(Token::abstract_token(TokenType::Bang, "!", line_no));
            } else {
                let next_char = maybe_next.unwrap();
                if *next_char == '=' {
                    iter.next().unwrap();
                    *curr_loc += 1;
                    Ok(Token::abstract_token(TokenType::BangEqual, "!=", line_no))
                } else {
                    Ok(Token::abstract_token(TokenType::Bang, "!", line_no))
                }
            }
        },
        '=' => {
            // Check for BangEqual
            let maybe_next = iter.peek();
            if maybe_next.is_none() {
                return Ok(Token::abstract_token(TokenType::Equal, "=", line_no));
            } else {
                let next_char = maybe_next.unwrap();
                if *next_char == '=' {
                    iter.next().unwrap();
                    *curr_loc += 1;
                    Ok(Token::abstract_token(TokenType::EqualEqual, "==", line_no))
                } else {
                    Ok(Token::abstract_token(TokenType::Equal, "=", line_no))
                }
            }
        },

        _ => Err("Failed to lex token"),
    }
}

#[cfg(test)]
mod test_lex {
    use super::*;

    #[test]
    fn test_lex_single_char_tokens() {
        let line = "(){},.-+;/*!!!".to_string();
        let tokens = lex_tokens(line.clone(), 1).unwrap();

        // We'll have line.len() + 1 tokens because an EOF token is included
        assert_eq!(line.len() + 1, tokens.len());
    }

    #[test]
    fn test_lex_multi_char_tokens() {
        let line = "{!=.".to_string();
        let tokens = lex_tokens(line.clone(), 1).unwrap();

        // First token: LeftBrace
        // Second token: BangEqual
        // Third token: Dot
        // Last token: EOF
        assert_eq!(4, tokens.len());
    }
}
