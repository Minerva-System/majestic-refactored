//! Test module for testing the printer and formatting functions of the VM.

use majestic::{printer, vm};

use crate::util::*;
use crate::*;

macro_rules! generate_test_builtin {
    ($vm:ident, $regex:expr, {$symbol:expr}) => {{
        format_obj_like!($vm, $symbol, $regex);
    }};

    ($vm:ident, $regex:expr, {$symbol:expr, $($rest:expr),+}) => {{
	generate_test_builtin!($vm, $regex, {$symbol});
	generate_test_builtin!($vm, $regex, {$($rest),+});
    }};
}

macro_rules! generate_test_obj_like {
    ($vm:ident, $regex:expr, {$symbol:expr}) => {{
	use majestic::parser::convert;

	let expr = get_expression($symbol);

	let ast_pointer = convert::build_ast(&mut $vm, expr)
            .expect("Typed pointer to AST within virtual machine");

	let pointer = $vm.evaluate(ast_pointer).expect("Evaluated expression");

        format_obj_like!($vm, pointer, $regex);
    }};

    ($vm:ident, $regex:expr, {$symbol:expr, $($rest:expr),+}) => {{
	generate_test_obj_like!($vm, $regex, {$symbol});
	generate_test_obj_like!($vm, $regex, {$($rest),+});
    }};
}

// undefined
#[test]
fn format_undefined() {
    let vm = vm::VirtualMachine::new();
    let undefined = vm::types::TypedPointer::default();
    assert_eq!("undefined", printer::format_object(&vm, &undefined));
}

// number
#[test]
fn format_number() {
    let mut vm = vm::VirtualMachine::new();
    generate_ast_test!(vm, "-10");
    generate_ast_test!(vm, "2");
    generate_ast_test!(vm, "3.14");
    generate_ast_test!(vm, "-2.999");
    generate_ast_test!(vm, "3/5");
    generate_ast_test!(vm, "-7/9");
    generate_ast_test!(vm, "2J5");
    generate_ast_test!(vm, "2j5", "2J5");
    generate_ast_test!(vm, "-10J4");
    generate_ast_test!(vm, "-10j4", "-10J4");
    generate_ast_test!(vm, "2.66J5/6");
    generate_ast_test!(vm, "2.66j5/6", "2.66J5/6");
    generate_ast_test!(vm, "0.32J9.23");
    generate_ast_test!(vm, "0.32j9.23", "0.32J9.23");
}

// atom
#[test]
fn format_atom() {
    let mut vm = vm::VirtualMachine::new();

    let num_registered_atoms = vm.atoms.last;

    generate_ast_test!(vm, "nil");
    generate_ast_test!(vm, "t");
    generate_ast_test!(vm, "prim");
    generate_ast_test!(vm, "lit");
    generate_ast_test!(vm, "closure");
    generate_ast_test!(vm, "error");
    generate_ast_test!(vm, "fn");
    generate_ast_test!(vm, "&");
    generate_ast_test!(vm, "apply");
    generate_ast_test!(vm, "macro");
    generate_ast_test!(vm, "mac");
    generate_ast_test!(vm, "quote");
    generate_ast_test!(vm, "unquote");
    generate_ast_test!(vm, "unquote-splice");
    generate_ast_test!(vm, "quasiquote");
    generate_ast_test!(vm, "do");
    generate_ast_test!(vm, "integer");
    generate_ast_test!(vm, "float");
    generate_ast_test!(vm, "fraction");
    generate_ast_test!(vm, "complex");
    generate_ast_test!(vm, "vector");
    generate_ast_test!(vm, "setq");

    assert_eq!(
        vm.atoms.last, num_registered_atoms,
        "atom table shouldn't grow"
    );

    generate_ast_test!(vm, "foo");
    generate_ast_test!(vm, "*bar*");
    generate_ast_test!(vm, "+baz+");
    generate_ast_test!(vm, "+");
    generate_ast_test!(vm, "-");
    generate_ast_test!(vm, "*");
    generate_ast_test!(vm, "/");
    generate_ast_test!(vm, "α");
    generate_ast_test!(vm, "β");
    generate_ast_test!(vm, "γ");
    generate_ast_test!(vm, "+/÷≢");
    generate_ast_test!(vm, "×∘⍳¨");
    generate_ast_test!(vm, "عالم");
    generate_ast_test!(vm, "世界");

    assert!(
        vm.atoms.last != num_registered_atoms,
        "atom table should grow"
    );
}

