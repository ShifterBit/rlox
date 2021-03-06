use crate::token::{Literal, Token, TokenType};
use crate::Lox;
use lazy_static::lazy_static;
use std::char;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and".to_owned(), TokenType::And);
        m.insert("class".to_owned(), TokenType::Class);
        m.insert("else".to_owned(), TokenType::Else);
        m.insert("false".to_owned(), TokenType::False);
        m.insert("for".to_owned(), TokenType::For);
        m.insert("fun".to_owned(), TokenType::Fun);
        m.insert("if".to_owned(), TokenType::If);
        m.insert("nil".to_owned(), TokenType::Nil);
        m.insert("or".to_owned(), TokenType::Or);
        m.insert("print".to_owned(), TokenType::Print);
        m.insert("return".to_owned(), TokenType::Return);
        m.insert("super".to_owned(), TokenType::Super);
        m.insert("this".to_owned(), TokenType::This);
        m.insert("true".to_owned(), TokenType::True);
        m.insert("var".to_owned(), TokenType::Var);
        m.insert("while".to_owned(), TokenType::While);

        m
    };
}

#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let end_of_file = Token::new(TokenType::Eof, "".to_owned(), None, self.line);
        self.tokens.push(end_of_file);
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => match self.match_('=') {
                true => self.add_token(TokenType::BangEqual),
                _ => self.add_token(TokenType::Bang),
            },

            '=' => match self.match_('=') {
                true => self.add_token(TokenType::EqualEqual),
                false => self.add_token(TokenType::Equal),
            },
            '<' => match self.match_('=') {
                true => self.add_token(TokenType::LessEqual),
                _ => self.add_token(TokenType::Less),
            },
            '>' => match self.match_('=') {
                true => self.add_token(TokenType::GreaterEqual),
                _ => self.add_token(TokenType::Greater),
            },
            '/' => match self.match_('/') {
                true => {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                }
                _ => self.add_token(TokenType::Slash),
            },
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    Lox::error(self.line, "Unexpected character".to_owned())
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>();
        let keyword_lookup = KEYWORDS.get(&text);
        let token_type = match keyword_lookup {
            Some(t) => t,
            None => &TokenType::Identifier,
        };
        self.add_token(token_type.to_owned());
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let value: Literal = Literal::Number(
            self.source
                .chars()
                .skip(self.start)
                .take(self.current - self.start)
                .collect::<String>()
                .parse::<f64>()
                .unwrap(),
        );

        self.add_full_token(TokenType::Number, Some(value));
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            Lox::error(self.line, "Unterminated string.".to_owned());
            return;
        }

        self.advance();
        let text = self
            .source
            .chars()
            .skip(self.start)
            .take((self.current) - (self.start))
            .collect::<String>();
        let text_without_quotes = text
            .chars()
            .skip(1)
            .take(text.len() - 2)
            .collect::<String>();

        let value: Literal = Literal::String(text_without_quotes.clone());

        self.add_full_token(TokenType::String, Some(value));
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        c as char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_full_token(token_type, Some(Literal::Nil));
    }

    fn add_full_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }
}
