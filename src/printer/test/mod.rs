use crate::vm::types::{TypedPointer, VirtualMachine};

fn util_build_object(vm: &mut VirtualMachine, expr: &str) -> TypedPointer {
    use chumsky::prelude::*;

    let expr = crate::parser::combinators::Combinators::parser()
        .parse(expr)
        .unwrap();
    crate::parser::convert::build_ast(vm, expr.first().unwrap().clone()).unwrap()
}

fn gen_format(vm: &mut VirtualMachine, expr: &str) -> String {
    let obj = util_build_object(vm, expr);
    super::format_object(vm, &obj)
}

#[test]
fn format_numbers() {
    let mut vm = VirtualMachine::default();
    assert_eq!(gen_format(&mut vm, "1"), "1");
    assert_eq!(gen_format(&mut vm, "3.14"), "3.14");
    assert_eq!(gen_format(&mut vm, "2/5"), "2/5");
    assert_eq!(gen_format(&mut vm, "3j4"), "3J4");
}

#[test]
fn format_atoms() {
    let mut vm = VirtualMachine::default();
    assert_eq!(gen_format(&mut vm, "foo"), "foo");
    assert_eq!(gen_format(&mut vm, "foo-bar"), "foo-bar");
    assert_eq!(gen_format(&mut vm, "*baz*"), "*baz*");
}

// functions
// literals
// built-in functions
// built-in literals
// environments
// cons'es, lists and dotted lists
