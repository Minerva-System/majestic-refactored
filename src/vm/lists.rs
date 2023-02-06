use super::error::{LispError, LispResult};
use super::types::*;

impl VirtualMachine {
    pub fn make_cons(&mut self) -> LispResult<TypedPointer> {
        let ptr = self.lists.last;
        if ptr >= LIST_AREA_SIZE {
            Err(LispError::list_area_allocation())
        } else {
            self.lists.last += 1;
            Ok(TypedPointer::new(DataType::Cons, ptr))
        }
    }

    fn get_cons(&self, cons: &TypedPointer) -> LispResult<&Cons> {
        if cons.tag != DataType::Cons {
            return Err(LispError::internal("attempted to get CAR/CDR of non-cons"));
        }

        Ok(self.lists.area.get(cons.value).unwrap())
    }

    fn get_cons_mut(&mut self, cons: &TypedPointer) -> LispResult<&mut Cons> {
        if cons.tag != DataType::Cons {
            return Err(LispError::internal("attempted to set CAR/CDR of non-cons"));
        }

        Ok(self.lists.area.get_mut(cons.value).unwrap())
    }

    pub fn set_car(&mut self, cons: &TypedPointer, value: TypedPointer) -> LispResult<()> {
        let cons: &mut Cons = self.get_cons_mut(cons)?;
        cons.car = value;
        Ok(())
    }

    pub fn set_cdr(&mut self, cons: &TypedPointer, value: TypedPointer) -> LispResult<()> {
        let cons: &mut Cons = self.get_cons_mut(cons)?;
        cons.cdr = value;
        Ok(())
    }

    pub fn get_car(&self, cons: &TypedPointer) -> LispResult<TypedPointer> {
        let cons = self.get_cons(cons)?;
        Ok(cons.car.clone())
    }

    pub fn get_cdr(&self, cons: &TypedPointer) -> LispResult<TypedPointer> {
        let cons = self.get_cons(cons)?;
        Ok(cons.cdr.clone())
    }

    pub fn get_cadr(&self, cons: &TypedPointer) -> LispResult<TypedPointer> {
        self.get_car(&self.get_cdr(cons)?)
    }

    pub fn get_caddr(&self, cons: &TypedPointer) -> LispResult<TypedPointer> {
        self.get_car(&self.get_cdr(&self.get_cdr(cons)?)?)
    }
}
