use crate::vm::constants::ConstSymbol;
use crate::vm::error::*;
use crate::vm::types::{Number, VirtualMachine};

#[test]
fn create_assign_atom() -> LispResult<()> {
    let mut vm = VirtualMachine::new();

    // create atom
    let atom_ptr = vm.make_atom("my-atom")?;

    // create number
    let number_ptr = vm.make_number(Number::Integer(10))?;

    // assign a value
    vm.assign_value(atom_ptr.clone(), number_ptr.clone())?;

    // lookup value assigned in atom table
    let atom = vm
        .atoms
        .area
        .get(atom_ptr.value)
        .ok_or(())
        .map_err(|_| LispError::internal("Could not find atom"))?;

    assert_eq!(atom.value, number_ptr);
    assert_eq!(atom.name, "my-atom");

    Ok(())
}

#[test]
fn self_eval_atoms() -> LispResult<()> {
    let mut vm = VirtualMachine::new();

    let nil_ptr = vm.make_atom("nil")?;
    let t_ptr = vm.make_atom("t")?;

    // Check if equal to expected constants
    assert_eq!(nil_ptr, ConstSymbol::NIL);
    assert_eq!(t_ptr, ConstSymbol::T);

    // Check if they are self evaluating
    let nil_val = &vm
        .atoms
        .area
        .get(nil_ptr.value)
        .ok_or(())
        .map_err(|_| LispError::internal("Could not find atom"))?
        .value;

    let t_val = &vm
        .atoms
        .area
        .get(t_ptr.value)
        .ok_or(())
        .map_err(|_| LispError::internal("Could not find atom"))?
        .value;

    assert_eq!(nil_ptr, *nil_val);
    assert_eq!(ConstSymbol::NIL, *nil_val);
    assert_eq!(t_ptr, *t_val);
    assert_eq!(ConstSymbol::T, *t_val);

    Ok(())
}
