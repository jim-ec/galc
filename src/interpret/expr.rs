pub enum Expr {
    Number(f64),
    Pseudoscalar,
    Basis(Vec<usize>),
    Binary(Binary, Box<Expr>, Box<Expr>),
    Unary(Unary, Box<Expr>),
    Norm(Box<Expr>),
    Unknown(String),
    Bottom,
}

#[derive(Clone, Copy)]
pub enum Binary {
    Geometric,
    Exterior,
    Regressive,
    LeftContraction,
    RightContraction,
    Inner,
    Scalar,
    Divide,
    Power,
    Add,
    Sub,
}

#[derive(Clone, Copy)]
pub enum Unary {
    Neg,
    Dual,
    Reverse,
    Inverse,
    Involution,
    Conjugate,
}
