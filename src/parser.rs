use crate::lexer::{LitType, OpType, Token, TokenType};

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
    // fn visit_bin(&mut self, b: Expr) -> T;
    // fn visit_grouping(&mut self, b: Expr) -> T;
    // fn visit_lit(&mut self, b: Expr) -> T;
    // fn visit_unary(&mut self, b: Expr) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
}

pub struct Parser;

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
    fn test_visit_bin() {
        
        let expressions = vec![
            // 1.5 + 1.5
            Expr::Bin(
                Box::new(Expr::Lit(LitType::Number(1.5))),
                OpType::Plus,
                Box::new(Expr::Lit(LitType::Number(1.5))),
            ),

            // (5 * 4)
            Expr::Grouping(
                Box::new(Expr::Bin(
                    Box::new(Expr::Lit(LitType::Number(5.0))),
                    OpType::Star,
                    Box::new(Expr::Lit(LitType::Number(4.0))),
                ))
            ),

            // "hello"
            Expr::Lit(
                LitType::String("hello".to_string())
            ),

            // -1
            Expr::Unary(OpType::Minus, Box::new(Expr::Lit(LitType::Number(1.0)))),
        ];

        let mut parser = Parser {};
        for expr in expressions {
            parser.visit_expr(&expr);
            println!();
        }
        
    }
}
