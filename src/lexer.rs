#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OpType {
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
    SlashSlash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl ToString for OpType {
    fn to_string(&self) -> String {
        match self {
            OpType::LeftParen => '('.to_string(),
            OpType::RightParen => ')'.to_string(),
            OpType::LeftBrace => '{'.to_string(),
            OpType::RightBrace => '}'.to_string(),
            OpType::Comma => ','.to_string(),
            OpType::Dot => '.'.to_string(),
            OpType::Minus => '-'.to_string(),
            OpType::Plus => '+'.to_string(),
            OpType::Semicolon => ';'.to_string(),
            OpType::Slash => '/'.to_string(),
            OpType::SlashSlash => "//".to_string(),
            OpType::Star => '*'.to_string(),
            OpType::Bang => '!'.to_string(),
            OpType::BangEqual => "!=".to_string(),
            OpType::Equal => '='.to_string(),
            OpType::EqualEqual => "==".to_string(),
            OpType::Greater => '>'.to_string(),
            OpType::GreaterEqual => ">=".to_string(),
            OpType::Less => '<'.to_string(),
            OpType::LessEqual => "<=".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum LitType {
    Identifier(String),
    String(String),
    Number(f32),
    False,
    True,
    Nil,
}

impl PartialEq for LitType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // The underlying data need not match; only the type needs to match
            (Self::Identifier(_), Self::Identifier(_)) => true,
            (Self::String(_), Self::String(_)) => true,
            (Self::Number(_), Self::Number(_)) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl ToString for LitType {
    fn to_string(&self) -> String {
        match self {
            LitType::Identifier(ref name) => String::from(name),
            LitType::String(ref s) => String::from(s),
            LitType::Number(ref x) => x.to_string(),
            LitType::False => "false".to_string(),
            LitType::True => "true".to_string(),
            LitType::Nil => "nil".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum KwordType {
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
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Op(OpType),
    Lit(LitType),
    Kword(KwordType),
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Token { token_type }
    }

    pub fn op_type(&self) -> Option<&OpType> {
        match &self.token_type {
            TokenType::Op(op) => Some(op),
            _ => None,
        }
    }

    pub fn str_val(&self) -> Option<String> {
        match self.token_type {
            TokenType::Lit(ref lit_type) => {
                match lit_type {
                    LitType::Identifier(s) => Some(String::from(s)),
                    LitType::String(s) => Some(String::from(s)),
                    LitType::Number(n) => Some(n.to_string()),
                    _ => None,
                }
            },
            _ => None,
        }
    }
}

pub struct LineLexer {
    line: Vec<char>,
    position: usize,
    curr_char: char,
}

impl LineLexer {
    pub fn new(line: String) -> Self {
        let src_line: Vec<char> = line.trim().chars().collect();
        let curr_char = src_line.clone().into_iter().next().unwrap_or('\0');

        LineLexer {
            line: src_line,
            position: 0,
            curr_char,
        }
    }

    pub fn lex_tokens(&mut self) -> Vec<Result<Token, String>> {
        let mut tokens = Vec::new();
        let src_size = self.line.len();

        while self.position < src_size {
            let token = self.lex_token();

            // Ignore comments
            if token.is_ok()
                && token.as_ref().unwrap().token_type == TokenType::Op(OpType::SlashSlash)
            {
                continue;
            }

            tokens.push(token);
        }

        tokens.push(Ok(Token::new(TokenType::Eof)));

        tokens
    }

    fn lex_token(&mut self) -> Result<Token, String> {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.consume_char();
            } else {
                break;
            }
        }

        if self.is_op_char() {
            // Parse an operator
            self.lex_op()
        } else if self.is_num_char() {
            // Parse a number
            self.lex_num()
        } else if self.peek().is_some() && self.peek().unwrap() == &'"' {
            // Parse a string
            self.lex_str()
        } else {
            // Parse an identifier
            self.lex_identifier_or_kword()
        }
    }

    fn is_op_char(&self) -> bool {
        if let Some(c) = self.peek() {
            "(){},.-+;/*!=><".contains(*c)
        } else {
            false
        }
    }

    fn is_num_char(&self) -> bool {
        if let Some(c) = self.peek() {
            c.is_numeric()
        } else {
            false
        }
    }

    fn lex_op(&mut self) -> Result<Token, String> {
        self.consume_char();
        match self.curr_char {
            // Handle single-char operators
            '(' => Ok(Token::new(TokenType::Op(OpType::LeftParen))),
            ')' => Ok(Token::new(TokenType::Op(OpType::RightParen))),
            '{' => Ok(Token::new(TokenType::Op(OpType::LeftBrace))),
            '}' => Ok(Token::new(TokenType::Op(OpType::RightBrace))),
            ',' => Ok(Token::new(TokenType::Op(OpType::Comma))),
            '.' => Ok(Token::new(TokenType::Op(OpType::Dot))),
            '-' => Ok(Token::new(TokenType::Op(OpType::Minus))),
            '+' => Ok(Token::new(TokenType::Op(OpType::Plus))),
            ';' => Ok(Token::new(TokenType::Op(OpType::Semicolon))),
            '*' => Ok(Token::new(TokenType::Op(OpType::Star))),

            // Handle multi-char operators
            '!' => {
                if let Some(next_char) = self.peek() {
                    if next_char == &'=' {
                        self.consume_char();
                        return Ok(Token::new(TokenType::Op(OpType::BangEqual)));
                    }
                }

                Ok(Token::new(TokenType::Op(OpType::Bang)))
            }
            '=' => {
                if let Some(next_char) = self.peek() {
                    if next_char == &'=' {
                        self.consume_char();
                        return Ok(Token::new(TokenType::Op(OpType::EqualEqual)));
                    }
                }

                Ok(Token::new(TokenType::Op(OpType::Equal)))
            }
            '>' => {
                if let Some(next_char) = self.peek() {
                    if next_char == &'=' {
                        self.consume_char();
                        return Ok(Token::new(TokenType::Op(OpType::GreaterEqual)));
                    }
                }

                Ok(Token::new(TokenType::Op(OpType::Greater)))
            }
            '<' => {
                if let Some(next_char) = self.peek() {
                    if next_char == &'=' {
                        self.consume_char();
                        return Ok(Token::new(TokenType::Op(OpType::LessEqual)));
                    }
                }

                Ok(Token::new(TokenType::Op(OpType::Less)))
            }
            // Handle comments
            '/' => {
                if let Some(next_char) = self.peek() {
                    if next_char == &'/' {
                        // Consume the rest of the source
                        while let Some(_next_char) = self.peek() {
                            self.consume_char();
                        }

                        return Ok(Token::new(TokenType::Op(OpType::SlashSlash)));
                    }
                }

                Ok(Token::new(TokenType::Op(OpType::Slash)))
            }

            _ => Err("Failed to parse token".to_string()),
        }
    }

    fn lex_num(&mut self) -> Result<Token, String> {
        let mut digits: String = String::new();

        while let Some(c) = self.peek() {
            if c.is_numeric() || c == &'.' {
                digits.push(*c);
                self.consume_char();
            } else {
                break;
            }
        }

        Ok(Token::new(TokenType::Lit(LitType::Number(
            digits.parse::<f32>().unwrap(),
        ))))
    }

    fn lex_str(&mut self) -> Result<Token, String> {
        let mut string_val = String::new();

        // Consume the quote character
        self.consume_char();

        // Consume string
        while let Some(c) = self.peek() {
            if c == &'"' {
                break;
            }

            string_val.push(*c);
            self.consume_char();
        }

        if self.peek().is_none() {
            return Err("Unclosed quotation".to_string());
        }

        // Consume the second quote character
        self.consume_char();

        Ok(Token::new(TokenType::Lit(LitType::String(string_val))))
    }

    fn lex_identifier_or_kword(&mut self) -> Result<Token, String> {
        let mut identifier: String = String::new();

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                identifier.push(*c);
                self.consume_char();
            } else if !c.is_whitespace() && !self.is_op_char() {
                let illegal_ch = *c;
                identifier.push(illegal_ch);
                self.consume_char();
                return Err(format!(
                    "Invalid character in identifier {}: {}",
                    identifier, illegal_ch
                ));
            } else {
                break;
            }
        }

        let maybe_kword_type = self.str_to_keyword_type(&identifier);
        if let Some(kword_type) = maybe_kword_type {
            Ok(Token::new(TokenType::Kword(kword_type)))
        } else {
            Ok(Token::new(TokenType::Lit(LitType::Identifier(identifier))))
        }
    }

    fn str_to_keyword_type(&self, maybe_kword: &str) -> Option<KwordType> {
        let kword_type = match maybe_kword {
            "and" => KwordType::And,
            "class" => KwordType::Class,
            "else" => KwordType::Else,
            "false" => KwordType::False,
            "fun" => KwordType::Fun,
            "for" => KwordType::For,
            "if" => KwordType::If,
            "nil" => KwordType::Nil,
            "or" => KwordType::Or,
            "print" => KwordType::Print,
            "return" => KwordType::Return,
            "super" => KwordType::Super,
            "this" => KwordType::This,
            "true" => KwordType::True,
            "var" => KwordType::Var,
            "while" => KwordType::While,
            _ => {
                return None;
            }
        };

        Some(kword_type)
    }

    // Return the next char in the line, consuming it.
    // This function assumes boundary checks have already been done.
    fn consume_char(&mut self) {
        self.curr_char = *self.line.get(self.position).unwrap();
        self.position += 1;
    }

    fn peek(&self) -> Option<&char> {
        self.line.get(self.position)
    }
}

#[cfg(test)]
mod test_lexer {
    use super::*;

    #[test]
    fn test_lex_tokens() {
        let mut lexer = LineLexer::new("var myVar = 5;".to_string());
        let tokens = lexer.lex_tokens();

        assert_eq!(6, tokens.len());

        let mut lexer = LineLexer::new("print \"hello\";".to_string());
        let tokens = lexer.lex_tokens();

        assert_eq!(4, tokens.len());

        let mut lexer = LineLexer::new("a*5".to_string());
        let tokens = lexer.lex_tokens();

        assert_eq!(4, tokens.len());
    }

    #[test]
    fn test_lex_str() {
        // Try to lex a string and assert that it succeeds
        let str = "\"hello\"";

        let mut lexer = LineLexer::new(str.to_string());
        let tok = lexer.lex_str();

        assert!(tok.is_ok());

        // Try to lex a bad string and assert that it fails
        let str = "\"hello";

        let mut lexer = LineLexer::new(str.to_string());
        let tok = lexer.lex_str();

        assert!(tok.is_err());
    }

    #[test]
    fn test_lex_number() {
        // Try to lex a string and assert that it succeeds
        let num = "32.1";

        let mut lexer = LineLexer::new(num.to_string());
        let tok = lexer.lex_num();

        assert!(tok.is_ok());
    }

    #[test]
    fn lex_kword() {
        let kword = "var";
        let mut lexer = LineLexer::new(kword.to_string());
        let tok = lexer.lex_identifier_or_kword().unwrap();
        assert_eq!(TokenType::Kword(KwordType::Var), tok.token_type);
    }

    #[test]
    fn lex_identifier() {
        // Define a list of strings paired with whether they're valid
        let test_inputs = vec![
            // Valid identifiers
            ("abc", true),
            ("a123", true),
            ("bcc2dd", true),
            // Invalid identifiers
            ("abc@", false),
            ("@#aa", false),
        ];

        // Run tests for each input
        for (identifier, is_valid) in test_inputs {
            let mut lexer = LineLexer::new(identifier.to_string());
            let result = lexer.lex_identifier_or_kword();
            if is_valid {
                // Assert the success type of lexing valid identifiers
                let tok = result.unwrap();
                assert_eq!(
                    TokenType::Lit(LitType::Identifier(String::from(identifier))),
                    tok.token_type
                );
            } else {
                // Assert that an error occurs when lexing invalid identifiers
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn lex_comment() {
        let src = "var // this is a comment";
        let mut lexer = LineLexer::new(src.to_string());
        let tokens = lexer.lex_tokens();

        // Two tokens: var, EOF
        assert_eq!(2, tokens.len());
    }

    #[test]
    fn test_empty() {
        let src = "";
        let mut lexer = LineLexer::new(src.to_string());
        let tokens = lexer.lex_tokens();

        // Only EOF token should be present
        assert_eq!(1, tokens.len());
    }

    #[test]
    fn test_illegal_char() {
        let src = "illegal?";
        let mut lexer = LineLexer::new(src.to_string());
        let tokens = lexer.lex_tokens();

        assert_eq!(2, tokens.len());
        assert!(tokens.get(0).unwrap().is_err());
    }
}
