use chumsky::{error::Simple, prelude::*};
use itertools::Itertools;

use crate::interpret::expr::{Basis, Binary, Expr, Unary};

type ExprParser<'a, T> = BoxedParser<'a, char, T, Simple<char>>;

pub fn parse(string: &str) -> Option<Expr> {
    let parser = expr_parser();
    match parser.parse(string) {
        Ok(expr) => Some(expr),
        Err(errors) => {
            for error in errors {
                println!(
                    "Parsing error, expected one of: {}",
                    error.expected().flatten().join(", ")
                );
                if let Some(found) = error.found() {
                    println!("But got: {found}")
                }
            }
            None
        }
    }
}

fn expr_parser<'a>() -> impl Parser<char, Expr, Error = Simple<char>> + Clone + 'a {
    recursive(|expr| binary_parser(expr.clone()))
        .then_ignore(end())
        .boxed()
}

fn number_parser<'a>() -> impl Parser<char, f64, Error = Simple<char>> + Clone + 'a {
    text::int(10)
        .then(just('.').ignore_then(text::int(10)).or_not())
        .map(|(mut number, frac)| {
            if let Some(frac) = frac {
                number.push_str(".");
                number += &frac;
            }
            // TODO: Throw parsing error instead of panicking.
            let number: f64 = number.parse().unwrap();
            number
        })
        .padded()
        .boxed()
}

fn basis_parser<'a>() -> impl Parser<char, Basis, Error = Simple<char>> + Clone + 'a {
    just('i')
        .to(Basis::Pseudoscalar)
        .or(just('e')
            .ignore_then(
                select! {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                }
                .repeated()
                .at_least(1),
            )
            .map(Basis::Vectors))
        .padded()
        .boxed()
}

fn blade_parser<'a>() -> impl Parser<char, Expr, Error = Simple<char>> + Clone + 'a {
    choice((
        number_parser()
            .then(basis_parser())
            .map(|(number, basis)| Expr::Blade(number, basis)),
        basis_parser().map(|basis| Expr::Blade(1.0, basis)),
        number_parser().map(|number| Expr::Blade(number, Basis::Vectors(vec![]))),
    ))
    .boxed()
}

fn variable_parser<'a>() -> impl Parser<char, Expr, Error = Simple<char>> + Clone + 'a {
    text::ident().map(Expr::Variable)
}

fn application_parser<'a>(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone + 'a,
) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + 'a {
    text::ident()
        .then(
            expr.clone()
                .separated_by(just(','))
                .delimited_by(just('('), just(')')),
        )
        .map(|(name, arguments)| Expr::Application(name, arguments))
        .boxed()
}

fn atom_parser<'a>(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone + 'a,
) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + 'a {
    choice((
        application_parser(expr.clone()),
        variable_parser(),
        blade_parser(),
        expr.clone().delimited_by(just('('), just(')')),
        expr.clone()
            .delimited_by(just('|'), just('|'))
            .map(|expr| Expr::Norm(Box::new(expr))),
    ))
    .boxed()
}

fn unary_parser<'a>(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone + 'a,
) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + 'a {
    select! {
        '-' => Unary::Neg,
        '!' => Unary::Dual,
        '~' => Unary::Reverse,
        '$' => Unary::Inverse,
    }
    .padded()
    .repeated()
    .then(atom_parser(expr))
    .foldr(|op, rhs| Expr::Unary(op, Box::new(rhs)))
    .boxed()
}

fn binary_parser<'a>(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone + 'a,
) -> impl Parser<char, Expr, Error = Simple<char>> + Clone + 'a {
    let binary = unary_parser(expr.clone());

    let binary: BoxedParser<char, Expr, Simple<char>> = binary
        .clone()
        .then(
            choice((just("**").to(Binary::Exponentiation),))
                .padded()
                .then(binary)
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| Expr::Binary(op, Box::new(lhs), Box::new(rhs)))
        .boxed();

    let binary: BoxedParser<char, Expr, Simple<char>> = binary
        .clone()
        .then(
            choice((
                just("*").to(Binary::Geometric),
                just("^").to(Binary::Exterior),
                just("&").to(Binary::Regressive),
                just(">>").to(Binary::LeftContraction),
                just("<<").to(Binary::RightContraction),
                just("<>").to(Binary::Inner),
                just("%").to(Binary::Scalar),
                just("/").to(Binary::Divide),
            ))
            .padded()
            .then(binary)
            .repeated(),
        )
        .foldl(|lhs, (op, rhs)| Expr::Binary(op, Box::new(lhs), Box::new(rhs)))
        .boxed();

    binary
}
