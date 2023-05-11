use crate::vm::{error::*, ConstSymbol, Number, VirtualMachine};

/// Simple test for pushing, popping and peeking from VM stack.
#[test]
fn push_pop_peek() -> LispResult<()> {
    let mut vm = VirtualMachine::new();
    let value1 = vm.make_number(Number::Integer(2))?;
    let value2 = vm.make_number(Number::Integer(3))?;

    vm.stack_push(value1.clone())?;
    vm.stack_push(value2.clone())?;
    assert_eq!(vm.stack_peek()?, value2.clone());
    assert_eq!(vm.stack_pop()?, value2);
    assert_eq!(vm.stack_peek()?, value1.clone());
    assert_eq!(vm.stack_pop()?, value1);
    assert!(vm.stack_peek().is_err());
    assert!(vm.stack_pop().is_err());

    Ok(())
}

/// Push values onto VM stack to its limits, then expect an overflow
/// on the next push.
#[test]
fn overflow() -> LispResult<()> {
    use crate::vm::LISP_STACK_SIZE;

    let mut vm = VirtualMachine::new();

    while vm.stack.last < LISP_STACK_SIZE {
        vm.stack_push(ConstSymbol::T)?;
    }

    assert!(vm.stack_push(ConstSymbol::NIL).is_err());
    assert!(vm.stack_push(ConstSymbol::T).is_err());
    assert!(vm.stack_push(ConstSymbol::DONE).is_err());

    Ok(())
}

/// Pop from empty VM stack, and also attempt to peek top of empty stack.
/// Both events should cause and underflow failure.
#[test]
fn underflow() -> LispResult<()> {
    let mut vm = VirtualMachine::new();

    assert!(vm.stack_pop().is_err());
    assert!(vm.stack_peek().is_err());

    Ok(())
}

/// Test stack unwinding: pop from stack until reaching a given marker which
/// should be the first occurrence of a typed pointer.
#[test]
fn unwind() -> LispResult<()> {
    let mut vm = VirtualMachine::new();

    // Marker will be the built-in literal DONE
    vm.stack_push(ConstSymbol::DONE)?;

    for _ in 0..3 {
        vm.stack_push(ConstSymbol::T)?;
    }

    vm.stack_push(ConstSymbol::DONE)?;

    for _ in 0..4 {
        vm.stack_push(ConstSymbol::NIL)?;
    }

    assert_eq!(vm.stack_unwind(ConstSymbol::DONE), 5); // 4 symbols + 1 marker
    assert_eq!(vm.stack_unwind(ConstSymbol::DONE), 4); // 3 symbols + 1 marker
    assert_eq!(vm.stack_unwind(ConstSymbol::DONE), 0); // stack is empty
    assert!(vm.stack_peek().is_err());
    assert!(vm.stack_pop().is_err());

    Ok(())
}
