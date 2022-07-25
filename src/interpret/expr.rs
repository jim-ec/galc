pub enum Expr {
    Blade(f64, Basis),
    Binary(Binary, Box<Expr>, Box<Expr>),
    Unary(Unary, Box<Expr>),
    Application(String, Vec<Expr>),
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
