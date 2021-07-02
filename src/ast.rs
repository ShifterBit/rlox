use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Expr>),
}
#[derive(Debug)]
pub enum Expr {
    // Literal Values
    Literal(Literal),

    // Compound Expressions
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
}
