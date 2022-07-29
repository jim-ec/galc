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
                            .map(|token| format!("{:?}", token))
                            .join(", ")
                    );
                    if let Some(found) = error.found() {
                        println!("But got: {:?}", found)
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
        Token::Identifier(identifier) => Expr::Identifier(identifier),
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

fn unary_parser<'a>(
    expr: impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a {
    select! {
        Token::Operator(operator) if operator == "-" => Unary::Neg,
        Token::Operator(operator) if operator == "!" => Unary::Dual,
        Token::Operator(operator) if operator == "~" => Unary::Reverse,
    }
    .repeated()
    .then(operand_parser(expr))
    .foldr(|op, rhs| Expr::Unary(op, Box::new(rhs)))
    .boxed()
}

fn binary_parser<'a>(
    expr: impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a {
    let binary = unary_parser(expr.clone());

    let binary: BoxedParser<Token, Expr, Simple<Token>> = binary
        .clone()
        .then(
            just(Token::Whitespace)
                .ignore_then(
                    select! { Token::Operator(operator) if operator == "^" => Binary::Power },
                )
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
                .or_not()
                .ignore_then(binary)
                .repeated(),
        )
        .foldl(|lhs, rhs| Expr::Binary(Binary::Geometric, Box::new(lhs), Box::new(rhs)))
        .boxed();

    let binary: BoxedParser<Token, Expr, Simple<Token>> = binary
        .clone()
        .then(
            just(Token::Whitespace)
                .ignore_then(select! {
                    Token::Operator(operator) if operator == r"/\" => Binary::Exterior,
                    Token::Operator(operator) if operator == r"\/" => Binary::Regressive,
                    Token::Operator(operator) if operator == r">>" => Binary::LeftContraction,
                    Token::Operator(operator) if operator == r"<<" => Binary::RightContraction,
                    Token::Operator(operator) if operator == r"|" => Binary::Inner,
                    Token::Operator(operator) if operator == r"*" => Binary::Scalar,
                    Token::Operator(operator) if operator == r"/" => Binary::Divide,
                })
                .then_ignore(just(Token::Whitespace))
                .then(binary)
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| Expr::Binary(op, Box::new(lhs), Box::new(rhs)))
        .boxed();

    binary
}
