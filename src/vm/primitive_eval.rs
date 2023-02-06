use super::error::{LispError, LispResult};
use super::{types::*, ConstSymbol};
use log::{debug, trace, warn};

impl VirtualMachine {
    pub fn dispatch_prim_eval(
        &mut self,
        fun: TypedPointer,
        argl: &[TypedPointer],
    ) -> LispResult<TypedPointer> {
        trace!("dispatch_prim_eval");
        debug!("argl has {} args", argl.len());
        match fun {
            ConstSymbol::BIN_CONS => builtin_cons(self, argl),
            ConstSymbol::BIN_CAR => builtin_car(self, argl),
            ConstSymbol::BIN_CDR => builtin_cdr(self, argl),
            ConstSymbol::BIN_LIST => builtin_list(self, argl),
            ConstSymbol::BIN_EVAL => builtin_eval(self, argl),
            ConstSymbol::BIN_EQ => builtin_eq(self, argl),
            _ => Err(LispError::internal("unknown primitive function")),
        }
    }
}

fn builtin_cons(vm: &mut VirtualMachine, argl: &[TypedPointer]) -> LispResult<TypedPointer> {
    trace!("builtin_cons");
    if argl.len() != 2 {
        return Err(LispError::arity("cons".to_owned()));
    }

    let car = argl[0].clone();
    let cdr = argl[1].clone();

    let cons = vm.make_cons()?;
    vm.set_car(&cons, car)?;
    vm.set_cdr(&cons, cdr)?;

    Ok(cons)
}

fn builtin_car(vm: &mut VirtualMachine, argl: &[TypedPointer]) -> LispResult<TypedPointer> {
    trace!("builtin_car");
    if argl.len() != 1 {
        return Err(LispError::arity("car".to_owned()));
    }

    let value = argl[0].clone();

    if value == ConstSymbol::NIL {
        Ok(ConstSymbol::NIL)
    } else {
        vm.get_car(&value)
    }
}

fn builtin_cdr(vm: &mut VirtualMachine, argl: &[TypedPointer]) -> LispResult<TypedPointer> {
    trace!("builtin_cdr");
    if argl.len() != 1 {
        return Err(LispError::arity("car".to_owned()));
    }

    let value = argl[0].clone();

    if value == ConstSymbol::NIL {
        Ok(ConstSymbol::NIL)
    } else {
        vm.get_cdr(&value)
    }
}

fn builtin_list(vm: &mut VirtualMachine, argl: &[TypedPointer]) -> LispResult<TypedPointer> {
    trace!("builtin_list");
    let mut iter = ConstSymbol::NIL;
    for value in argl.iter().rev() {
        let cons = vm.make_cons()?;
        vm.set_car(&cons, value.clone())?;
        vm.set_cdr(&cons, iter.clone())?;
        iter = cons;
    }

    Ok(iter)
}

fn builtin_eval(vm: &mut VirtualMachine, argl: &[TypedPointer]) -> LispResult<TypedPointer> {
    trace!("builtin_eval");
    if argl.len() != 1 {
        return Err(LispError::arity("eval".to_owned()));
    }

    // Push stack frame
    vm.stack_push(vm.registers.argl.clone())?;
    vm.stack_push(vm.registers.cont.clone())?;
    vm.stack_push(vm.registers.env.clone())?;
    vm.stack_push(vm.registers.exp.clone())?;
    vm.stack_push(vm.registers.fun.clone())?;
    vm.stack_push(vm.registers.unev.clone())?;

    // Evaluate
    let val = vm.evaluate(argl[0].clone())?;

    // Pop stack frame
    vm.registers.unev = vm.stack_pop()?;
    vm.registers.fun = vm.stack_pop()?;
    vm.registers.exp = vm.stack_pop()?;
    vm.registers.env = vm.stack_pop()?;
    vm.registers.cont = vm.stack_pop()?;
    vm.registers.argl = vm.stack_pop()?;

    Ok(val)
}

fn builtin_eq(vm: &mut VirtualMachine, argl: &[TypedPointer]) -> LispResult<TypedPointer> {
    trace!("builtin_eq");
    if argl.len() != 2 {
        return Err(LispError::arity("eq".to_owned()));
    }

    let first = argl[0].clone();
    let second = argl[1].clone();

    let convert = |v| if v { ConstSymbol::T } else { ConstSymbol::NIL };

    Ok(if first.tag != second.tag {
        ConstSymbol::NIL
    } else {
        match first.tag {
            // Most values can be pointer-compared
            DataType::Atom
            | DataType::Cons
            | DataType::BuiltInFunction
            | DataType::BuiltInLiteral
            | DataType::Function
            | DataType::Literal => convert(first.value == second.value),
            // Environment comparison is undefined, so we better not compare at all
            DataType::Environment => {
                return Err(LispError::internal(
                    "attempted to eq-compare two environments",
                ))
            }
            // Numbers should be compared by actual value
            DataType::Number => {
                warn!("eq-comparing numbers; this should be improved");
                let first: &Number = vm.numbers.area.get(first.value).unwrap();
                let second: &Number = vm.numbers.area.get(second.value).unwrap();
                convert(first == second)
            }
            // "undefined == undefined" could be seen as true, but this
            // does not make sense at all
            DataType::Undefined => {
                return Err(LispError::internal(
                    "attempted to compare two undefined values",
                ))
            }
        }
    })
}
