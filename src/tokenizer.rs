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
    pub fn new(input: String) -> Self {
        Tokenizer {
            input: input.to_string(),
            position: 0,
            current_char: input.chars().next(),
        }
    
    }
    fn advance(&mut self) { ... }
    fn skip_whitespace(&mut self) { ... }
    fn read_number(&mut self) -> Token { ... }
    pub fn next_token(&mut self) -> Token { ... }

}