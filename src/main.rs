mod ast;
mod scanner;
mod token;

use ast::{BinaryOperator, Expr, UnaryOperator};
use scanner::Scanner;
use std::env;
use std::fs;
use std::io;
use std::process;
use token::{Token, TokenType};

fn main() {
    let mut lox: Lox = Default::default();
    // lox.init();
    let expression = Box::new(Expr::Binary(
        Box::new(Expr::Unary(
            UnaryOperator::Minus,
            Box::new(Expr::Number(123.0)),
        )),
        BinaryOperator::Star,
        Box::new(Expr::Grouping(Box::new(Expr::Number(45.67)))),
    ));
    println!("{:?}", expression);
}

#[derive(Default)]
struct Lox {
    had_error: bool,
}
impl Lox {
    fn init(&mut self) {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            println!("Usage: rlox [script]");
            process::exit(64);
        } else if args.len() == 1 {
            self.run_file(&args[0]);
        } else {
            self.run_prompt();
        }
    }
    fn run_file(&self, path: &String) {
        let file = fs::read_to_string(path).unwrap();
        self.run(&file);
        if self.had_error {
            process::exit(65);
        }
    }

    fn run_prompt(&mut self) {
        loop {
            println!("> ");
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            if line.is_empty() {
                break;
            }
            self.run(&line);
            self.had_error = false;
        }
    }

    fn run(&self, source: &String) {
        let mut scanner: Scanner = Scanner::new(source.clone());
        let tokens: Vec<Token> = scanner.scan_tokens();

        for token in tokens.iter() {
            println!("{}", token);
        }
    }

    fn error(line: i32, message: String) {
        Lox::report(line, "".to_owned(), message);
    }

    fn report(line: i32, location: String, message: String) {
        eprintln!(
            "[line {line} ] Error {location}: {message}",
            line = line,
            location = location,
            message = message
        );
    }
}
