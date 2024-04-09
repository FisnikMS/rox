mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod token;

use core::panic;
use std::{
    env, fs,
    io::{self, Write},
};

use crate::{expr::Expr, parser::Parser, scanner::Scanner};
fn main() {
    let env: Vec<String> = env::args().collect();
    if env.len() > 1 {
        run_file(&env[1]);
    } else {
        run_prompt();
    }
}

fn run_prompt() {
    let mut input: String = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                if input == "exit" {
                    break;
                }
                run(&input);
                input = String::new();
                unsafe {
                    error::HAD_ERROR = false;
                }
            }
            Err(_err) => println!("something went wrong"),
        }
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source.to_string());
    let tokens: &Vec<token::Token> = scanner.scan_tokens();

    let mut parser: Parser = Parser::new(tokens);
    let expressions: Box<Expr> = parser.parse();
    // Stop if there was a syntax error.
    unsafe {
        if error::HAD_ERROR {
            return;
        }
    }

    println!("{:?}", tokens);
    println!("{:?}", expressions);
    for token in tokens {
        print!("{}", token);
    }
    println!("");
}

fn run_file(source: &str) {
    let content = fs::read_to_string(source).expect("Should have been able to read the file");
    unsafe {
        if error::HAD_ERROR {
            panic!("error")
        };
    }

    run(&content);
}
