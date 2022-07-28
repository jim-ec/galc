pub enum Expr {
    Blade(f64, Basis),
    Binary(Binary, Box<Expr>, Box<Expr>),
    Unary(Unary, Box<Expr>),
    Norm(Box<Expr>),
    Application(String, Vec<Expr>),
    Variable(String),
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

#[derive(Clone)]
pub enum Basis {
    Pseudoscalar,
    Vectors(Vec<usize>),
}
