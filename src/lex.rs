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
pub fn lex_tokens(line: String) -> Result<Vec<Token>, &'static str> {
    let mut line_no = 1;
    let mut curr_loc = 0;
    let line_length = line.len();
    let mut tokens: Vec<Token> = vec![];
    let chars: Vec<_> = line.chars().collect();

    while curr_loc < line_length {
        let next_token = lex_token(&mut curr_loc, &chars, line_no)?;
        tokens.push(next_token);
    }

    tokens.push(Token::abstract_token(TokenType::EOF, "", line_no));

    Ok(tokens)
}

fn lex_token(curr_loc: &mut usize, line: &Vec<char>, line_no: u32) -> Result<Token, &'static str> {
    let next_char: char = *line.get(*curr_loc).unwrap();
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

        _ => {
            Err("Failed to lex token")
        },
    }
}


#[cfg(test)]
mod test_lex {
    use super::*;

    #[test]
    fn test_lex_token() {
        let mut curr_loc = 0;
        let line_no = 1;
        let token = lex_token(&mut curr_loc, &",".to_string().chars().collect(), line_no).unwrap();
        assert_eq!(token.token_type, TokenType::Comma);
    }

    #[test]
    fn test_lex_single_char_tokens() {
        let line = "(){},.-+;/*".to_string();
        let tokens = lex_tokens(line.clone()).unwrap();

        // We'll have line.len() + 1 tokens because an EOF token is included
        assert_eq!(line.len() + 1, tokens.len());
    }
}
