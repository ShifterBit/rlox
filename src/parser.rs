use crate::ast::*;
use crate::token::{Literal, Token, TokenType};

// ------------ Syntax Grammar ------------
///
// -------- EXPRESSIONS --------
// expression     -> equality ;
// equality       -> comparison ( ("!=" | "==" ) comparison )* ;
// comparison     -> term ( (">" | ">=" | "<=" | "<" ) term )* ;
// unary          -> ( "-" | "!" ) unary | primary ;
// term           -> factor ( ("-" | "+") factor)* ;
// factor         -> unary ( ("/" | "*") unary)* ;
// primary        ->  NUMBER | String | "true" | "false" | "nil" | "(" expression ")" ;

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comparison()?;
        while self.match_(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_(&vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::LessEqual,
            TokenType::Less,
        ]) {
            let operator: Token = self.previous();
            let right: Expr = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_(&vec![TokenType::Plus, TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.match_(&vec![TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_(&vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.match_(&vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if self.match_(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_(&vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal.unwrap()));
        }

        if self.match_(&vec![TokenType::LeftParen]) {
            let expr: Expr = self.expression()?;
            self.consume(
                self.peek().clone().token_type,
                &"Expect ')' after expression.".to_owned(),
            )
            .unwrap();
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        return Err(ParseError::new(
            self.peek().clone(),
            "Expect Expression".to_owned(),
        ));
    }

    fn match_(&mut self, token_types: &Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type.to_owned()) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, token_type: TokenType, message: &String) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        } else {
            return Err(ParseError::new(self.peek().clone(), message.to_owned()));
        }
    }

    fn syncronize(&mut self) {
        self.advance();

        while !self.at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl ParseError {
    pub fn new(token: Token, message: String) -> Self {
        ParseError { token, message }
    }
}
