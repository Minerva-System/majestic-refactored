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
    let nil_val = vm.lookup_atom_value(nil_ptr.clone())?;
    let t_val = vm.lookup_atom_value(t_ptr.clone())?;

    assert_eq!(nil_ptr, nil_val);
    assert_eq!(ConstSymbol::NIL, nil_val);
    assert_eq!(t_ptr, t_val);
    assert_eq!(ConstSymbol::T, t_val);

    Ok(())
}

#[test]
fn reassign_number() -> LispResult<()> {
    let mut vm = VirtualMachine::new();

    let my_atom = vm.make_atom("test")?;
    let my_number = vm.make_number(Number::Integer(50))?;
    vm.assign_value(my_atom.clone(), my_number.clone())?;

    let lookup = vm.lookup_atom_value(my_atom.clone())?;
    assert_eq!(lookup, my_number);
    assert_eq!(vm.numbers.area[lookup.value], Number::Integer(50));

    let another_number = vm.make_number(Number::Integer(30))?;
    vm.assign_value(my_atom.clone(), another_number.clone())?;

    let lookup = vm.lookup_atom_value(my_atom)?;
    assert_eq!(lookup, my_number); // Pointers must be the same
    assert!(my_number != another_number); // Values must not use same pointer

    // Value was copied to old number slot
    assert_eq!(vm.numbers.area[lookup.value], Number::Integer(30));

    Ok(())
}

// allocate ATOM_TABLE_SIZE atoms, then expect error
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

// allocate NUMBER_TABLE_SIZE numbers, then expect error

// attempt assignment to non-atom value, then expect error

// create fake unallocated atom with big index, attempt
// assignment, then expect error
