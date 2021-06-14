mod scanner;
mod token;

use scanner::Scanner;
use std::env;
use std::fs;
use std::io;
use std::process;
use token::Token;

fn main() {
    let mut lox: Lox = Lox::new();
    lox.init();
}

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn init(&mut self) {
        let args: Vec<String> = env::args().collect();
        println!("args: {:#?}",args);
        if args.len() > 1 {
            println!("Usage: rlox [script]");
            process::exit(64);
        } else if args.len() >= 2 {
            self.run_file(&args[0])
        } else {
            self.run_prompt();
        }
    }

    fn run_file(&self, path: &String) {
        let string = fs::read_to_string(path).expect("Error Reading File");
        self.run(string);
    }

    fn run(&self, source: String) {
        let mut scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token);
        }
    }

    fn run_prompt(&mut self) {
        loop {
            println!("> ");

            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            if line.len() == 0 {
                break;
            }
            self.run(line);

            self.had_error = false;
        }
    }

    fn error(line: i32, message: String) {
        Lox::report(line, "".to_owned(), message);
    }

    fn report(line: i32, location: String, message: String) {
        println!("[line {}] Error {}: {}", line, location, message);
    }
}
