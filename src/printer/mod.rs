use crate::vm::*;

#[cfg(test)]
mod test;

pub fn print_object(vm: &VirtualMachine, ptr: &TypedPointer) {
    print!("{}", format_object(vm, ptr));
}

pub fn format_object(vm: &VirtualMachine, ptr: &TypedPointer) -> String {
    match ptr.tag {
        DataType::Undefined => "undefined".to_string(),
        DataType::Number => format!("{}", vm.numbers.area[ptr.value]),
        DataType::Atom => vm.atoms.area[ptr.value].name.to_string(),
        DataType::Function => format!("#<FUNCTION {{{:#08x}}}>", ptr.value),
        DataType::Literal => format!("#<LITERAL {{{:#08x}}}>", ptr.value),
        DataType::BuiltInFunction => format!("#<BUILTIN-FUNCTION {{{:#08x}}}>", ptr.value),
        DataType::BuiltInLiteral => format!("#<BUILTIN-LITERAL {{{:#08x}}}>", ptr.value),
        DataType::Environment => format!("#<ENV{}>", ptr.value),
        DataType::Cons => {
            let mut s: String = String::new();
            s.push('(');
            s.push_str(&format_list(vm, ptr));
            s
        }
    }
}

pub fn format_list(vm: &VirtualMachine, ptr: &TypedPointer) -> String {
    let car = &vm.lists.area[ptr.value].car;
    let cdr = &vm.lists.area[ptr.value].cdr;

    let mut s: String = String::new();

    s.push_str(&format_object(vm, car));

    if cdr.tag == DataType::Cons {
        s.push(' ');
        s.push_str(&format_list(vm, cdr));
    } else if (cdr.tag == DataType::Atom) && (cdr.value == 0) {
        // Trick for checking for nil
        s.push(')');
    } else {
        s.push_str(" . ");
        s.push_str(&format_object(vm, cdr));
        s.push(')');
    }

    s
}
