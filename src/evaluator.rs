use crate::tokenizer::Token;
use crate::parser::Expr;

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Number(n) => *n,

            Expr::Unary { op, expr } => {
                let val = expr.eval();
                match op {
                    Token::Plus => val,      // +x is just x
                    Token::Minus => -val,    // -x flips the sign
                    _ => panic!("Invalid unary operator"),
                }
            }

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
