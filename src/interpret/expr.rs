#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Pseudoscalar,
    Basis(Vec<usize>),
    Binary(Binary, Box<Expr>, Box<Expr>),
    Power(Box<Expr>, isize),
    Unary(Unary, Box<Expr>),
    Norm(Box<Expr>),
    Unknown(String),
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub enum Binary {
    Geometric,
    Exterior,
    Regressive,
    LeftContraction,
    RightContraction,
    Inner,
    Scalar,
    Divide,
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy)]
pub enum Unary {
    Neg,
    Dual,
    Reverse,
    Inverse,
    Involution,
    Conjugate,
}
