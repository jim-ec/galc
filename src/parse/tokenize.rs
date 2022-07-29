use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Whitespace,
    Number(String),
    Basis(Vec<usize>),
    Identifier(String),
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    Subtraction,
    Dual,
    Reverse,
    ExteriorProduct,
    RegressiveProduct,
    LeftContraction,
    RightContraction,
    InnerProduct,
    ScalarProduct,
    Division,
    Power,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Vec<Simple<char>>> {
    tokenizer().parse(input)
}

fn tokenizer<'a>() -> impl Parser<char, Vec<Token>, Error = Simple<char>> + Clone + 'a {
    let whitespace: BoxedParser<char, Token, Simple<char>> = filter(|&c: &char| c.is_whitespace())
        .repeated()
        .at_least(1)
        .to(Token::Whitespace)
        .boxed();

    let operator: BoxedParser<char, Token, Simple<char>> = choice((
        just(r"-").to(Token::Subtraction),
        just(r"!").to(Token::Dual),
        just(r"~").to(Token::Reverse),
        just(r"/\").to(Token::ExteriorProduct),
        just(r"\/").to(Token::RegressiveProduct),
        just(r">>").to(Token::LeftContraction),
        just(r"<<").to(Token::RightContraction),
        just(r"|").to(Token::InnerProduct),
        just(r"*").to(Token::ScalarProduct),
        just(r"/").to(Token::Division),
        just(r"^").to(Token::Power),
    ))
    .boxed();

    let number: BoxedParser<char, Token, Simple<char>> = text::int(10)
        .then(just('.').ignore_then(text::int(10)).or_not())
        .map(|(int, frac)| {
            Token::Number(match frac {
                Some(frac) => format!("{int}.{frac}"),
                None => int,
            })
        })
        .boxed();

    let basis: BoxedParser<char, Token, Simple<char>> = just('e')
        .ignore_then(
            filter_map(|span, c: char| match c.to_digit(10) {
                Some(x) => Ok(x as usize),
                None => Err(Simple::custom(span, format!("'{}' is not a digit", c))),
            })
            .repeated()
            .at_least(1),
        )
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
