//! Utils for building integration tests.

use majestic::parser::expression;

/// Helper function to produce an expression from a fixed expression in text form.
/// Panics when the expression contains syntax errors, and returns only the first
/// parsed expression.
pub fn get_expression(text: &str) -> expression::Expr {
    use chumsky::Parser;
    use majestic::parser::combinators;

    let (parsed, errs) = combinators::Combinators::parser().parse_recovery(text);
    if !errs.is_empty() {
        panic!("Given expression contains syntax errors");
    }
    parsed
        .expect("Parsed expression")
        .first()
        .expect("First expression")
        .to_owned()
}

/// Helper macro for testing the output of an AST. Results should be equal to
/// the expected output. When an expected output is not given, it is assumed
/// that the results should equal the input text.
#[macro_export]
macro_rules! generate_ast_test {
    ($vm:ident, $text:expr) => {
        generate_ast_test!($vm, $text, $text);
    };

    ($vm:ident, $text:expr, $expected:expr) => {{
        use majestic::{parser::convert, printer};
        // Get parsed expression as a Rust-like syntax tree
        let expr = get_expression($text);

        // Try building a syntax tree withing the virtual machine
        let pointer = convert::build_ast(&mut $vm, expr)
            .expect("Typed pointer to AST within virtual machine");

        // Check whether the printed expression is equal to the input text
        assert_eq!($expected, printer::format_object(&$vm, &pointer));
    }};
}

/// Helper macro for testing the output of formatting an object (referenced by
/// its typed pointer) against a regular expression.
#[macro_export]
macro_rules! format_obj_like {
    ($vm:ident, $typedptr:expr, $regex:expr) => {{
        use majestic::printer;
        use regex::Regex;

        // Build regex engine from regex text
        let re = Regex::new($regex).expect("Regex engine built");

        // Check whether the printed expression matches the regular expression
        assert!(re.is_match(&printer::format_object(&$vm, &$typedptr)));
    }};
}

/// Helper macro for testing the output of an AST against a regular expression.
/// Results should match the given regular expression.
#[macro_export]
macro_rules! generate_ast_like {
    ($vm:ident, $text:expr, $regex:expr) => {{
        use majestic::parser::convert;

        // Get parsed expression as a Rust-like syntax tree
        let expr = get_expression($text);

        // Try building a syntax tree withing the virtual machine
        let pointer = convert::build_ast(&mut $vm, expr)
            .expect("Typed pointer to AST within virtual machine");

        format_obj_like!($vm, pointer, $regex);
    }};
}

/// Unit struct holding common regular expressions.
pub struct RegularExpression;

#[allow(dead_code)]
impl RegularExpression {
    /// Regular expression for environment textual format.
    /// ### Examples
    /// - `#<ENV0>`
    /// - `#<ENV3>`
    /// - `#<ENV59>`
    pub const ENVIRONMENT: &str = r"^(?u)#<ENV[0-9]*>$";

    /// Regular expression for function textual format.
    /// ### Example
    /// `#<FUNCTION {0xdeadbeef}>`
    pub const FUNCTION: &str = r"(?u)^#<FUNCTION \{0x[0-9a-z]*\}>$";

    /// Regular expression for literal textual format.
    /// ### Example
    /// `#<LITERAL {0xdeadbeef}>`
    pub const LITERAL: &str = r"(?u)^#<LITERAL \{0x[0-9a-z]*\}>$";

    /// Regular expression for built-in function textual format.
    /// ### Example
    /// `#<BUILTIN-FUNCTION {0xdeadbeef}>`
    pub const BUILTIN_FUNCTION: &str = r"(?u)^#<BUILTIN-FUNCTION \{0x[0-9a-z]*\}>$";

    /// Regular expression for built-in literal textual format.
    /// ### Example
    /// `#<BUILTIN-LITERAL {0xdeadbeef}>`
    pub const BUILTIN_LITERAL: &str = r"(?u)^#<BUILTIN-LITERAL \{0x[0-9a-z]*\}>$";

    /// Regular expression for typed pointers. Remember to trim the string
    /// before comparing.
    /// ### Example
    /// `CONS::0xdeadbeef`
    pub const TYPED_POINTER: &str = r"(?u)^[A-Z]*::0x[0-9a-z]*$";
}
