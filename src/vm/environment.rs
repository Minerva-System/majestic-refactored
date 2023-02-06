use super::error::{LispError, LispResult};
use super::{types::*, ConstSymbol};

impl VirtualMachine {
    pub fn make_environment(&mut self, prev: TypedPointer) -> LispResult<TypedPointer> {
        if (prev.tag != DataType::Environment) && (prev != ConstSymbol::NIL) {
            return Err(LispError::internal(
                "attempted to build environment with invalid parent",
            ));
        }

        if self.environments.last > ENV_TABLE_SIZE {
            return Err(LispError::environment_table_allocation());
        }

        let ptr = self.environments.last;
        self.environments.last += 1;

        self.environments.area[ptr].prev = prev;

        Ok(TypedPointer::new(DataType::Environment, ptr))
    }

    pub fn env_bind(
        &mut self,
        env: TypedPointer,
        atom: TypedPointer,
        value: TypedPointer,
    ) -> LispResult<()> {
        if env.tag != DataType::Environment {
            return Err(LispError::internal(
                "attempted to use non-environment as environment",
            ));
        }

        if atom.tag != DataType::Atom {
            return Err(LispError::internal(
                "attempted to assign value to non-atom within environment",
            ));
        }

        let env: &mut Environment = self.environments.area.get_mut(env.value).unwrap();

        let _ = env.data.insert(atom, value);

        Ok(())
    }

    pub fn env_lookup(
        &self,
        env: TypedPointer,
        atom: TypedPointer,
    ) -> LispResult<Option<TypedPointer>> {
        if env.tag != DataType::Environment {
            return Err(LispError::internal(
                "attempted to use non-environment as environment",
            ));
        }

        if atom.tag != DataType::Atom {
            return Err(LispError::internal(
                "attempted to get value of non-atom within environment",
            ));
        }

        let env: &Environment = self.environments.area.get(env.value).unwrap();

        Ok(env.data.get(&atom).cloned())
    }

    pub fn env_parent(&self, env: TypedPointer) -> LispResult<TypedPointer> {
        if env.tag != DataType::Environment {
            return Err(LispError::internal(
                "attempted to use non-environment as environment",
            ));
        }

        let env: &Environment = self.environments.area.get(env.value).unwrap();

        Ok(env.prev.clone())
    }
}
