pub const RESERVED: &[char] = &['(', ')', '[', ']', '\'', '`', ',', '"', '@', '.'];

#[derive(Debug, Clone, PartialEq)]
pub enum NumberExpr {
    Integer(i64),
    Float(f64),
    Fraction(i64, i64),
    Complex(Box<NumberExpr>, Box<NumberExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AtomExpr {
    Number(NumberExpr),
    String(String),
    Symbol(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixType {
    Quote,
    Quasiquote,
    Unquote,
    UnquoteSplice,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(AtomExpr),
    Prefixed(PrefixType, Box<Expr>),
    List(Vec<Expr>),
    DottedList(Vec<Expr>),
    Vector(Vec<Expr>),
    Cons(Box<Expr>, Box<Expr>),
    Comment(String),
}

impl Expr {
    pub fn make_integer(s: String) -> Expr {
        Expr::Atom(AtomExpr::Number(NumberExpr::Integer(s.parse().unwrap())))
    }

    pub fn make_float(s: String) -> Expr {
        Expr::Atom(AtomExpr::Number(NumberExpr::Float(s.parse().unwrap())))
    }

    pub fn make_fraction(s: String) -> Expr {
        let separator = s.find('/').unwrap();
        let numerator = &s[0..separator];
        let denominator = &s[separator + 1..];

        Expr::Atom(AtomExpr::Number(NumberExpr::Fraction(
            numerator.parse().unwrap(),
            denominator.parse().unwrap(),
        )))
    }

    pub fn make_complex(v: Vec<Expr>) -> Expr {
        let first = v.get(0).unwrap();
        let second = v.get(1).unwrap();

        if let (Expr::Atom(AtomExpr::Number(real)), Expr::Atom(AtomExpr::Number(imag))) =
            (first, second)
        {
            Expr::Atom(AtomExpr::Number(NumberExpr::Complex(
                Box::new(real.clone()),
                Box::new(imag.clone()),
            )))
        } else {
            panic!("Could not recover complex number parts");
        }
    }

    pub fn make_symbol(s: String) -> Expr {
        Expr::Atom(AtomExpr::Symbol(s))
    }

    pub fn make_string(s: String) -> Expr {
        Expr::Atom(AtomExpr::String(s))
    }

    pub fn make_cons(v: Vec<Expr>) -> Expr {
        Expr::Cons(
            Box::new(v.get(0).unwrap().clone()),
            Box::new(v.get(1).unwrap().clone()),
        )
    }
}
