use crate::vm::constants::ConstSymbol;
use crate::vm::error::*;
use crate::vm::types::{DataType, Number, VirtualMachine};

/// Create and assign a value to an atom.
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

/// Test self-evaluating atoms `t` and `nil`.
#[test]
fn self_eval_atoms() -> LispResult<()> {
    let mut vm = VirtualMachine::new();

    let nil_ptr = vm.make_atom("nil")?;
    let t_ptr = vm.make_atom("t")?;

    // Check if equal to expected constants
    assert_eq!(nil_ptr, ConstSymbol::NIL);
    assert_eq!(t_ptr, ConstSymbol::T);

    // Check if they are self evaluating
    let nil_val = vm.lookup_atom_value(nil_ptr.clone())?;
    let t_val = vm.lookup_atom_value(t_ptr.clone())?;

    assert_eq!(nil_ptr, nil_val);
    assert_eq!(ConstSymbol::NIL, nil_val);
    assert_eq!(t_ptr, t_val);
    assert_eq!(ConstSymbol::T, t_val);

    Ok(())
}

/// Test number assignment.
/// - Creates an atom;
/// - Assigns some atom to it;
/// - Assigns a number to it;
/// - Assigns a new number to it (should reuse number slot);
/// - Assigns some atom again, rendering the number slot unused.
#[test]
fn assign_number() -> LispResult<()> {
    let mut vm = VirtualMachine::new();

    let my_atom = vm.make_atom("test")?;
    let my_number = vm.make_number(Number::Integer(50))?;

    // Assign atom to atom "test"
    vm.assign_value(my_atom.clone(), ConstSymbol::T)?;
    let lookup = vm.lookup_atom_value(my_atom.clone())?;
    assert_eq!(lookup.tag, DataType::Atom);
    assert_eq!(lookup, ConstSymbol::T);
    // (t is self-evaluating)
    assert_eq!(vm.atoms.area[lookup.value].name, "t");
    assert_eq!(vm.atoms.area[lookup.value].value, ConstSymbol::T);

    // Assign number 50 to atom "test", then assign 30
    vm.assign_value(my_atom.clone(), my_number.clone())?;
    let lookup = vm.lookup_atom_value(my_atom.clone())?;
    assert_eq!(lookup.tag, DataType::Number);
    assert_eq!(lookup, my_number);
    assert_eq!(vm.numbers.area[lookup.value], Number::Integer(50));

    let another_number = vm.make_number(Number::Integer(30))?;
    vm.assign_value(my_atom.clone(), another_number.clone())?;

    // TODO: The number 30 that we make is leaking here, but I suppose
    // it should be handled in another way by the VM -- garbage collection
    // should take care of it.

    let lookup = vm.lookup_atom_value(my_atom.clone())?;
    assert_eq!(lookup.tag, DataType::Number);
    assert_eq!(lookup, my_number); // Pointers must be the same
    assert!(my_number != another_number); // Values must not use same pointer

    // Value was copied to old number slot
    assert_eq!(vm.numbers.area[lookup.value], Number::Integer(30));

    // Assign other atom to "test", forcing a number deallocation
    vm.assign_value(my_atom.clone(), ConstSymbol::T)?;

    // Old looked-up number must be on unused numbers stack
    // TODO: Is this really a responsibility of the routine? This probably
    // should be handled by garbage collection.
    assert!(vm.numbers.unused.contains(&lookup.value));

    // Check if t is really assigned to atom "test"
    let lookup = vm.lookup_atom_value(my_atom)?;
    assert_eq!(lookup.tag, DataType::Atom);
    assert_eq!(lookup, ConstSymbol::T);
    assert_eq!(vm.atoms.area[lookup.value].name, "t");
    assert_eq!(vm.atoms.area[lookup.value].value, ConstSymbol::T);

    Ok(())
}

/// Allocate `ATOM_TABLE_SIZE` atoms, then expect error while allocating
/// the next one.
#[test]
fn allocate_max_atoms() -> LispResult<()> {
    use crate::vm::ATOM_TABLE_SIZE;

    let mut vm = VirtualMachine::new();

    let mut names = vec![];

    let mut gen_name = move || {
        use rand::{distributions::Uniform, Rng};

        let mut name;
        loop {
            name = rand::thread_rng()
                .sample_iter(Uniform::new(char::from(97), char::from(122)))
                .take(7)
                .map(char::from)
                .collect::<String>();

            if !names.contains(&name) {
                break;
            }
        }
        names.push(name.clone());
        name
    };

    while vm.atoms.last < ATOM_TABLE_SIZE {
        let name = gen_name();
        vm.make_atom(&name)?;
    }

    assert!(vm.make_atom(&gen_name()).is_err());
    assert!(vm.make_atom(&gen_name()).is_err());
    assert!(vm.make_atom(&gen_name()).is_err());

    Ok(())
}

/// Allocate `NUMBER_TABLE_SIZE` numbers, then expect error while allocating
/// the next one.
#[test]
fn allocate_max_numbers() -> LispResult<()> {
    use crate::vm::NUMBER_TABLE_SIZE;

    let mut vm = VirtualMachine::new();

    while vm.numbers.last < NUMBER_TABLE_SIZE {
        vm.make_number(Number::Integer(vm.numbers.last as i64))?;
    }

    assert!(vm.make_number(Number::Float(5.0)).is_err());
    assert!(vm.make_number(Number::Fraction(5, 6)).is_err());
    assert!(vm.make_number(Number::Integer(-6)).is_err());

    Ok(())
}

/// Test assignment and lookup with pointers that do not point to
/// atoms. These processes should fail.
#[test]
fn assign_to_non_atom() -> LispResult<()> {
    let mut vm = VirtualMachine::new();

    // Number 9, not assigned to any atom
    let number = vm.make_number(Number::Integer(9))?;

    // (t . nil), or (t)
    let cons = vm.make_cons()?;
    vm.set_car(&cons, ConstSymbol::T)?;
    vm.set_cdr(&cons, ConstSymbol::NIL)?;

    // test number for assignment
    let test = vm.make_number(Number::Integer(10))?;

    // Attempt illegal assignments to non-atoms
    assert!(vm.assign_value(number.clone(), ConstSymbol::ERROR).is_err());
    assert!(vm.assign_value(number.clone(), test.clone()).is_err());
    assert!(vm.assign_value(cons.clone(), ConstSymbol::ERROR).is_err());
    assert!(vm.assign_value(cons.clone(), test).is_err());

    // Attempt illegal lookups to non-atoms
    assert!(vm.lookup_atom_value(number).is_err());
    assert!(vm.lookup_atom_value(cons).is_err());

    Ok(())
}

/// Test assignment and lookup with pointer to an atom that does not
/// exist. These processes should fail.
#[test]
fn assign_to_illegal_atom() -> LispResult<()> {
    use crate::vm::TypedPointer;

    let mut vm = VirtualMachine::new();

    let illegal_atom = TypedPointer {
        tag: DataType::Atom,
        value: 99999,
    };

    let test = vm.make_number(Number::Integer(10))?;

    let cons = vm.make_cons()?;
    vm.set_car(&cons, ConstSymbol::T)?;
    vm.set_cdr(&cons, ConstSymbol::NIL)?;

    // Attempt illegal assignments to illegal atom
    assert!(vm.assign_value(illegal_atom.clone(), cons).is_err());
    assert!(vm.assign_value(illegal_atom.clone(), test).is_err());

    // Attempt illegal lookups to value of illegal atom
    assert!(vm.lookup_atom_value(illegal_atom).is_err());

    Ok(())
}
