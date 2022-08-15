mod tokenize;

use chumsky::prelude::*;
use itertools::Itertools;

use crate::interpret::expr::{Binary, Expr, Unary};

use self::tokenize::Token;

pub fn parse(string: &str) -> Option<Expr> {
    match tokenize::tokenize(string) {
        Ok(tokens) => match expr_parser().parse(tokens) {
            Ok(expr) => Some(expr),
            Err(errors) => {
                for error in errors {
                    println!(
                        "Parsing error, expected one of: {}",
                        error
                            .expected()
                            .flatten()
                            .map(|token| format!("{token}"))
                            .join(", ")
                    );
                    if let Some(found) = error.found() {
                        println!("But got: {found}")
                    }
                }
                None
            }
        },
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

fn expr_parser<'a>() -> impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a {
    recursive(|expr| binary_parser(expr.clone()))
        .padded_by(just(Token::Whitespace).repeated())
        .then_ignore(end())
        .boxed()
}

fn operand_parser<'a>(
    expr: impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a {
    select! {
        Token::Number(number) => Expr::Number(number.parse().unwrap()),
        Token::Basis(basis) => Expr::Basis(basis),
        Token::Identifier(identifier) if identifier == "i" => Expr::Pseudoscalar,
        Token::Identifier(identifier) => Expr::Unknown(identifier),
        Token::Bottom => Expr::Bottom,
    }
    .or(expr
        .clone()
        .delimited_by(just(Token::ParenOpen), just(Token::ParenClose)))
    .or(expr
        .clone()
        .delimited_by(just(Token::BracketOpen), just(Token::BracketClose))
        .map(|expr| Expr::Norm(Box::new(expr))))
    .boxed()
}

fn binary_parser<'a>(
    expr: impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a {
    let binary: BoxedParser<Token, Expr, Simple<Token>> = select! {
        Token::Minus => Unary::Neg,
        Token::Asteriks => Unary::Dual,
        Token::Tilde => Unary::Reverse,
        Token::Excl => Unary::Conjugate,
        Token::Hat => Unary::Involution,
    }
    .repeated()
    .then(operand_parser(expr))
    .foldr(|op, rhs| Expr::Unary(op, Box::new(rhs)))
    .boxed();

    let binary: BoxedParser<Token, Expr, Simple<Token>> = binary
        .clone()
        .then(
            select! { Token::Hat => Binary::Power }
                .then(binary)
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| Expr::Binary(op, Box::new(lhs), Box::new(rhs)))
        .boxed();

    let binary: BoxedParser<Token, Expr, Simple<Token>> = binary
        .clone()
        .then(just(Token::Whitespace).ignore_then(binary).repeated())
        .foldl(|lhs, rhs| Expr::Binary(Binary::Geometric, Box::new(lhs), Box::new(rhs)))
        .boxed();

    let binary: BoxedParser<Token, Expr, Simple<Token>> = binary
        .clone()
        .then(
            just(Token::Whitespace)
                .ignore_then(select! {
                    Token::Wedge => Binary::Exterior,
                    Token::AntiWedge => Binary::Regressive,
                    Token::LeftContraction => Binary::LeftContraction,
                    Token::RightContraction => Binary::RightContraction,
                    Token::InnerProduct => Binary::Inner,
                    Token::Asteriks => Binary::Scalar,
                    Token::Solidus => Binary::Divide,
                })
                .then_ignore(just(Token::Whitespace))
                .then(binary)
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| Expr::Binary(op, Box::new(lhs), Box::new(rhs)))
        .boxed();

    let binary: BoxedParser<Token, Expr, Simple<Token>> = binary
        .clone()
        .then(
            just(Token::Whitespace)
                .ignore_then(select! {
                    Token::Plus => Binary::Add,
                    Token::Minus => Binary::Sub,
                })
                .then_ignore(just(Token::Whitespace))
                .then(binary)
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| Expr::Binary(op, Box::new(lhs), Box::new(rhs)))
        .boxed();

    binary
}
