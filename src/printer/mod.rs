use crate::vm::*;

pub fn print_object(vm: &VirtualMachine, ptr: &TypedPointer) {
    match ptr.tag {
        DataType::Undefined => print!("undefined"),
        DataType::Number => print!("{}", vm.numbers.area[ptr.value]),
        DataType::Atom => print!("{}", vm.atoms.area[ptr.value].name),
        DataType::Function => print!("#<FUNCTION {{{:#08x}}}>", ptr.value),
        DataType::Literal => print!("#<LITERAL {{{:#08x}}}>", ptr.value),
        DataType::BuiltInFunction => print!("#<BUILTIN-FUNCTION {{{:#08x}}}>", ptr.value),
        DataType::BuiltInLiteral => print!("#<BUILTIN-LITERAL {{{:#08x}}}>", ptr.value),
        DataType::Cons => {
            print!("(");
            print_list(&vm, &ptr);
        }
    }
}

pub fn print_list(vm: &VirtualMachine, ptr: &TypedPointer) {
    let car = &vm.lists.area[ptr.value].car;
    let cdr = &vm.lists.area[ptr.value].cdr;

    print_object(&vm, car);

    if cdr.tag == DataType::Cons {
        print!(" ");
        print_list(&vm, cdr);
    } else if (cdr.tag == DataType::Atom) && (cdr.value == 0) {
        // Trick for checking for nil
        print!(")");
    } else {
        print!(" . ");
        print_object(&vm, cdr);
        print!(")");
    }
}
