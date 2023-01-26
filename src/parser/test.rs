use crate::parser::combinators::Combinators;
use crate::parser::expression::*;
use chumsky::prelude::*;

#[test]
fn parse_integer() {
    let parser = Combinators::integer().then_ignore(end());
    let helper = |i: i64| Ok(Expr::Atom(AtomExpr::Number(NumberExpr::Integer(i))));

    assert_eq!(helper(5), parser.parse("5"));
    assert_eq!(helper(-2), parser.parse("-2"));
    assert_eq!(helper(0), parser.parse("0"));
    assert_eq!(helper(99), parser.parse("   99"));

    assert!(parser.parse("5e").is_err());
    assert!(parser.parse("e6").is_err());
    assert!(parser.parse("72     *").is_err());
    // TODO
}

#[test]
fn parse_float() {
    use float_cmp::approx_eq;

    let parser = Combinators::float().then_ignore(end());
    //let helper = |i: f64| Ok(Expr::Atom(AtomExpr::Number(NumberExpr::Float(i))));

    let test_parser = |num, text| {
        let parsed = parser.parse(text);
        let parsed_extract = if let Ok(Expr::Atom(AtomExpr::Number(NumberExpr::Float(i)))) = parsed
        {
            i
        } else {
            return false;
        };

        approx_eq!(f64, parsed_extract, num, ulps = 2)
    };

    assert!(test_parser(1.0, "1.0"));
    assert!(test_parser(0.0, "0.0"));
    assert!(test_parser(-10.56, "-10.56"));

    assert!(parser.parse("0.5.6").is_err());
    assert!(parser.parse(".5").is_err());
    assert!(parser.parse("-.5").is_err());
    // TODO
}

#[test]
fn parse_fraction() {
    let parser = Combinators::fraction().then_ignore(end());
    let helper = |n, d| Ok(Expr::Atom(AtomExpr::Number(NumberExpr::Fraction(n, d))));

    assert_eq!(helper(2, 3), parser.parse("2/3"));
    assert_eq!(helper(3, 4), parser.parse("3/4"));
    assert_eq!(helper(5, 8), parser.parse("5/8"));
    assert_eq!(helper(4, 6), parser.parse("4/6"));

    assert!(parser.parse("2.0/3.0").is_err());
    assert!(parser.parse("5j1/3").is_err());
    // TODO
}

// real numbers (light test)

// complex numbers

// numbers (light test)

// symbols

// strings

// atoms

// comments

// expressions -- lists

// expressions -- dotted lists (todo)

// expressions -- cons

// expressions -- vectors

// expressions -- quote, quasiquote, unquote
