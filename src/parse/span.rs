use std::ops::Range;

use crate::parse::{Expr, Token};

#[derive(Debug, Clone)]
pub struct Spanned<T>(pub T, pub Range<usize>);

pub fn translate_spans(expr: Spanned<Expr>, tokens: &Vec<Spanned<Token>>) -> Spanned<Expr> {
    let start = tokens[expr.1.start].1.start;
    let end = tokens[expr.1.end - 1].1.end;
    let span = start..end;

    let expr = match expr.0 {
        Expr::Binary(op, lhs, rhs) => Expr::Binary(
            op,
            Box::new(translate_spans(*lhs, tokens)),
            Box::new(translate_spans(*rhs, tokens)),
        ),
        Expr::Power(expr, exp) => Expr::Power(Box::new(translate_spans(*expr, tokens)), exp),
        Expr::Unary(op, expr) => Expr::Unary(op, Box::new(translate_spans(*expr, tokens))),
        Expr::Norm(expr) => Expr::Norm(Box::new(translate_spans(*expr, tokens))),
        expr => expr,
    };

    Spanned(expr, span)
}
