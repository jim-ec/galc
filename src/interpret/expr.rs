pub enum Expr {
    Number(f64),
    Pseudoscalar,
    Basis(Vec<usize>),
    Binary(Binary, Box<Expr>, Box<Expr>),
    Unary(Unary, Box<Expr>),
    Norm(Box<Expr>),
    Identifier(String),
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
}

#[derive(Clone, Copy)]
pub enum Unary {
    Neg,
    Dual,
    Reverse,
    Inverse,
    Involute,
    Conjugate,
}
