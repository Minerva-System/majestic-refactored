use super::error::{LispError, LispResult};
use super::{types::*, ConstSymbol};
use log::{debug, trace};

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
