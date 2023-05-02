//! Test module for building proper ASTs on a VM, by leveraging an AST built by
//! the parser.

use majestic::vm;

use crate::util::*;
use crate::*;

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
    generate_ast_test!(vm, "()", "nil");
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
