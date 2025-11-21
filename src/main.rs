mod tokenizer;
mod parser;

use tokenizer::Tokenizer;
use parser::Parser;

use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        let result = ast.eval();

        println!("{}", result);
    }
}
