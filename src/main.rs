mod tokenizer;
use tokenizer::{Tokenizer, Token};
use std::io::{self, Write};

enum Operators {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operators {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operators::Add),
            '-' => Some(Operators::Subtract),
            '*' => Some(Operators::Multiply),
            '/' => Some(Operators::Divide),
            _ => None,
        }
    }
}

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
            println!("Exiting the program.");
            break;
        }

        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        
        println!("Tokens: {:?}", tokens);
        
        //for now, just show tokens - will add evaluation later
        if tokens.len() > 1 { //more than just EOF
            println!("I see you want to calculate: {:?}", tokens);

        }
        let mut op_char = None;
        for c in input.chars() {
            if Operators::from_char(c).is_some() {
                op_char = Some(c);
                break;
            }
        }

        if op_char.is_none() {
            println!("No operator found.");
            continue;
        }

        let operator = op_char.unwrap();

        
        let parts: Vec<&str> = input.split(operator).collect();
        if parts.len() != 2 {
            println!("Invalid expression.");
            continue;
        }

        
        let left = match parts[0].trim().parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid left number.");
                continue;
            }
        };

        let right = match parts[1].trim().parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid right number.");
                continue;
            }
        };

        
        let op = Operators::from_char(operator).unwrap();

        
        let result = match op {
            Operators::Add => left + right,
            Operators::Subtract => left - right,
            Operators::Multiply => left * right,
            Operators::Divide => {
                if right == 0.0 {
                    println!("Cannot divide by zero.");
                    continue;
                }
                left / right
            }
        };

        
        println!("{}", result);
    }
}
