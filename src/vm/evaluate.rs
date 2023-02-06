use std::collections::VecDeque;

use super::constants::*;
use super::error::{LispError, LispResult};
use super::types::*;
use log::{debug, trace};

impl VirtualMachine {
    pub fn evaluate(&mut self, exp: TypedPointer) -> LispResult<TypedPointer> {
        self.registers.exp = exp;
        self.registers.env = ConstSymbol::E0;
        self.registers.cont = ConstSymbol::DONE;
        self.registers.val = TypedPointer::default();
        self.ev_eval_dispatch()?;

        Ok(self.registers.val.clone())
    }

    fn ev_eval_dispatch(&mut self) -> LispResult<()> {
        trace!("eval_dispatch");
        let exp = self.registers.exp.clone();
        match exp.tag {
            // Self-evaluating expressions
            DataType::Number | DataType::Literal => self.ev_self_eval(),

            // Variables
            DataType::Atom => self.ev_variable(),

            _ => {
                // Special Forms
                if EvalHelper::special_form_p(&self, ConstSymbol::SETQ, exp.clone())? {
                    self.ev_setq()
                } else if EvalHelper::special_form_p(&self, ConstSymbol::QUOTE, exp.clone())? {
                    self.ev_quote()
                } else if EvalHelper::special_form_p(&self, ConstSymbol::FN, exp.clone())? {
                    self.ev_fn()
                } else if EvalHelper::special_form_p(&self, ConstSymbol::DO, exp.clone())? {
                    return Err(LispError::internal(
                        "unimplemented evaluation for special form DO",
                    ));
                } else {
                    // Application
                    if EvalHelper::applicationp(&self, exp)? {
                        self.ev_application()
                    } else {
                        self.ev_expression_error()
                    }
                }
            }
        }
    }

    fn ev_self_eval(&mut self) -> LispResult<()> {
        trace!("self_eval");
        self.registers.val = self.registers.exp.clone();

        self.ev_goto_continue_register()
    }

    fn ev_variable(&mut self) -> LispResult<()> {
        trace!("variable");
        self.registers.val = self.lookup(self.registers.exp.clone())?;

        self.ev_goto_continue_register()
    }

    fn ev_setq(&mut self) -> LispResult<()> {
        trace!("setq");
        self.stack_push(self.registers.cont.clone())?;
        let cadr = self.get_cadr(&self.registers.exp.clone())?;
        self.stack_push(cadr)?;
        self.registers.exp = self.get_caddr(&self.registers.exp.clone())?;
        self.registers.cont = ConstSymbol::EVAL_ASSIGN;

        self.ev_eval_dispatch()
    }

    // ev-fn
    // (push (fetch unev))
    // (assign val (cons (fetch env) nil))
    // (assign unev (cdr (fetch exp)))
    // (assign val (cons (fetch unev) (fetch val)))
    // (assign val (ptr 'function (untype (fetch val))))
    // (pop unev)
    fn ev_fn(&mut self) -> LispResult<()> {
        trace!("fn");
        self.stack_push(self.registers.unev.clone())?;

        let cons1 = self.make_cons()?;
        self.set_car(&cons1, self.registers.env.clone())?;
        self.set_cdr(&cons1, ConstSymbol::NIL)?;
        self.registers.val = cons1;

        self.registers.unev = self.get_cdr(&self.registers.exp.clone())?;

        let cons2 = self.make_cons()?;
        self.set_car(&cons2, self.registers.unev.clone())?;
        self.set_cdr(&cons2, self.registers.val.clone())?;
        self.registers.val = cons2;

        let ptr = TypedPointer::new(DataType::Function, self.registers.val.value);
        self.registers.val = ptr;

        self.registers.unev = self.stack_pop()?;

        self.ev_goto_continue_register()
    }

    fn ev_quote(&mut self) -> LispResult<()> {
        trace!("quote");
        self.registers.val = self.get_cadr(&self.registers.exp.clone())?;

        self.ev_goto_continue_register()
    }

    fn ev_application(&mut self) -> LispResult<()> {
        trace!("application");
        self.registers.unev = self.get_cdr(&self.registers.exp.clone())?;
        self.registers.exp = self.get_car(&self.registers.exp.clone())?;
        self.stack_push(self.registers.cont.clone())?;
        self.stack_push(self.registers.env.clone())?;
        self.stack_push(self.registers.unev.clone())?;
        self.registers.cont = ConstSymbol::EVAL_ARGS;

        self.ev_eval_dispatch()
    }

    fn ev_goto_continue_register(&mut self) -> LispResult<()> {
        trace!("goto_continue_register");
        let cont = self.registers.cont.clone();
        match cont {
            ConstSymbol::DONE => self.ev_done(),
            ConstSymbol::EVAL_ARGS => self.ev_eval_args(),
            ConstSymbol::ACCUMULATE_ARG => self.ev_accumulate_arg(),
            ConstSymbol::ACCUMULATE_LAST_ARG => self.ev_accumulate_last_arg(),
            ConstSymbol::EVAL_ASSIGN => self.ev_eval_assign(),
            _ => self.ev_expression_error(),
        }
    }

