pub enum Expr {
    Blade(f64, Vec<usize>),
    Geometric(Box<Expr>, Box<Expr>),
    Exterior(Box<Expr>, Box<Expr>),
    Regressive(Box<Expr>, Box<Expr>),
    LeftContraction(Box<Expr>, Box<Expr>),
    RightContraction(Box<Expr>, Box<Expr>),
    Inner(Box<Expr>, Box<Expr>),
    Scalar(Box<Expr>, Box<Expr>),
}
