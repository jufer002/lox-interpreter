use crate::lexer::{LitType, OpType, Token, TokenType, KwordType};

#[derive(Debug)]
pub enum Expr {
    // Binary expression
    Bin(Box<Expr>, OpType, Box<Expr>),
    // Grouping expression
    Grouping(Box<Expr>),
    // Literal expression
    Lit(LitType),
    // Unary expression
    Unary(OpType, Box<Expr>),
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
}

pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, curr: 0 }
    }

    pub fn parse_tokens(&mut self) {}

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.curr += 1;
        }

        self.prev()
    }

    // Return whether any of the given token types match the current token and advance by one
    fn consume(&mut self, tok_types: &[TokenType]) -> bool {
        for tok_type in tok_types {
            let tok = self.peek();
            if &tok.token_type == tok_type {
                self.advance();
                return true;
            }
        }

        false
    }

    // Return whether the current token is of the given type
    fn check(&self, tok_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == *tok_type
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.curr).expect("Missing token")
    }

    fn is_at_end(&self) -> bool {
        self.curr >= self.tokens.len() || self.peek().token_type == TokenType::Eof
    }

    fn prev(&mut self) -> &Token {
        self.tokens.get(self.curr - 1).expect("Missing token")
    }

    // Parse an expression
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // Parse an equality
    fn equality(&mut self) -> Expr {
        // Parse the LH comparator
        let mut expr = self.comparison();

        while self.consume(&[
            TokenType::Op(OpType::EqualEqual),
            TokenType::Op(OpType::BangEqual),
        ]) {
            let op = self.prev().op_type().expect("Expected operator").clone();
            let rh = self.comparison();
            expr = Expr::Bin(Box::new(expr), op, Box::new(rh));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.consume(&[
            TokenType::Op(OpType::Greater),
            TokenType::Op(OpType::GreaterEqual),
            TokenType::Op(OpType::Less),
            TokenType::Op(OpType::LessEqual),
        ]) {
            let op = self.prev().op_type().expect("Expected operator").clone();
            let rh = self.term();
            expr = Expr::Bin(Box::new(expr), op, Box::new(rh));
        }

        expr
    }

    // Parse a term
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.consume(&[
            TokenType::Op(OpType::Plus),
            TokenType::Op(OpType::Minus),
        ]) {
            let op = self.prev().op_type().expect("Expected operator").clone();
            let rh = self.factor();
            expr = Expr::Bin(Box::new(expr), op, Box::new(rh));
        }

        expr
    }

    // Parse a factor
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.consume(&[
            TokenType::Op(OpType::Slash),
            TokenType::Op(OpType::Star),
        ]) {
            let op = self.prev().op_type().expect("Expected operator").clone();
            let rh = self.unary();
            expr = Expr::Bin(Box::new(expr), op, Box::new(rh));
        }

        expr
    }

    // Parse a unary rule
    fn unary(&mut self) -> Expr {
        if self.consume(&[
            TokenType::Op(OpType::Bang),
            TokenType::Op(OpType::Minus),
        ]) {
            let op = self.prev().op_type().expect("Expected operator").clone();
            return Expr::Unary(op.clone(), Box::new(self.unary()));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        // false
        if self.consume(&[
            TokenType::Kword(KwordType::False),
        ]) {
            return Expr::Lit(LitType::False);
        }
        // true
        if self.consume(&[
            TokenType::Kword(KwordType::True),
        ]) {
            return Expr::Lit(LitType::True);
        }
        // nil
        if self.consume(&[
            TokenType::Kword(KwordType::Nil),
        ]) {
            return Expr::Lit(LitType::Nil);
        }
        // string or number
        if self.consume(&[
            TokenType::Lit(LitType::String("".to_string())),
            TokenType::Lit(LitType::Number(0.0)),
        ]) {
            let str_val = self.prev().str_val().expect("Expected string").to_owned();

            return Expr::Lit(LitType::String(str_val));
        }

        if !self.consume(&[
            TokenType::Op(OpType::LeftParen)
        ]) {
            panic!("No parse match in primary()");
        }

        let expr = self.expression();
        if !self.consume(&[TokenType::Op(OpType::RightParen)]) {
            panic!("Missing ')' in primary()");
        }

        Expr::Grouping(Box::new(expr))
    }
}

impl Visitor<()> for Parser {
    fn visit_expr(&mut self, expr: &Expr) {
        match *expr {
            Expr::Bin(ref left, ref op, ref right) => {
                self.visit_expr(left);
                print!(" {} ", op.to_string());
                self.visit_expr(right);
            }
            Expr::Grouping(ref expr) => {
                print!("(");
                self.visit_expr(expr);
                print!(")");
            }
            Expr::Lit(ref lit_type) => {
                print!("{}", lit_type.to_string());
            }
            Expr::Unary(ref op, ref expr) => {
                print!("{}", op.to_string());
                self.visit_expr(expr);
            }
        };
    }
}

#[cfg(test)]
mod test_parser {
    use super::*;

    #[test]
    fn test_visit() {
        let expressions = vec![
            // 1.5 + 1.5
            Expr::Bin(
                Box::new(Expr::Lit(LitType::Number(1.5))),
                OpType::Plus,
                Box::new(Expr::Lit(LitType::Number(1.5))),
            ),
            // (5 * 4)
            Expr::Grouping(Box::new(Expr::Bin(
                Box::new(Expr::Lit(LitType::Number(5.0))),
                OpType::Star,
                Box::new(Expr::Lit(LitType::Number(4.0))),
            ))),
            // "hello"
            Expr::Lit(LitType::String("hello".to_string())),
            // -1
            Expr::Unary(OpType::Minus, Box::new(Expr::Lit(LitType::Number(1.0)))),
        ];

        let mut parser = Parser::new(Vec::new());
        for expr in expressions {
            parser.visit_expr(&expr);
            println!();
        }
    }
}
