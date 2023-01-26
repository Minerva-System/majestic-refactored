use crate::parser::expression::*;
use chumsky::prelude::*;

pub struct Combinators;

impl Combinators {
    // == NUMERIC PARSERS ==

    pub fn integer() -> impl Parser<char, Expr, Error = Simple<char>> {
        just('-')
            .or_not()
            .chain::<char, _, _>(text::digits(10))
            .padded()
            .collect::<String>()
            .map(Expr::make_integer)
            .labelled("integer")
    }

    pub fn float() -> impl Parser<char, Expr, Error = Simple<char>> {
        just('-')
            .or_not()
            .chain(text::digits(10))
            .chain::<char, _, _>(just('.').chain(text::digits(10)))
            .padded()
            .collect::<String>()
            .map(Expr::make_float)
            .labelled("float")
    }

    pub fn fraction() -> impl Parser<char, Expr, Error = Simple<char>> {
        just('-')
            .or_not()
            .chain(text::digits(10))
            .chain(just('/'))
            .chain::<char, _, _>(text::digits(10))
            .padded()
            .collect::<String>()
            .map(Expr::make_fraction)
            .labelled("fraction")
    }

    // meta-label to differentiate from complex
    pub fn real() -> impl Parser<char, Expr, Error = Simple<char>> {
        Self::float().or(Self::fraction()).or(Self::integer())
    }

    pub fn complex() -> impl Parser<char, Expr, Error = Simple<char>> {
        Self::real()
            .then_ignore(just('j').or(just('J')))
            .chain(Self::real())
            .padded()
            .map(Expr::make_complex)
            .labelled("complex")
    }

    pub fn number() -> impl Parser<char, Expr, Error = Simple<char>> {
        Self::complex().or(Self::real())
    }

    // == NON-NUMERIC ATOMIC PARSERS ==

    pub fn symbol() -> impl Parser<char, Expr, Error = Simple<char>> {
        let valid_first_char =
            |c: &char| !RESERVED.contains(c) && !c.is_numeric() && !c.is_whitespace();

        let valid_char = |c: &char| !RESERVED.contains(c) && !c.is_whitespace();

        filter(valid_first_char)
            .map(Some)
            .chain::<char, _, _>(filter(valid_char).repeated())
            .collect::<String>()
            .map(Expr::make_symbol)
            .labelled("symbol")
    }

    pub fn string() -> impl Parser<char, Expr, Error = Simple<char>> {
        just('"')
            .ignore_then(filter(|c| *c != '"').repeated())
            .then_ignore(just('"'))
            .collect::<String>()
            .map(Expr::make_string)
            .labelled("string")
    }

    // == SINGLE-EXPRESSION PARSERS ==

    pub fn atom() -> impl Parser<char, Expr, Error = Simple<char>> {
        Self::number().or(Self::string()).or(Self::symbol())
    }

    pub fn comment() -> impl Parser<char, Expr, Error = Simple<char>> {
        just(";")
            .ignore_then(take_until(just('\n')))
            .map(|pair| {
                let value: String = pair.0.iter().cloned().collect::<String>();
                Expr::Comment(value)
            })
            .padded()
            .labelled("comment")
    }

    // == COMBINED EXPRESSIONS & THE EXPRESSION PARSER ==

    pub fn expression() -> impl Parser<char, Expr, Error = Simple<char>> {
        recursive(|expression: Recursive<'_, char, Expr, Simple<char>>| {
            let list = expression
                .clone()
                .padded()
                .repeated()
                .delimited_by(just('('), just(')'))
                .collect::<Vec<Expr>>()
                .map(Expr::List)
                .labelled("list");

            let cons = expression
                .clone()
                .padded()
                .then_ignore(just('.').padded())
                .chain::<Expr, _, _>(expression.clone())
                .delimited_by(just('('), just(')'))
                .collect::<Vec<Expr>>()
                .map(Expr::make_cons)
                .labelled("cons");

            let vector = expression
                .clone()
                .padded()
                .repeated()
                .delimited_by(just('['), just(']'))
                .collect::<Vec<Expr>>()
                .map(Expr::Vector)
                .labelled("vector");

            let quoted = just('\'')
                .ignore_then(expression.clone())
                .map(|e| Expr::Prefixed(PrefixType::Quote, Box::new(e.clone())))
                .labelled("quoted expression");

            let quasiquoted = just('`')
                .ignore_then(expression.clone())
                .map(|e| Expr::Prefixed(PrefixType::Quasiquote, Box::new(e.clone())))
                .labelled("quasiquoted expression");

            let unquoted_list = just(",@")
                .ignore_then(expression.clone())
                .map(|e| Expr::Prefixed(PrefixType::UnquoteList, Box::new(e.clone())))
                .labelled("unquoted expression");

            let unquoted = just(',')
                .ignore_then(expression.clone())
                .map(|e| Expr::Prefixed(PrefixType::Unquote, Box::new(e.clone())))
                .labelled("unquoted expression");

            Self::comment()
                .or(quoted)
                .or(quasiquoted)
                .or(unquoted_list)
                .or(unquoted)
                .or(Self::atom())
                .or(cons)
                .or(list)
                .or(vector)
                .padded()
        })
    }

    // == ACTUAL PARSER ==

    pub fn parser() -> impl Parser<char, Vec<Expr>, Error = Simple<char>> {
        Self::expression().repeated().then_ignore(end())
    }
}
