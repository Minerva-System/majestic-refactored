use super::expression::*;
use crate::vm::error::*;
use crate::vm::*;

pub fn build_ast(mut vm: &mut VirtualMachine, expr: Expr) -> LispResult<TypedPointer> {
    match expr {
        Expr::Atom(atom_expr) => build_atom_ast(&mut vm, atom_expr),
        Expr::Prefixed(prefix, boxed_expr) => build_prefixed_ast(&mut vm, prefix, *boxed_expr),
        Expr::List(exprs) => build_list_ast(&mut vm, exprs),
        Expr::DottedList(exprs) => build_dotted_list_ast(&mut vm, exprs),
        Expr::Vector(_exprs) => Err(LispError::internal("vector storage not implemented")),
        Expr::Cons(boxed_car, boxed_cdr) => build_cons_ast(&mut vm, *boxed_car, *boxed_cdr),
        _ => vm.make_atom("nil"),
    }
}

fn build_atom_ast(vm: &mut VirtualMachine, atom_expr: AtomExpr) -> LispResult<TypedPointer> {
    match atom_expr {
        AtomExpr::Number(number_expr) => vm.make_number(build_number_ast(number_expr)),
        AtomExpr::String(_string) => Err(LispError::internal("string storage not implemented")),
        AtomExpr::Symbol(name) => vm.make_atom(&name),
    }
}

fn build_number_ast(expr: NumberExpr) -> Number {
    match expr {
        NumberExpr::Integer(num) => Number::Integer(num),
        NumberExpr::Float(num) => Number::Float(num),
        NumberExpr::Fraction(numer, denom) => Number::Fraction(numer, denom),
        NumberExpr::Complex(real, imag) => {
            Number::complex(build_number_ast(*real), build_number_ast(*imag))
        }
    }
}

fn build_list_ast(mut vm: &mut VirtualMachine, exprs: Vec<Expr>) -> LispResult<TypedPointer> {
    if exprs.is_empty() {
        return Ok(ConstSymbol::NIL);
    }

    let first = vm.make_cons()?;
    let mut iter = first.clone();
    for (i, expr) in exprs.iter().enumerate() {
        let ptr = build_ast(&mut vm, expr.clone())?;
        vm.set_car(&iter, ptr)?;

        if i == exprs.len() - 1 {
            vm.set_cdr(&iter, ConstSymbol::NIL)?;
        } else {
            let cons = vm.make_cons()?;
            vm.set_cdr(&iter, cons.clone())?;
            iter = cons;
        }
    }

    Ok(first)
}

fn build_dotted_list_ast(
    mut vm: &mut VirtualMachine,
    exprs: Vec<Expr>,
) -> LispResult<TypedPointer> {
    if exprs.is_empty() {
        // probably a weird syntax error that slipped through the cracks,
        // if this code is reached
        eprintln!("Empty dotted list detected. Is there a parser problem?");
        return Ok(ConstSymbol::NIL);
    }

    // also notice that we shouldn't have a dotted list with a single element
    if exprs.len() == 1 {
        panic!("There is no such thing as a single-element dotted list");
    }

    let first = vm.make_cons()?;
    let mut iter = first.clone();
    for (i, expr) in exprs.iter().enumerate() {
        let ptr = build_ast(&mut vm, expr.clone())?;

        if i == exprs.len() - 1 {
            // If we're at the end, assign to previous cdr
            vm.set_cdr(&iter, ptr)?;
        } else {
            if i > 0 {
                let cons = vm.make_cons()?;
                vm.set_cdr(&iter, cons.clone())?;
                iter = cons;
            }
            vm.set_car(&iter, ptr)?;
        }
    }

    Ok(first)
}

fn build_cons_ast(
    mut vm: &mut VirtualMachine,
    car_expr: Expr,
    cdr_expr: Expr,
) -> LispResult<TypedPointer> {
    let car = build_ast(&mut vm, car_expr)?;
    let cdr = build_ast(&mut vm, cdr_expr)?;

    let cons = vm.make_cons()?;
    vm.set_car(&cons, car)?;
    vm.set_cdr(&cons, cdr)?;

    Ok(cons)
}

pub fn build_prefixed_ast(
    mut vm: &mut VirtualMachine,
    prefix: PrefixType,
    expr: Expr,
) -> LispResult<TypedPointer> {
    let prefix_expr = Expr::make_symbol(
        match prefix {
            PrefixType::Quote => "quote",
            PrefixType::Quasiquote => "quasiquote",
            PrefixType::Unquote => "unquote",
            PrefixType::UnquoteSplice => "unquote-splice",
        }
        .to_owned(),
    );

    build_list_ast(&mut vm, vec![prefix_expr, expr])
}
