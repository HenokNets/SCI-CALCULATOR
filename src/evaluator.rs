
use crate::tokenizer::Token;
use crate::parser::Expr;

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Number(n) => *n,

            Expr::Binary { left, op, right } => {
                let l = left.eval();
                let r = right.eval();

                match op {
                    Token::Plus => l + r,
                    Token::Minus => l - r,
                    Token::Star => l * r,
                    Token::Slash => {
                        if r == 0.0 {
                            panic!("Division by zero");
                        }
                        l / r
                    }
                    _ => panic!("Invalid operator in AST"),
                }
            }
        }
    }
}



