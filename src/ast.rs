use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum Expr {
    // Literal Values
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Literal(Literal),

    // Compound Expressions
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
}
