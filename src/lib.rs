pub mod ast;
pub mod parser;
pub mod scanner;
pub mod token;

use parser::{ParseError, Parser};
use scanner::Scanner;
use std::env;
use std::fs;
use std::io;
use std::process;
use token::{Token, TokenType};

#[derive(Default)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn init(&mut self) {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            println!("Usage: rlox [script]");
            process::exit(64);
        } else if args.len() == 2 {
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
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse();
        if self.had_error {
            return;
        }
        println!("{:#?}", expression.unwrap());

        // for token in tokens.iter() {
        //     println!("{:?}", token);
        // }
    }

    fn error(line: i32, message: String) {
        Lox::report(line, "".to_owned(), message);
    }

    fn parse_error(error: ParseError) {
        match error.token.token_type {
            TokenType::Eof => Lox::report(error.token.line, " at end".to_owned(), error.message),
            _ => Lox::report(
                error.token.line,
                format!("at, {}", error.token.lexeme),
                error.message,
            ),
        }
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
