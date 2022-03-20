#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum LitType {
    Identifier(String),
    String(String),
    Number(f32),
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
    token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Token { token_type }
    }
}

pub struct Lexer {
    line: Vec<char>,
    position: usize,
    curr_char: char,
}

impl Lexer {
    pub fn new(line: String) -> Self {
        let src_line: Vec<char> = line.trim().chars().collect();
        let curr_char = src_line.clone().into_iter().next().unwrap();

        Lexer {
            line: src_line,
            position: 0,
            curr_char,
        }
    }

    pub fn lex_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = vec![];
        let src_size = self.line.len();

        while self.position < src_size {
            tokens.push(self.lex_token()?);
        }

        tokens.push(Token::new(TokenType::Eof));

        Ok(tokens)
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
            self.lex_identifier()
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
            '/' => Ok(Token::new(TokenType::Op(OpType::Slash))),
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

    fn lex_identifier(&mut self) -> Result<Token, String> {
        let mut identifier: String = String::new();

        while let Some(c) = self.peek() {
            if c.is_alphabetic() {
                identifier.push(*c);
                self.consume_char();
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
mod test_lex {
    use super::*;

    #[test]
    fn test_lex_tokens() {
        let mut lexer = Lexer::new("var myVar = 5;".to_string());
        let tokens = lexer.lex_tokens().unwrap();

        assert_eq!(6, tokens.len());

        let mut lexer = Lexer::new("print \"hello\";".to_string());
        let tokens = lexer.lex_tokens().unwrap();

        assert_eq!(4, tokens.len());
    }
}
