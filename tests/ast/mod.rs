//! Test module for building proper ASTs on a VM, by leveraging an AST built by
//! the parser.

use majestic::{
    parser::{combinators, convert, expression},
    printer, vm,
};

/// Helper function to produce an expression from a fixed expression in text form.
/// Panics when the expression contains syntax errors, and returns only the first
/// parsed expression.
fn get_expression(text: &str) -> expression::Expr {
    use chumsky::Parser;

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

macro_rules! generate_ast_test {
    ($vm:ident, $text:expr) => {
        generate_ast_test!($vm, $text, $text);
    };

    ($vm:ident, $text:expr, $expected:expr) => {{
        // Get parsed expression as a Rust-like syntax tree
        let expr = get_expression($text);

        // Try building a syntax tree withing the virtual machine
        let pointer = convert::build_ast(&mut $vm, expr)
            .expect("Typed pointer to AST within virtual machine");

        // Check whether the printed expression is equal to the input text
        assert_eq!($expected, printer::format_object(&$vm, &pointer));
    }};
}

// atom
#[test]
fn convert_ast_atom() {
    let mut vm = vm::VirtualMachine::new();
    generate_ast_test!(vm, "t");
    generate_ast_test!(vm, "nil");
}

// list
#[test]
fn convert_ast_list() {
    let mut vm = vm::VirtualMachine::new();
    generate_ast_test!(vm, "(1 2 3)");
}

// dotted list
#[test]
fn convert_ast_dotted_list() {
    let mut vm = vm::VirtualMachine::new();
    generate_ast_test!(vm, "(1 2 3 . 4)");
}

// vector -- NOTE: VECTOR STORAGE IS NOT YET IMPLEMENTED
#[ignore]
#[test]
fn convert_ast_vector() {
    let mut vm = vm::VirtualMachine::new();
    generate_ast_test!(vm, "[1 2 3 4]");
}

// cons
#[test]
fn convert_ast_cons() {
    let mut vm = vm::VirtualMachine::new();
    generate_ast_test!(vm, "(5 . 4)");
    generate_ast_test!(vm, "(1 . (2 . (3 . 4)))", "(1 2 3 . 4)");
    generate_ast_test!(vm, "(1 . (2 . nil))", "(1 2)");
}

// prefixed
#[test]
fn convert_ast_prefixed() {
    let mut vm = vm::VirtualMachine::new();
    generate_ast_test!(vm, "(quote foo)");
    generate_ast_test!(vm, "(quasiquote foo)");
    generate_ast_test!(vm, "(quasiquote (foo (unquote bar)))");
    generate_ast_test!(vm, "(quasiquote (foo (unquote-splice bar)))");

    generate_ast_test!(vm, "'foo", "(quote foo)");
    generate_ast_test!(vm, "`foo", "(quasiquote foo)");
    generate_ast_test!(vm, "`(foo ,bar)", "(quasiquote (foo (unquote bar)))");
    generate_ast_test!(
        vm,
        "`(foo ,@bar)",
        "(quasiquote (foo (unquote-splice bar)))"
    );
}
