use super::error::{LispError, LispResult};
use super::types::*;

impl VirtualMachine {
    pub fn stack_push(&mut self, ptr: TypedPointer) -> LispResult<()> {
        if self.stack.last >= LISP_STACK_SIZE {
            Err(LispError::stack_overflow())
        } else {
            let position = self.stack.last;
            self.stack.area[position] = ptr;
            self.stack.last += 1;
            Ok(())
        }
    }

    pub fn stack_peek(&self) -> LispResult<TypedPointer> {
        if self.stack.last == 0 {
            Err(LispError::stack_underflow())
        } else {
            Ok(self.stack.area[self.stack.last - 1].clone())
        }
    }

    pub fn stack_pop(&mut self) -> LispResult<TypedPointer> {
        if self.stack.last == 0 {
            Err(LispError::stack_underflow())
        } else {
            self.stack.last -= 1;
            Ok(self.stack.area[self.stack.last].clone())
        }
    }

    // Note: popped amount also counts the popped marker
    pub fn stack_unwind(&mut self, marker: TypedPointer) -> usize {
        let mut popped = 0;
        while self.stack.area[self.stack.last] != marker {
            if self.stack.last == 0 {
                return popped;
            }
            self.stack.last -= 1;
            popped += 1;
        }

        if self.stack.last > 0 {
            // Erase marker
            self.stack.area[self.stack.last] = TypedPointer::default();
        }

        popped
    }
}
