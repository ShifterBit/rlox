use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Box<Expr>),
    Print(Box<Expr>),
    Var(Token, Box<Option<Expr>>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
}
#[derive(Debug)]
pub enum Expr {
    // Literal Values
    Literal(Literal),

    // Compound Expressions
    Assignment(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Variable(Token),
}
