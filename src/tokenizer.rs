#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    EOF,

}

pub struct Tokenizer {
    input: String,
    position: usize,
    current_char: Option<char>,

}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        let input = input.to_string();
        let current_char = input.chars().next();
        Tokenizer {
            input,
            position: 0,
            current_char,
        }
    
    }
    fn advance(&mut self) { 
        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input.chars().nth(self.position);
        }
     }

     
    fn skip_whitespace(&mut self) { 
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }


    }
    fn read_number(&mut self) -> Token { 
        let mut number_str = String::new();
        while let Some (c) = self.current_char {
            if c.is_ascii_digit() || c == '.' {
                number_str.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let number: f64 = number_str.parse().unwrap();
        Token::Number(number)
    }
    pub fn next_token(&mut self) -> Token { ... }

}