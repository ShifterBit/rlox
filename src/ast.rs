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

#[derive(Debug)]
pub enum UnaryOperator {
    Bang,
    Minus,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Slash,
    Star,
    Plus,
    Minus,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    BangEqual,
    EqualEqual,
}

#[derive(Debug)]
pub enum Expr {
    // Literal Values
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,

    // Compound Expressions
    Binary(Box<Expr>, BinaryOperator, Box<Expr>),
    Unary(UnaryOperator, Box<Expr>),
    Grouping(Box<Expr>),
}
