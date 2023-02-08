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

#[test]
fn parse_real_numbers() {
    let parser = Combinators::real().then_ignore(end());
    let helper = |n| Ok(Expr::Atom(AtomExpr::Number(n)));

    assert_eq!(helper(NumberExpr::Integer(200)), parser.parse("200"));
    assert_eq!(helper(NumberExpr::Float(3.14)), parser.parse("3.14"));
    assert_eq!(helper(NumberExpr::Fraction(2, 3)), parser.parse("2/3"));

    assert!(parser.parse("5e").is_err());
    assert!(parser.parse("e6").is_err());
    assert!(parser.parse("72     *").is_err());
    assert!(parser.parse("0.5.6").is_err());
    assert!(parser.parse(".5").is_err());
    assert!(parser.parse("-.5").is_err());
    assert!(parser.parse("2.0/3.0").is_err());
    assert!(parser.parse("5j1/3").is_err());
}

// complex numbers
#[test]
fn parse_complex_numbers() {
    let parser = Combinators::complex().then_ignore(end());
    let helper = |r, i| {
        Ok(Expr::Atom(AtomExpr::Number(NumberExpr::Complex(
            Box::new(r),
            Box::new(i),
        ))))
    };

    assert_eq!(
        helper(NumberExpr::Integer(2), NumberExpr::Integer(3)),
        parser.parse("2j3")
    );
    assert_eq!(
        helper(NumberExpr::Integer(2), NumberExpr::Integer(3)),
        parser.parse("2J3")
    );
    assert_eq!(
        helper(NumberExpr::Float(3.5), NumberExpr::Integer(9)),
        parser.parse("3.5j9")
    );
    assert_eq!(
        helper(NumberExpr::Float(3.5), NumberExpr::Integer(9)),
        parser.parse("3.5J9")
    );

    assert!(parser.parse("j3").is_err());
    assert!(parser.parse("5j").is_err());
    assert!(parser.parse("2jj9").is_err());
    assert!(parser.parse("5j4j3").is_err());
}

// numbers (light test)
#[test]
fn parse_numbers() {
    let parser = Combinators::number().then_ignore(end());

    let helper = |v| Ok(Expr::Atom(AtomExpr::Number(v)));

    assert_eq!(helper(NumberExpr::Integer(2)), parser.parse("2"));
    assert_eq!(helper(NumberExpr::Float(3.14)), parser.parse("3.14"));
    assert_eq!(helper(NumberExpr::Fraction(5, 3)), parser.parse("5/3"));
    assert_eq!(
        helper(NumberExpr::Complex(
            Box::new(NumberExpr::Integer(10)),
            Box::new(NumberExpr::Fraction(6, 29))
        )),
        parser.parse("10j6/29")
    );
}

// symbols
#[test]
fn parse_symbols() {
    let parser = Combinators::symbol().then_ignore(end());

    let helper = |v: &str| Ok(Expr::Atom(AtomExpr::Symbol(v.to_owned())));

    assert_eq!(helper("foo"), parser.parse("foo"));
    assert_eq!(helper("*test*"), parser.parse("*test*"));
    assert_eq!(helper("+bar+"), parser.parse("+bar+"));
    assert_eq!(helper("setq"), parser.parse("setq"));
    assert_eq!(helper("baz123"), parser.parse("baz123"));

    assert!(parser.parse("123foo").is_err());
}

// strings
#[test]
fn parse_string() {
    let parser = Combinators::string().then_ignore(end());

    let helper = |v: &str| Ok(Expr::Atom(AtomExpr::String(v.to_owned())));

    assert_eq!(helper("hello"), parser.parse("\"hello\""));
    assert_eq!(
        helper(&format!("hello,\nworld")),
        parser.parse(format!("\"hello,\nworld\""))
    );

    assert!(parser.parse("\"hello my friend").is_err());
    assert!(parser.parse("hello my friend\"").is_err());
}

// atoms
#[test]
fn parse_atom() {
    let parser = Combinators::atom().then_ignore(end());

    let number_helper = |v| Ok(Expr::Atom(AtomExpr::Number(v)));
    let symbol_helper = |v: &str| Ok(Expr::Atom(AtomExpr::Symbol(v.to_owned())));
    let string_helper = |v: &str| Ok(Expr::Atom(AtomExpr::String(v.to_owned())));

    assert_eq!(number_helper(NumberExpr::Integer(0)), parser.parse("0"));
    assert_eq!(number_helper(NumberExpr::Float(2.66)), parser.parse("2.66"));
    assert_eq!(
        number_helper(NumberExpr::Fraction(9, 12)),
        parser.parse("9/12")
    );
    assert_eq!(
        number_helper(NumberExpr::Complex(
            Box::new(NumberExpr::Float(5.98)),
            Box::new(NumberExpr::Float(27.9))
        )),
        parser.parse("5.98J27.9")
    );
    assert_eq!(symbol_helper("bah"), parser.parse("bah"));
    assert_eq!(symbol_helper("*blah*"), parser.parse("*blah*"));
    assert_eq!(symbol_helper("+test+"), parser.parse("+test+"));
    assert_eq!(symbol_helper("eval"), parser.parse("eval"));
    assert_eq!(symbol_helper("lalala456"), parser.parse("lalala456"));
    assert_eq!(string_helper("how"), parser.parse("\"how\""));
    assert_eq!(
        string_helper(&format!("how\nare you?")),
        parser.parse(format!("\"how\nare you?\""))
    );

    assert!(parser.parse("5e").is_err());
    assert!(parser.parse("123foo").is_err());
    assert!(parser.parse("0.5.6").is_err());
    assert!(parser.parse(".5").is_err());
    assert!(parser.parse("-.5").is_err());
    assert!(parser.parse("2.0/3.0").is_err());
    assert!(parser.parse("5j").is_err());
    assert!(parser.parse("2jj9").is_err());
    assert!(parser.parse("5j4j3").is_err());
    assert!(parser.parse("\"hello my friend").is_err());
    assert!(parser.parse("hello my friend\"").is_err());
}

// // comments
// #[test]
// fn parse_comment() {
//     unimplemented!();
// }

// // expressions -- lists
// #[test]
// fn parse_expression_list() {
//     unimplemented!();
// }

// // expressions -- dotted lists
// #[test]
// fn parse_expression_dotted_list() {
//     unimplemented!();
// }

// // expressions -- cons
// #[test]
// fn parse_expression_cons() {
//     unimplemented!();
// }

// // expressions -- vectors
// #[test]
// fn parse_expression_vector() {
//     unimplemented!();
// }

// // expressions -- quote, quasiquote, unquote
// #[test]
// fn parse_expression_quote() {
//     unimplemented!();
// }

// #[test]
// fn parse_expression_quasiquote() {
//     unimplemented!();
// }

// #[test]
// fn parse_expression_unquote() {
//     unimplemented!();
// }
