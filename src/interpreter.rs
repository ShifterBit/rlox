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
            self.interpret_statement(statement);
        }
    }

    fn interpret_statement(&mut self, statement: Box<Stmt>) -> Option<Literal> {
        match *statement {
            Stmt::Expr(expr) => {
                let expression = self.evaluate(&expr);
                match expression {
                    Ok(l) => {
                        match l.clone() {
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
                        // None
                        Some(l)
                    }
                    Err(e) => {
                        Lox::runtime_error(e);
                        None
                    }
                }
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(&expr);
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
                        // println!("{:?}", l);
                        None
                    }
                    Err(e) => {
                        Lox::runtime_error(e);
                        None
                    }
                }
            }
            Stmt::Var(name, initializer) => {
                let mut value: Literal = Literal::Nil;
                match *initializer {
                    Some(e) => {
                        let f = self.evaluate(&e);
                        match f {
                            Ok(e) => value = e,
                            Err(e) => Lox::runtime_error(e),
                        }
                    }
                    None => {}
                }
                self.environment.define(&name.lexeme, value);
                None
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Literal(literal) => self.evaluate_literal(literal.to_owned()),
            Expr::Unary(op, e) => self.evaluate_unary(op.to_owned(), &e),
            Expr::Binary(lhs, op, rhs) => self.evaluate_binary(&lhs, op.to_owned(), &rhs),
            Expr::Grouping(e) => self.evaluate(&e),
            Expr::Variable(e) => self.environment.get(e.clone()),
            Expr::Assignment(t, e) => {
                let value = self.evaluate(&e)?;
                self.environment.assign(t.to_owned(), value.clone())?;
                Ok(value)
            }
        }
    }

    fn evaluate_literal(&mut self, expr: Literal) -> Result<Literal, RuntimeError> {
        Ok(expr)
    }

    fn evaluate_unary(&mut self, op: Token, expr: &Expr) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(expr)?;

        match op.token_type {
            TokenType::Bang => Ok(Literal::Bool(self.is_truthy(&right))),
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
        left: &Expr,
        op: Token,
        right: &Expr,
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

    fn stringify(value: &Literal) -> String {
        match value {
            Literal::Nil => "nil".to_owned(),
            Literal::Number(f) => match f {
                f if f - f.floor() == 0.0 => {
                    let mut float = f.to_string();
                    float.pop();
                    float.pop();
                    float
                }
                _ => f.to_string(),
            },
            Literal::String(s) => s.to_owned(),
            Literal::Bool(b) => b.to_string(),
        }
    }

    fn is_truthy(&self, value: &Literal) -> bool {
        match value {
            Literal::Nil => false,
            Literal::Bool(b) => *b,
            _ => true,
        }
    }

    fn is_equal(lhs: &Literal, rhs: &Literal) -> bool {
        match (lhs, rhs) {
            (Literal::Nil, Literal::Nil) => true,
            (Literal::Nil, _) => false,
            _ => lhs.eq(&rhs),
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
