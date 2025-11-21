mod tokenizer;
mod parser;

use tokenizer::Tokenizer;
use parser::Parser;

mod evaluator;


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

        let mut tokenizer: Tokenizer = Tokenizer::new(input);
        let tokens: Vec<tokenizer::Token> = tokenizer.tokenize();

        let mut parser: Parser = Parser::new(tokens);
        let ast: parser::Expr = parser.parse();

        let result: f64 = ast.eval();

        println!("{}", result);
    }
}
