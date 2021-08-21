use crate::ast::*;
use crate::token::{Literal, Token, TokenType};
use crate::Lox;

// ------------ Syntax Grammar ------------
//
//
// -------- Program --------
// program          -> declaration* EOF
//
// -------- Declarations --------
// declaration      -> varDeclaration
//                   | statement ;
// -------- Statements --------
// statement        -> exprStmt
//                   | ifStmt
//                   | whileStmt
//                   | printStmt
//                   | block ;
// block            -> "{" declaration* "}" ;
// exprStmt         -> expression ";" ;
// printStmt        -> "print" expression ";" ;
// -------- EXPRESSIONS --------
// expression       -> assignment ;
// assignment       -> IDENTIFIER "=" assignment
//                   | logic_or ;
// logic_or         -> logic_and ("or" logic_and)* ;
// logic_and        -> equality ("or" equality)* ;
// equality         -> comparison ( ("!=" | "==" ) comparison )* ;
// comparison       -> term ( (">" | ">=" | "<=" | "<" ) term )* ;
// unary            -> ( "-" | "!" ) unary | primary ;
// term             -> factor ( ("-" | "+") factor)* ;
// factor           -> unary ( ("/" | "*") unary)* ;
// primary          ->  NUMBER | String | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Vec<Box<Stmt>> {
        let mut statements: Vec<Box<Stmt>> = Vec::new();
        while !self.at_end() {
            match self.declaration() {
                Some(s) => statements.push(Box::new(s)),
                None => {}
            }
        }

        return statements;
    }
    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_(&vec![TokenType::Var]) {
            match self.var_declaration() {
                Ok(s) => Some(s),
                Err(e) => {
                    self.syncronize();
                    Lox::parse_error(e);
                    None
                }
            }
        } else {
            match self.statement() {
                Ok(s) => Some(s),
                Err(e) => {
                    self.syncronize();
                    Lox::parse_error(e);
                    None
                }
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name: Token =
            self.consume(TokenType::Identifier, &"Expect variable name.".to_owned())?;
        let mut initializer = Expr::Literal(Literal::Nil);
        if self.match_(&vec![TokenType::Equal]) {
            initializer = self.expression()?;
        }
        self.consume(
            TokenType::Semicolon,
            &"Expect ';' after variable declaration.".to_owned(),
        )?;
        Ok(Stmt::Var(name, Box::new(Some(initializer))))
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_(&vec![TokenType::If]) {
            self.if_statement()
        } else if self.match_(&vec![TokenType::For]) {
            self.for_statement()
        } else if self.match_(&vec![TokenType::Print]) {
            self.print_statement()
        } else if self.match_(&vec![TokenType::While]) {
            self.while_statement()
        } else if self.match_(&vec![TokenType::LeftBrace]) {
            self.block_statement()
        } else {
            self.expression_statement()
        }
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, &"Expect '(' after 'while'.".to_owned())?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            &"Expect ')' after 'condition'".to_owned(),
        )?;
        let body = self.statement()?;
        Ok(Stmt::While(Box::new(condition), Box::new(body)))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, &"Expect '(' after 'for'.".to_owned())?;
        let initializer: Option<Stmt>;

        if self.match_(&vec![TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_(&vec![TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?); 
        }

        let mut condition = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, &"Expect ';' after loop condition.".to_owned())?;

        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }

        self.consume(TokenType::RightParen, &"Expect ')' after for clauses.".to_owned())?;
        let mut body = self.statement()?;

        if increment.is_some() {
            body = Stmt::Block(vec![body, Stmt::Expr(Box::new(increment.unwrap()))])
        }

        if condition.is_none() {
            condition = Some(Expr::Literal(Literal::Bool(true)));
            }

        body = Stmt::While(Box::new(condition.unwrap()), Box::new(body));

        if initializer.is_some() {
            body = Stmt::Block(vec![initializer.unwrap(), body]);
            }
        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, &"Expect '(' after 'if'.".to_owned())?;
        let condition = self.expression().unwrap();
        self.consume(
            TokenType::RightParen,
            &"Expect ')' after condition.".to_owned(),
        )?;

        let then_branch = self.statement()?;
        if self.match_(&vec![TokenType::Else]) {
            let else_branch = self.statement()?;
            Ok(Stmt::If(
                Box::new(condition),
                Box::new(then_branch),
                Some(Box::new(else_branch)),
            ))
        } else {
            Ok(Stmt::If(Box::new(condition), Box::new(then_branch), None))
        }
    }

    fn block_statement(&mut self) -> Result<Stmt, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();
        loop {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
            if self.check(TokenType::RightBrace) || self.at_end() {
                break;
            }
        }
        self.consume(TokenType::RightBrace, &"Expect '}' after block.".to_owned())?;
        Ok(Stmt::Block(statements))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression();
        match value {
            Ok(e) => {
                let semicolon_exists =
                    self.consume(TokenType::Semicolon, &"Expect ';' after value.".to_owned());
                match semicolon_exists {
                    Ok(_) => Ok(Stmt::Print(Box::new(e))),
                    Err(e) => Err(e),
                }
            }

            Err(e) => Err(e),
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression();
        match expr {
            Ok(e) => {
                let semicolon_exists = self.consume(
                    TokenType::Semicolon,
                    &"Expect ';' after expression.".to_owned(),
                );
                match semicolon_exists {
                    Ok(_) => Ok(Stmt::Expr(Box::new(e))),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.match_(&vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            match expr {
                Expr::Variable(t) => Ok(Expr::Assignment(t, Box::new(value))),
                _ => Err(ParseError::new(
                    equals,
                    "Invalid assignment target.".to_owned(),
                )),
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;
        while self.match_(&vec![TokenType::Or]) {
            let operator: Token = self.previous();
            let right: Expr = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while self.match_(&vec![TokenType::Or]) {
            let operator: Token = self.previous();
            let right: Expr = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
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

        if self.match_(&vec![TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous()));
        }

        if self.match_(&vec![TokenType::LeftParen]) {
            let expr: Expr = self.expression()?;
            let right_paren = self.consume(
                self.peek().clone().token_type,
                &"Expect ')' after expression.".to_owned(),
            );
            match right_paren {
                Ok(_) => return Ok(Expr::Grouping(Box::new(expr))),
                Err(e) => return Err(e),
            }
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
