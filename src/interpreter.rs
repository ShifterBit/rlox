use crate::ast::{Expr, Stmt};
use crate::environment::Environment;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;
use crate::Lox;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Box<Stmt>>) {
        for statement in statements {
            self.interpret_statement(statement).unwrap();
        }
    }

    fn interpret_statement(
        &mut self,
        statement: Box<Stmt>,
    ) -> Result<Option<Literal>, RuntimeError> {
        match *statement {
            Stmt::Expr(expr) => {
                let expression = self.evaluate(expr);
                match expression {
                    Ok(l) => {
                        // match l.clone() {
                        //     Literal::Bool(b) => {
                        //         println!("{}", b);
                        //     }
                        //     Literal::Number(n) => {
                        //         println!("{}", n);
                        //     }
                        //     Literal::String(s) => {
                        //         println!("\"{}\"", s);
                        //     }
                        //     Literal::Nil => {
                        //         println!("nil");
                        //     }
                        // };
                        // None
                        Ok(Some(l))
                    }
                    Err(e) => {
                        Lox::runtime_error(e);
                        Ok(None)
                    }
                }
            }
            Stmt::While(condition, body) => {
                while Interpreter::is_truthy(self.evaluate(condition.clone())?) {
                    self.interpret_statement(body.to_owned())?;
                }
                Ok(None)
            }

            Stmt::If(condition, then_branch, else_branch) => {
                if Interpreter::is_truthy(self.evaluate(condition).unwrap()) {
                    self.interpret_statement(then_branch)
                } else if else_branch.is_some() {
                    self.interpret_statement(else_branch.unwrap())
                } else {
                    Ok(None)
                }
            }
            Stmt::Block(s) => {
                self.environment = Environment::from(self.environment.clone());
                for statement in s.iter() {
                    self.interpret_statement(Box::new(statement.clone()))
                        .unwrap();
                }

                if let Some(enclosing) = self.environment.enclosing.clone() {
                    self.environment = enclosing.clone().take();
                }
                Ok(None)
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr);
                match value {
                    Ok(l) => {
                        match l {
                            Literal::Bool(b) => {
                                println!("{}", b);
                            }
                            Literal::Number(n) => {
                                println!("{}", n);
                            }
                            Literal::String(s) => {
                                println!("\"{}\"", s);
                            }
                            Literal::Nil => {
                                println!("nil");
                            }
                        };
                        Ok(None)
                    }
                    Err(e) => {
                        Lox::runtime_error(e);
                        Ok(None)
                    }
                }
            }
            Stmt::Var(name, initializer) => {
                let mut value: Literal = Literal::Nil;
                match *initializer {
                    Some(e) => {
                        let f = self.evaluate(Box::new(e));
                        match f {
                            Ok(e) => value = e,
                            Err(e) => Lox::runtime_error(e),
                        }
                    }
                    None => {}
                }
                self.environment.define(&name.lexeme, value);
                Ok(None)
            }
        }
    }

    fn evaluate(&mut self, expr: Box<Expr>) -> Result<Literal, RuntimeError> {
        match *expr {
            Expr::Literal(literal) => self.evaluate_literal(literal.to_owned()),
            Expr::Logical(lhs, op, rhs) => {
                let left = self.evaluate(lhs)?;
                if op.token_type == TokenType::Or {
                    if Interpreter::is_truthy(left.clone()) {
                        return Ok(left);
                    }
                } else {
                    if !Interpreter::is_truthy(left.clone()) {
                        return Ok(left);
                    }
                }
                self.evaluate(rhs)
            }
            Expr::Unary(op, e) => self.evaluate_unary(op.to_owned(), e),
            Expr::Binary(lhs, op, rhs) => self.evaluate_binary(lhs, op.to_owned(), rhs),
            Expr::Grouping(e) => self.evaluate(e),
            Expr::Variable(e) => self.environment.get(e),
            Expr::Assignment(t, e) => {
                let value = self.evaluate(e)?;
                self.environment.assign(t, value.clone()).unwrap();
                Ok(value)
            }
        }
    }

    fn evaluate_literal(&mut self, expr: Literal) -> Result<Literal, RuntimeError> {
        Ok(expr)
    }

    fn evaluate_unary(&mut self, op: Token, expr: Box<Expr>) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(expr)?;

        match op.token_type {
            TokenType::Bang => Ok(Literal::Bool(Interpreter::is_truthy(right))),
            TokenType::Minus => match right {
                Literal::Number(f) => Ok(Literal::Number(f * -1 as f64)),
                _ => Err(RuntimeError::new(
                    op,
                    "Invalid negation operand.".to_owned(),
                )),
            },
            _ => Err(RuntimeError::new(op, "Invalid unary operand.".to_owned())),
        }
    }

    fn evaluate_binary(
        &mut self,
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    ) -> Result<Literal, RuntimeError> {
        let lhs: Literal = self.evaluate(left)?;
        let rhs: Literal = self.evaluate(right)?;

        match op.token_type {
            TokenType::Greater => match (lhs, rhs) {
                (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Bool(lhs > rhs)),
                _ => Err(RuntimeError::new(
                    op,
                    "Operands must be numbers.".to_owned(),
                )),
            },
            TokenType::GreaterEqual => match (lhs, rhs) {
                (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Bool(lhs >= rhs)),
                _ => Err(RuntimeError::new(
                    op,
                    "Operands must be numbers.".to_owned(),
                )),
            },
            TokenType::LessEqual => match (lhs, rhs) {
                (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Bool(lhs <= rhs)),
                _ => Err(RuntimeError::new(
                    op,
                    "Operands must be numbers.".to_owned(),
                )),
            },
            TokenType::EqualEqual => match (lhs, rhs) {
                (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Bool(lhs == rhs)),
                (Literal::String(lhs), Literal::String(rhs)) => Ok(Literal::Bool(lhs == rhs)),
                (Literal::String(_), Literal::Number(_)) => Ok(Literal::Bool(false)),
                (Literal::Number(_), Literal::String(_)) => Ok(Literal::Bool(false)),
                (Literal::Bool(lhs), Literal::Bool(rhs)) => Ok(Literal::Bool(lhs == rhs)),
                (Literal::Nil, Literal::Nil) => Ok(Literal::Bool(true)),
                _ => Ok(Literal::Bool(false)),
            },
            TokenType::Less => match (lhs, rhs) {
                (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Bool(lhs < rhs)),
                _ => Err(RuntimeError::new(
                    op,
                    "Operands must be numbers.".to_owned(),
                )),
            },
            TokenType::Minus => match (lhs, rhs) {
                (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Number(lhs - rhs)),
                _ => Err(RuntimeError::new(
                    op,
                    "Operands must be numbers.".to_owned(),
                )),
            },
            TokenType::Plus => match (lhs, rhs) {
                (Literal::Number(lhs), Literal::Number(rhs)) => {
                    Ok(Literal::Number(lhs.clone() + rhs.clone()))
                }
                (Literal::String(lhs), Literal::String(rhs)) => Ok(Literal::String(lhs + &rhs)),
                _ => Err(RuntimeError::new(
                    op,
                    "Operands must be either two numbers or two strings.".to_owned(),
                )),
            },
            TokenType::Slash => match (lhs, rhs) {
                (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Number(lhs / rhs)),
                _ => Err(RuntimeError::new(
                    op,
                    "Operands must be numbers.".to_owned(),
                )),
            },
            TokenType::Star => match (lhs, rhs) {
                (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Number(lhs * rhs)),
                _ => Err(RuntimeError::new(
                    op,
                    "Operands must be numbers.".to_owned(),
                )),
            },
            _ => Err(RuntimeError::new(
                op,
                "Operands must be either numbers or strings.".to_owned(),
            )),
        }
    }

    fn is_truthy(value: Literal) -> bool {
        match value {
            Literal::Nil => false,
            Literal::Bool(b) => b,
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        RuntimeError {
            token,
            message: message.to_owned(),
        }
    }
}

impl error::Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.token.to_string(), self.message)
    }
}
