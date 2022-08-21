pub mod span;
mod token;

use chumsky::prelude::*;

use crate::interpret::expr::{Binary, Expr, Unary};

use self::{span::Spanned, token::Token};

type Range = std::ops::Range<usize>;

fn report_error(span: Range) {
    println!("Syntax error at {}..{}", span.start, span.end);
}

pub fn parse(string: &str) -> Option<Spanned<Expr>> {
    match token::tokenize(string) {
        Ok(spanned_tokens) => {
            let tokens: Vec<Token> = spanned_tokens
                .iter()
                .map(|Spanned(item, _)| item)
                .cloned()
                .collect();

            match expr_parser().parse(tokens) {
                Ok(expr) => Some(expr),
                Err(errors) => {
                    for error in errors {
                        let span = Range {
                            start: spanned_tokens[error.span().start].1.start,
                            end: spanned_tokens[error.span().end - 1].1.end,
                        };
                        report_error(span);
                    }
                    None
                }
            }
        }
        Err(errors) => {
            for error in errors {
                report_error(error.span());
            }
            None
        }
    }
}

fn expr_parser<'a>() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone + 'a {
    recursive(|expr| binary_parser(expr.clone()))
        .padded_by(just(Token::Whitespace).repeated())
        .then_ignore(end())
        .boxed()
}

fn operand_parser<'a>(
    expr: impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone + 'a,
) -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone + 'a {
    select! {
        Token::Number(number) => Expr::Number(number.parse().unwrap()),
        Token::Basis(basis) => Expr::Basis(basis),
        Token::Identifier(identifier) if identifier == "i" => Expr::Pseudoscalar,
        Token::Identifier(identifier) => Expr::Unknown(identifier),
        Token::Bottom => Expr::Bottom,
    }
    .map_with_span(Spanned)
    .or(expr
        .clone()
        .delimited_by(just(Token::ParenOpen), just(Token::ParenClose)))
    .or(expr
        .clone()
        .delimited_by(just(Token::BracketOpen), just(Token::BracketClose))
        .map(|expr| Expr::Norm(Box::new(expr)))
        .map_with_span(Spanned))
    .boxed()
}

fn binary_parser<'a>(
    expr: impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone + 'a,
) -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone + 'a {
    let binary = select! {
        Token::Minus => Unary::Neg,
        Token::Asteriks => Unary::Dual,
        Token::Tilde => Unary::Reverse,
        Token::Excl => Unary::Conjugate,
        Token::Hat => Unary::Involution,
    }
    .map_with_span(Spanned)
    .repeated()
    .then(operand_parser(expr))
    .foldr(|op, rhs| {
        Spanned(
            Expr::Unary(op.0, Box::new(rhs.clone())),
            op.1.start..rhs.1.end,
        )
    })
    .boxed();

    let binary = binary
        .clone()
        .then(
            just(Token::Hat)
                .ignore_then(
                    just(Token::Minus)
                        .repeated()
                        .then(select! { Token::Number(n) => n.parse().unwrap() })
                        .foldr(|_, n: isize| -n),
                )
                .map_with_span(Spanned)
                .repeated(),
        )
        .foldl(|lhs, rhs| {
            Spanned(
                Expr::Power(Box::new(lhs.clone()), rhs.0),
                lhs.1.start..rhs.1.end,
            )
        })
        .boxed();

    let binary = binary
        .clone()
        .then(
            just(Token::Whitespace)
                .or_not()
                .ignore_then(binary)
                .repeated(),
        )
        .foldl(|lhs, rhs| {
            Spanned(
                Expr::Binary(
                    Binary::Geometric,
                    Box::new(lhs.clone()),
                    Box::new(rhs.clone()),
                ),
                lhs.1.start..rhs.1.end,
            )
        })
        .boxed();

    let binary = binary
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
        .foldl(|lhs, (op, rhs)| {
            Spanned(
                Expr::Binary(op, Box::new(lhs.clone()), Box::new(rhs.clone())),
                lhs.1.start..rhs.1.end,
            )
        })
        .boxed();

    let binary = binary
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
        .foldl(|lhs, (op, rhs)| {
            Spanned(
                Expr::Binary(op, Box::new(lhs.clone()), Box::new(rhs.clone())),
                lhs.1.start..rhs.1.end,
            )
        })
        .boxed();

    binary
}
