use crate::ast::Expr;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

pub fn interpret(expr: &Expr) {
    let value = evaluate(expr).unwrap();
    println!("{}", stringify(value));
}

fn evaluate(expr: &Expr) -> Result<Literal, RuntimeError> {
    match expr {
        Expr::Literal(literal) => evaluate_literal(literal.to_owned()),
        Expr::Unary(op, e) => evaluate_unary(op.to_owned(), &e),
        Expr::Binary(lhs, op, rhs) => evaluate_binary(&lhs, op.to_owned(), &rhs),
        Expr::Grouping(e) => evaluate(&e),
    }
}

fn evaluate_literal(expr: Literal) -> Result<Literal, RuntimeError> {
    Ok(expr)
}

fn evaluate_unary(op: Token, expr: &Expr) -> Result<Literal, RuntimeError> {
    let right = evaluate(expr)?;

    match op.token_type {
        TokenType::Bang => Ok(Literal::Bool(is_truthy(&right))),
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

fn evaluate_binary(left: &Expr, op: Token, right: &Expr) -> Result<Literal, RuntimeError> {
    let lhs: Literal = evaluate(left)?;
    let rhs: Literal = evaluate(right)?;

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
            (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Number(lhs - rhs)),
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
            (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Number(lhs / rhs)),
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
fn is_truthy(value: &Literal) -> bool {
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

#[derive(Debug)]
struct RuntimeError {
    token: Token,
    message: String,
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