    fn ev_done(&mut self) -> LispResult<()> {
        trace!("done");
        // Result is in registers.val
        Ok(())
    }

    fn ev_eval_args(&mut self) -> LispResult<()> {
        trace!("eval_args");
        self.registers.unev = self.stack_pop()?;
        self.registers.env = self.stack_pop()?;
        self.registers.fun = self.registers.val.clone();
        self.stack_push(self.registers.fun.clone())?;
        self.registers.argl = ConstSymbol::NIL;

        self.ev_eval_arg_loop()
    }

    fn ev_eval_arg_loop(&mut self) -> LispResult<()> {
        trace!("eval_arg_loop");
        self.stack_push(self.registers.argl.clone())?;

        if self.registers.unev.clone() == ConstSymbol::NIL {
            self.registers.argl = self.stack_pop()?;
            self.registers.fun = self.stack_pop()?;

            return self.ev_apply_dispatch();
        }

        self.registers.exp = self.get_car(&self.registers.unev.clone())?;

        if EvalHelper::last_operand_p(&self, self.registers.unev.clone())? {
            self.ev_eval_last_arg()
        } else {
            self.stack_push(self.registers.env.clone())?;
            self.stack_push(self.registers.unev.clone())?;
            self.registers.cont = ConstSymbol::ACCUMULATE_ARG;

            self.ev_eval_dispatch()
        }
    }

    fn ev_eval_last_arg(&mut self) -> LispResult<()> {
        trace!("eval_last_arg");
        self.registers.cont = ConstSymbol::ACCUMULATE_LAST_ARG;

        self.ev_eval_dispatch()
    }

    fn ev_accumulate_arg(&mut self) -> LispResult<()> {
        trace!("accumulate_arg");
        self.registers.unev = self.stack_pop()?;
        self.registers.env = self.stack_pop()?;
        self.registers.argl = self.stack_pop()?;
        let new_argl = self.make_cons()?;
        self.set_car(&new_argl, self.registers.val.clone())?;
        self.set_cdr(&new_argl, self.registers.argl.clone())?;
        self.registers.argl = new_argl;
        self.registers.unev = self.get_cdr(&self.registers.unev.clone())?;

        self.ev_eval_arg_loop()
    }

    fn ev_accumulate_last_arg(&mut self) -> LispResult<()> {
        trace!("accumulate_last_arg");
        self.registers.argl = self.stack_pop()?;

        let new_argl = self.make_cons()?;
        self.set_car(&new_argl, self.registers.val.clone())?;
        self.set_cdr(&new_argl, self.registers.argl.clone())?;
        self.registers.argl = new_argl;
        self.registers.fun = self.stack_pop()?;

        self.ev_apply_dispatch()
    }

    fn ev_eval_assign(&mut self) -> LispResult<()> {
        trace!("eval_assign");
        self.registers.exp = self.registers.val.clone();
        self.registers.val = self.stack_pop()?;
        self.assign_value(self.registers.val.clone(), self.registers.exp.clone())?;
        self.registers.val = self.registers.exp.clone();
        self.registers.cont = self.stack_pop()?;

        self.ev_goto_continue_register()
    }

    fn ev_expression_error(&mut self) -> LispResult<()> {
        trace!("expression_error");
        Err(LispError::internal("expression error"))
    }
}

impl VirtualMachine {
    fn ev_apply_dispatch(&mut self) -> LispResult<()> {
        trace!("apply_dispatch");

        let fun = self.registers.fun.clone();

        if EvalHelper::primitive_function_p(fun.clone()) {
            self.ev_primitive_fn_apply()
        } else if EvalHelper::compound_function_p(fun.clone()) {
            self.ev_compound_fn_apply()
        } else {
            Err(LispError::internal("unknown function type"))
        }
    }

    fn ev_primitive_fn_apply(&mut self) -> LispResult<()> {
        trace!("primitive_fn_apply");
        self.registers.val =
            self.ev_apply_primitive_fn(self.registers.fun.clone(), self.registers.argl.clone())?;
        self.registers.cont = self.stack_pop()?;

        self.ev_goto_continue_register()
    }

    fn ev_compound_fn_apply(&mut self) -> LispResult<()> {
        trace!("compound_fn_apply");

        let (lambda_list, body, env) = EvalHelper::get_fn_parts(&self, self.registers.fun.clone())?;

        self.registers.exp = EvalHelper::prepare_multiple_list_eval(self, body)?;

        self.registers.env = self.ev_make_bindings(
            lambda_list.clone(),
            self.registers.argl.clone(),
            env.clone(),
        )?;
        self.registers.cont = self.stack_pop()?;

        self.ev_eval_dispatch()
    }

    fn ev_apply_primitive_fn(
        &mut self,
        fun: TypedPointer,
        argl: TypedPointer,
    ) -> LispResult<TypedPointer> {
        trace!("apply_primitive_fn");
        // apply primitive fn to list of arguments.
        // Invert ARGL into vector
        let argl = {
            let mut v = Vec::new();
            let mut argl = argl.clone();

            while argl != ConstSymbol::NIL {
                let car = self.get_car(&argl.clone())?;
                argl = self.get_cdr(&argl.clone())?;
                v.push(car);
            }

            v
        };

        self.dispatch_prim_eval(fun, &argl)
    }

