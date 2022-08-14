use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Whitespace,
    Number(String),
    Basis(Vec<usize>),
    Identifier(String),
    Bottom,
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    Plus,
    Minus,
    Tilde,
    Wedge,
    AntiWedge,
    LeftContraction,
    RightContraction,
    InnerProduct,
    Asteriks,
    Solidus,
    Hat,
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
        just(r"+").to(Token::Plus),
        just(r"-").to(Token::Minus),
        just(r"~").to(Token::Tilde),
        just(r"/\").to(Token::Wedge),
        just(r"\/").to(Token::AntiWedge),
        just(r"-|").to(Token::LeftContraction),
        just(r"|-").to(Token::RightContraction),
        just(r"|").to(Token::InnerProduct),
        just(r"*").to(Token::Asteriks),
        just(r"/").to(Token::Solidus),
        just(r"^").to(Token::Hat),
    ))
    .boxed();

    let bottom: BoxedParser<char, Token, Simple<char>> = just(r"_|_").to(Token::Bottom).boxed();

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

    choice((
        whitespace, bottom, operator, number, basis, identifier, delimiter,
    ))
    .repeated()
    .then_ignore(end())
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Whitespace => write!(f, "whitespace"),
            Token::Number(_) => write!(f, "number"),
            Token::Basis(_) => write!(f, "basis"),
            Token::Identifier(_) => write!(f, "identifier"),
            Token::Bottom => write!(f, "_|_"),
            Token::ParenOpen => write!(f, "("),
            Token::ParenClose => write!(f, ")"),
            Token::BracketOpen => write!(f, "["),
            Token::BracketClose => write!(f, "]"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Tilde => write!(f, "~"),
            Token::Wedge => write!(f, "/\\"),
            Token::AntiWedge => write!(f, "\\/"),
            Token::LeftContraction => write!(f, "-|"),
            Token::RightContraction => write!(f, "|-"),
            Token::InnerProduct => write!(f, "|"),
            Token::Asteriks => write!(f, "*"),
            Token::Solidus => write!(f, "/"),
            Token::Hat => write!(f, "^"),
        }
    }
}