// function
#[test]
fn format_function() {
    use crate::util::RegularExpression;

    let mut vm = vm::VirtualMachine::new();

    generate_test_obj_like!(vm, RegularExpression::FUNCTION, {
    "(fn (x) (* x x))",
    "(fn (x y) (+ x y))",
    "(fn () (cons 1 2))"
    });
}

// literal
#[ignore]
#[test]
fn format_literal() {
    // TODO: Implement when we finally have a way to create literals
    unimplemented!();
}

// built-in function
#[test]
fn format_builtin_function() {
    use crate::util::RegularExpression;
    use vm::constants::ConstSymbol;

    let vm = vm::VirtualMachine::new();

    generate_test_builtin!(vm, RegularExpression::BUILTIN_FUNCTION, {
        ConstSymbol::BIN_CONS,
        ConstSymbol::BIN_LIST,
        ConstSymbol::BIN_CAR,
        ConstSymbol::BIN_CDR,
        ConstSymbol::BIN_EVAL,
        ConstSymbol::BIN_EQ
    });
}

// built-in literal
#[test]
fn format_builtin_literal() {
    use crate::util::RegularExpression;
    use vm::constants::ConstSymbol;

    let vm = vm::VirtualMachine::new();

    generate_test_builtin!(vm, RegularExpression::BUILTIN_LITERAL, {
        ConstSymbol::DONE,
        ConstSymbol::EVAL_ARGS,
        ConstSymbol::ACCUMULATE_ARG,
        ConstSymbol::ACCUMULATE_LAST_ARG,
        ConstSymbol::EVAL_ASSIGN
    });
}

// environment
#[test]
fn format_environment() {
    use crate::util::RegularExpression;
    use vm::constants::ConstSymbol;

    let vm = vm::VirtualMachine::new();

    generate_test_builtin!(vm, RegularExpression::ENVIRONMENT, { ConstSymbol::E0 });
}

// cons
#[test]
fn format_cons() {
    let mut vm = vm::VirtualMachine::new();
    generate_ast_test!(vm, "(1 . 2)");
    generate_ast_test!(vm, "(a . b)");
    generate_ast_test!(vm, "(1 . b)");
    generate_ast_test!(vm, "(a . 2)");
    generate_ast_test!(vm, "(foo . bar)");
    generate_ast_test!(vm, "(baz . quux)");
    generate_ast_test!(
        vm,
        "((foo . bar) . (baz . quux))",
        "((foo . bar) baz . quux)"
    );
}

// list
#[test]
fn format_list() {
    let mut vm = vm::VirtualMachine::new();
    generate_ast_test!(vm, "(1 2)");
    generate_ast_test!(vm, "(1 2 3)");
    generate_ast_test!(vm, "(setq x 5)");
    generate_ast_test!(vm, "(fn (x) (* x x))");
    generate_ast_test!(vm, "(() () ())", "(nil nil nil)");
}

// dotted list
#[test]
fn format_dotted_list() {
    let mut vm = vm::VirtualMachine::new();
    generate_ast_test!(vm, "(1 . 2)");
    generate_ast_test!(vm, "(1 2 . 3)");
    generate_ast_test!(vm, "(foo x . 5)");
    generate_ast_test!(vm, "(baz (x) . (* x . x))", "(baz (x) * x . x)");
    generate_ast_test!(vm, "(() () . ())", "(nil nil)");
}