    fn ev_make_bindings(
        &mut self,
        lambda_list: TypedPointer,
        argl: TypedPointer,
        env: TypedPointer,
    ) -> LispResult<TypedPointer> {
        trace!("make_bindings");

        // Invert ARGL into vector
        let mut argl_inv = VecDeque::new();
        let mut lambda_list_vec = VecDeque::new();

        // TODO: These bindings do not consider special cases yet.
        // For now, arity does not change!
        let bindings = {
            let mut argl = argl.clone();
            let mut ll = lambda_list.clone();

            while argl != ConstSymbol::NIL {
                let car = self.get_car(&argl.clone())?;
                argl = self.get_cdr(&argl.clone())?;
                argl_inv.push_back(car);
            }

            while ll != ConstSymbol::NIL {
                let car = self.get_car(&ll.clone())?;
                ll = self.get_cdr(&ll.clone())?;
                lambda_list_vec.push_front(car);
            }

            lambda_list_vec.iter().zip(argl_inv.iter())
        };

        let new_env = self.make_environment(env)?;

        for (symbol, value) in bindings {
            self.env_bind(new_env.clone(), symbol.clone(), value.clone())?;
        }

        Ok(new_env)
    }
}

struct EvalHelper;
impl EvalHelper {
    #[inline]
    fn special_form_p(
        vm: &VirtualMachine,
        special: TypedPointer,
        exp: TypedPointer,
    ) -> LispResult<bool> {
        Ok((exp.tag == DataType::Cons) && (vm.get_car(&exp.clone())? == special))
    }

    #[inline]
    fn functionp(ptr: TypedPointer) -> bool {
        (ptr.tag == DataType::BuiltInFunction) || (ptr.tag == DataType::Function)
    }

    #[inline]
    fn applicationp(vm: &VirtualMachine, ptr: TypedPointer) -> LispResult<bool> {
        // TODO: check if car is function OR special form
        Ok((ptr.tag == DataType::Cons) && (Self::functionp(vm.lookup(vm.get_car(&ptr.clone())?)?)))
    }

    #[inline]
    fn last_operand_p(vm: &VirtualMachine, ptr: TypedPointer) -> LispResult<bool> {
        Ok(vm.get_cdr(&ptr)? == ConstSymbol::NIL)
    }

    #[inline]
    fn primitive_function_p(ptr: TypedPointer) -> bool {
        ptr.tag == DataType::BuiltInFunction
    }

    // #[inline]
    // pub fn primitive_special_p(ptr: TypedPointer) -> bool {
    // 	ptr.tag == DataType::BuiltInLiteral
    // }

    #[inline]
    fn compound_function_p(ptr: TypedPointer) -> bool {
        ptr.tag == DataType::Function
    }

    /// Returns a tuple (lambda-list, body, environment)
    #[inline]
    fn get_fn_parts(
        vm: &VirtualMachine,
        ptr: TypedPointer,
    ) -> LispResult<(TypedPointer, TypedPointer, TypedPointer)> {
        trace!("helper--get_fn_parts");
        if ptr.tag != DataType::Function {
            return Err(LispError::internal(
                "Attempted to dismember non-function into function parts",
            ));
        }

        // Cast value of FUN to CONS since functions are stored on list area
        let fun = TypedPointer::new(DataType::Cons, ptr.value);

        // ( (lambda-list . body) <env> )
        let env = vm.get_cadr(&fun)?;
        let fun = vm.get_car(&fun)?;
        let lambda_list = vm.get_car(&fun)?;
        let body = vm.get_cdr(&fun)?;

        Ok((lambda_list, body, env))
    }

    #[inline]
    fn prepare_multiple_list_eval(
        vm: &mut VirtualMachine,
        list: TypedPointer,
    ) -> LispResult<TypedPointer> {
        trace!("helper--prepare_multiple_list_eval");
        let cons = vm.make_cons()?;
        vm.set_car(&cons, ConstSymbol::DO)?;
        vm.set_cdr(&cons, list)?;
        Ok(cons)
    }
}

impl VirtualMachine {
    pub fn lookup(&self, atom: TypedPointer) -> LispResult<TypedPointer> {
        if atom.tag != DataType::Atom {
            return Err(LispError::internal("attempted to lookup value of non-atom"));
        }

        let mut env = self.registers.env.clone();

        let mut value = TypedPointer::default();

        while env != ConstSymbol::NIL {
            match self.env_lookup(env.clone(), atom.clone())? {
                Some(v) => {
                    value = v;
                    break;
                }
                None => env = self.env_parent(env.clone())?,
            }
        }

        if value.tag != DataType::Undefined {
            return Ok(value);
        }

        let atom: &Atom = self.atoms.area.get(atom.value).unwrap();
        Ok(atom.value.clone())
    }
}
