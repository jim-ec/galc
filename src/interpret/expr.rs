use crate::parse::span::Spanned;

use num::BigRational;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(BigRational),
    Pseudoscalar,
    Basis(Vec<usize>),
    Unknown(String),
    Bottom,
    Binary(Binary, Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Power(Box<Spanned<Expr>>, isize),
    Unary(Unary, Box<Spanned<Expr>>),
    Norm(Box<Spanned<Expr>>),
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
