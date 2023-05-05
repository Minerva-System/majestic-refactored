use super::error::{LispError, LispResult};
use super::types::*;

impl VirtualMachine {
    pub fn make_atom(&mut self, name: &str) -> LispResult<TypedPointer> {
        if let Some(untptr) = self.atom_index.get(name) {
            if untptr >= &self.atoms.last {
                return Err(LispError::internal(
                    "attempted to retrieve an unallocated atom",
                ));
            }

            // FIXME: Is this really necessary? Since atoms are never disallocated,
            // we should probably trust what's on the index
            return match self.atoms.area.get(*untptr) {
                Some(_) => Ok(TypedPointer::new(DataType::Atom, *untptr)),
                None => Err(LispError::internal("atom pointer out of range")),
            };
        }

        if self.atoms.last > ATOM_TABLE_SIZE {
            return Err(LispError::atom_table_allocation());
        }

        let ptr = self.atoms.last;
        self.atoms.last += 1;

        let atom: &mut Atom = self.atoms.area.get_mut(ptr).unwrap();
        atom.name = String::from(name);
        atom.value = TypedPointer::new(DataType::Undefined, 0);
        // atom.bindlist = 0;
        // atom.plist = 0;

        self.atom_index.insert(String::from(name), ptr);

        Ok(TypedPointer::new(DataType::Atom, ptr))
    }

    pub fn make_number(&mut self, value: Number) -> LispResult<TypedPointer> {
        let ptr = self.numbers.get_next_unsafe();
        if ptr >= NUMBER_TABLE_SIZE {
            return Err(LispError::number_table_allocation());
        }
        self.numbers.area[ptr] = value;
        Ok(TypedPointer::new(DataType::Number, ptr))
    }

    pub fn assign_value(&mut self, atom: TypedPointer, value: TypedPointer) -> LispResult<()> {
        if atom.tag != DataType::Atom {
            return Err(LispError::internal("attempted to assign value to non-atom"));
        }

        if atom.value >= self.atoms.last {
            return Err(LispError::internal(
                "attempted to assign to unallocated atom",
            ));
        }

        let atom: &mut Atom = self.atoms.area.get_mut(atom.value).unwrap();

        // Treat edge cases if the atom is currently holding a number.
        if atom.value.tag == DataType::Number {
            // If assigning a non-number to the atom, deallocate number slot
            if value.tag != DataType::Number {
                self.numbers.unused.push_back(atom.value.value);
                atom.value = value;
            } else {
                // If assigning a number to this atom which currently holds
                // a number, then just copy the number
                let new_number: &Number = self.numbers.area.get(value.value).unwrap();
                self.numbers.area[atom.value.value] = new_number.clone();
            }
        } else {
            atom.value = value;
        }

        Ok(())
    }

    pub fn lookup_atom_value(&self, atom: TypedPointer) -> LispResult<TypedPointer> {
        if atom.tag != DataType::Atom {
            return Err(LispError::internal("attempted to lookup value of non-atom"));
        }

        if atom.value >= self.atoms.last {
            return Err(LispError::internal(
                "attempted to lookup assigned value of unallocated atom",
            ));
        }

        let atom: &Atom = self.atoms.area.get(atom.value).ok_or(()).map_err(|_| {
            LispError::internal("attempted to lookup assigned value of unexisting atom")
        })?;
        Ok(atom.value.clone())
    }
}
