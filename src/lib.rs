pub mod ast;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;

use interpreter::{Interpreter, RuntimeError};
use parser::{ParseError, Parser};
use scanner::Scanner;
use std::env;
use std::fs;
use std::io;
use std::process;
use token::{Literal, Token, TokenType};

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

#[derive(Default)]
pub struct Lox {
    // had_error: bool,
// had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox {
            // had_error: false,
            // had_runtime_error: false,
        }
    }

    pub fn init(&mut self) {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() > 1 {
            println!("Usage: rlox [script]");
            process::exit(64);
        } else if args.len() == 1 {
            self.run_file(&args[0]);
        } else {
            self.run_prompt();
        }
    }

    fn run_file(&mut self, path: &String) {
        let file = fs::read_to_string(path).unwrap();
        self.run(&file);
        unsafe {
            if HAD_ERROR {
                process::exit(65);
            }
            if HAD_RUNTIME_ERROR {
                process::exit(70);
            }
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
            unsafe {
                HAD_ERROR = false;
            }
        }
    }

    fn run(&mut self, source: &String) {
        let mut scanner: Scanner = Scanner::new(source.clone());
        let tokens: Vec<Token> = scanner.scan_tokens();
        let mut parser: Parser = Parser::new(tokens.clone());
        let expression = parser.parse();

        unsafe {
            if HAD_ERROR {
                return;
            }
        }

        let interpreter = Interpreter::new();
        interpreter.interpret(expression);
    }

    fn error(line: i32, message: String) {
        Lox::report(line, "".to_owned(), message);
    }

    fn runtime_error(error: RuntimeError) {
        println!("{} \n [line {}]", error.message, error.token.line);
        unsafe { HAD_RUNTIME_ERROR = true }
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
        unsafe {
            HAD_ERROR = true;
        }
    }
}
