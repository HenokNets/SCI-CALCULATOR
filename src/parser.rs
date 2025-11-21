use crate::tokenizer::Token;

#[derive(Debug)]

pub enum Expr {
    Number (f64),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new (tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

        fn current(&self) -> &Token {
        &self.tokens[self.position]
    }
}
