use crate::tokenizer::Token;

#[derive(Debug)]

pub enum Expr {
    Number (f64),
    Binary {
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

        fn advance(&mut self) {
        if self.position < self.tokens.len() - 1 {
            self.position += 1;
        }
    }

    // Grammar:
    //
    // expr        = term (("+" | "-") term)*
    // term        = factor (("*" | "/") factor)*
    // factor      = NUMBER | "(" expr ")"
    
    pub fn parse(&mut self) -> Expr {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Expr {
        let mut node = self.parse_term();

        loop {
            match self.current() {
                Token::Plus | Token::Minus => {
                    let op = self.current().clone();
                    self.advance();
                    let right = self.parse_term();
                    node = Expr::Binary {
                        left: Box::new(node),
                        op,
                        right: Box::new(right),
                    };
                }
                    _ => break,
            }
        }
        node
    }

    fn parse_factor(&mut self) -> Expr {
        match self.current() {
            Token::Number(n) => {
                let val = *n;
                self.advance();
                Expr::Number(val)
            }

            Token::LParen => {
                self.advance(); //skip '('
                let expr = self.parse_expr();

                match self.current() {
                    Token::RParen => self.advance(),
                    _ => panic!("Expected ')'"),
                }

                expr
            }

            other => panic!("Unexpected token in factor: {:?}", other),
        }
    }
}




