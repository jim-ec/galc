pub enum Expr {
    Blade(f64, Basis),
    Binary(Binary, Box<Expr>, Box<Expr>),
    Unary(Unary, Box<Expr>),
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
    Grade,
    AntiGrade,
}

#[derive(Clone)]
pub enum Basis {
    Pseudoscalar,
    Vectors(Vec<usize>),
}
