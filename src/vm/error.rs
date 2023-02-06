use std::{error::Error, fmt};

#[derive(Debug)]
enum LispErrorKind {
    StackOverflow,
    StackUnderflow,
    AtomTableAllocation,
    NumberTableAllocation,
    ListAreaAllocation,
    EnvironmentTableAllocation,
    Internal(&'static str),
    Arity(String),
}

#[derive(Debug)]
pub struct LispError {
    kind: LispErrorKind,
}

impl LispError {
    pub fn stack_overflow() -> Self {
        Self {
            kind: LispErrorKind::StackOverflow,
        }
    }

    pub fn stack_underflow() -> Self {
        Self {
            kind: LispErrorKind::StackUnderflow,
        }
    }

    pub fn atom_table_allocation() -> Self {
        Self {
            kind: LispErrorKind::AtomTableAllocation,
        }
    }

    pub fn number_table_allocation() -> Self {
        Self {
            kind: LispErrorKind::NumberTableAllocation,
        }
    }

    pub fn list_area_allocation() -> Self {
        Self {
            kind: LispErrorKind::ListAreaAllocation,
        }
    }

    pub fn environment_table_allocation() -> Self {
        Self {
            kind: LispErrorKind::EnvironmentTableAllocation,
        }
    }

    pub fn arity(fn_name: String) -> Self {
        Self {
            kind: LispErrorKind::Arity(fn_name),
        }
    }

    pub fn internal(reason: &'static str) -> Self {
        Self {
            kind: LispErrorKind::Internal(reason),
        }
    }
}

impl Error for LispError {}

impl fmt::Display for LispError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error with Majestic Lisp environment: {}",
            match &self.kind {
                LispErrorKind::StackOverflow => "stack overflow".to_owned(),
                LispErrorKind::StackUnderflow => "stack underflow".to_owned(),
                LispErrorKind::AtomTableAllocation => "atom table allocation error".to_owned(),
                LispErrorKind::NumberTableAllocation => "number table allocation error".to_owned(),
                LispErrorKind::ListAreaAllocation => "list area allocation error".to_owned(),
                LispErrorKind::EnvironmentTableAllocation =>
                    "environment area allocation error".to_owned(),
                LispErrorKind::Arity(name) =>
                    format!("arity error while applying function {}", name),
                LispErrorKind::Internal(cause) => format!("internal error: {}", cause),
            }
        )
    }
}

pub type LispResult<T> = Result<T, LispError>;
