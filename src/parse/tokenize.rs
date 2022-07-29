use chumsky::prelude::*;

use crate::interpret::expr::Basis;

#[derive(Debug, Clone)]
pub enum Token {
    Whitespace,
    Number(String),
    Basis(Basis),
    Identifier(String),
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    Operator,
}

pub fn tokenize(input: &str, errors: &mut Vec<Simple<char>>) -> Option<Vec<Token>> {
    match tokenizer().parse(input) {
        Ok(tokens) => Some(tokens),
        Err(mut tokenizer_errors) => {
            errors.append(&mut tokenizer_errors);
            None
        }
    }
}

fn tokenizer<'a>() -> impl Parser<char, Vec<Token>, Error = Simple<char>> + Clone + 'a {
    let whitespace: BoxedParser<char, Token, Simple<char>> = filter(|&c: &char| c.is_whitespace())
        .repeated()
        .at_least(1)
        .to(Token::Whitespace)
        .boxed();

    let operator: BoxedParser<char, Token, Simple<char>> = one_of(r"+-*/\&<>%|")
        .repeated()
        .at_least(1)
        .to(Token::Operator)
        .boxed();

    let number: BoxedParser<char, Token, Simple<char>> = text::int(10)
        .then(just('.').ignore_then(text::int(10)).or_not())
        .map(|(mut int, frac)| {
            Token::Number(match frac {
                Some(frac) => format!("{int}.{frac}"),
                None => int,
            })
        })
        .boxed();

    let basis: BoxedParser<char, Token, Simple<char>> = just('i')
        .to(Basis::Pseudoscalar)
        .or(just('e')
            .ignore_then(
                filter_map(|span, c: char| match c.to_digit(10) {
                    Some(x) => Ok(x as usize),
                    None => Err(Simple::custom(span, format!("'{}' is not a digit", c))),
                })
                .repeated()
                .at_least(1),
            )
            .map(Basis::Vectors))
        .map(Token::Basis)
        .boxed();

    let identifier: BoxedParser<char, Token, Simple<char>> =
        filter(|&c| unicode_ident::is_xid_start(c))
            .map(|c| String::from(c))
            .then(filter(|&c| unicode_ident::is_xid_continue(c)).repeated())
            .foldl(|mut s, c| {
                s.push(c);
                s
            })
            .or(just("⊥".to_string()).or(just("_|_".to_string())))
            .map(Token::Identifier)
            .boxed();

    let delimiter: BoxedParser<char, Token, Simple<char>> = select! {
        '(' => Token::ParenOpen,
        ')' => Token::ParenClose,
        '[' => Token::BracketOpen,
        ']' => Token::BracketClose,
    }
    .boxed();

    choice((whitespace, operator, number, basis, identifier, delimiter))
        .repeated()
        .then_ignore(end())
}
