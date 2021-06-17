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

    fn expression(&self) {
        self.equality()
    }

    fn equality(&self) {
        let mut expr: Expr = self.comparison();
        while self.match_(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        expr
    }

    fn comparison(&self) {
        let mut expr = self.term();
        while self.match_(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::LessEqual,
            TokenType::Less,
        ]) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        expr
    }

    fn term(&self) {
        let mut expr = self.factor();
        while self.match_(vec![TokenType::Plus, TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        expr
    }

    fn factor(&self) {
        let mut expr = self.unary();
        while self.match_(&vec![TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        expr
    }

    fn unary(&self) {
        if self.match_(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            Expr::Unary(operator, Box::new(right))
        }
        self.primary()
    }

    fn primary(&self) {
        if self.match_(&vec![TokenType::False]) {
            Expr::Boolean(Literal::Bool(false))
        }

        if self.match_(&vec![TokenType::True]) {
            Expr::Boolean(Literal::Bool(true))
        }

        if self.match_(&vec![TokenType::Nil]) {
            Expr::Literal(Literal::Nil)
        }

        if self.match_(&vec![TokenType::Number, TokenType::String]) {
            Expr::Literal(self.previous().literal)
        }

        if self.match_(&vec![TokenType::LeftParen]) {
            let expr: Expr = self.expression();
            self.advance();
            Expr::Grouping(expr)
        }
    }

    fn match_(&self, token_types: &Vec<TokenType>) {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                true
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) {
        if self.at_end() {
            false
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }
}
