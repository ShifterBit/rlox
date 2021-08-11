use crate::token::{Literal, Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Box<Expr>),
    Print(Box<Expr>),
    Var(Token, Box<Option<Expr>>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>),
}
#[derive(Debug, Clone)]
pub enum Expr {
    // Literal Values
    Literal(Literal),

    // Compound Expressions
    Assignment(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Variable(Token),
}
