// pub const ATOM_TABLE_SIZE: usize = 100000; // 300000 atoms
// pub const NUMBER_TABLE_SIZE: usize = 100000; // 300000 numbers (indexed after atom table)
// pub const LIST_AREA_SIZE: usize = 16777216; // 16MB list area
// pub const LISP_STACK_SIZE: usize = 8388608; // 8MB stack

use super::ConstSymbol;
use radix_trie::Trie;

pub const ATOM_TABLE_SIZE: usize = 10000; // 30000 atoms
pub const NUMBER_TABLE_SIZE: usize = 10000; // 30000 numbers (indexed after atom table)
pub const LIST_AREA_SIZE: usize = 524288; // # of cells, total 16MB
pub const LISP_STACK_SIZE: usize = 524288; // # of pointers, total 8MB
pub const ENV_TABLE_SIZE: usize = 2000; // 1000 environments
pub const MAX_ENV_CAPACITY: usize = 200; // Each env can contain at most 200 bindings

pub type UntypedPointer = usize;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DataType {
    Undefined,
    Cons,
    Atom,
    Number,
    BuiltInFunction,
    BuiltInLiteral,
    Function,
    Literal,
    Environment,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TypedPointer {
    pub tag: DataType,
    pub value: UntypedPointer,
}

impl TypedPointer {
    pub fn new(tag: DataType, value: UntypedPointer) -> Self {
        Self { tag, value }
    }
}

impl Default for TypedPointer {
    fn default() -> Self {
        Self::new(DataType::Undefined, 0)
    }
}

impl std::fmt::Display for TypedPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}::{:#08x}",
            match self.tag {
                DataType::Undefined => "UNDEF",
                DataType::Cons => " CONS",
                DataType::Atom => " ATOM",
                DataType::Number => "  NUM",
                DataType::BuiltInFunction => "BINFN",
                DataType::BuiltInLiteral => "BINLT",
                DataType::Function => "FUNCT",
                DataType::Literal => "LITER",
                DataType::Environment => "  ENV",
                _ => "UNKNW",
            },
            self.value
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Cons {
    pub marked: u8,
    pub car: TypedPointer,
    pub cdr: TypedPointer,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Atom {
    pub name: String,
    pub value: TypedPointer,
    // pub plist: UntypedPointer,
    // pub bindlist: UntypedPointer,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Number {
    #[default]
    Undefined,
    Integer(i64),
    Float(f64),
    Fraction(i64, i64),
    Complex(Box<Number>, Box<Number>),
}

impl Number {
    pub fn complex(real: Number, imag: Number) -> Number {
        if let Number::Complex(_, _) = real {
            panic!("Cannot create complex number with complex real part");
        }

        if let Number::Complex(_, _) = imag {
            panic!("Cannot create complex number with complex imaginary part");
        }

        Number::Complex(Box::new(real), Box::new(imag))
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Integer(num) => write!(f, "{}", num),
            Number::Float(num) => write!(f, "{}", num),
            Number::Fraction(numer, denom) => write!(f, "{}/{}", numer, denom),
            Number::Complex(real, imag) => write!(f, "{}J{}", real, imag),
            _ => write!(f, "??number??"),
        }
    }
}

/// Registers for the virtual machine.
/// - `exp`: Expression to be evaluated.
/// - `env`: Evaluation environment.
/// - `fun`: Procedure to be applied.
/// - `argl`: List of evaluated arguments.
/// - `continue`: Place to go next.
/// - `val`: Result of evaluation.
/// - `unev`: Temporary register for expressions.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct RegisterArea {
    pub exp: TypedPointer,
    pub env: TypedPointer,
    pub fun: TypedPointer,
    pub argl: TypedPointer,
    pub cont: TypedPointer,
    pub val: TypedPointer,
    pub unev: TypedPointer,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StackArea {
    pub last: UntypedPointer,
    pub area: Vec<TypedPointer>,
}

impl Default for StackArea {
    fn default() -> Self {
        Self {
            last: 0,
            area: (0..LISP_STACK_SIZE)
                .map(|_| TypedPointer::default())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AtomTable {
    pub last: UntypedPointer,
    pub area: Vec<Atom>,
}

impl Default for AtomTable {
    fn default() -> Self {
        Self {
            last: 0,
            area: (0..ATOM_TABLE_SIZE).map(|_| Atom::default()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberTable {
    pub last: UntypedPointer,
    pub area: Vec<Number>,
    pub unused: std::collections::VecDeque<UntypedPointer>,
}

impl Default for NumberTable {
    fn default() -> Self {
        Self {
            last: 0,
            area: (0..NUMBER_TABLE_SIZE).map(|_| Number::default()).collect(),
            unused: std::collections::VecDeque::new(),
        }
    }
}

impl NumberTable {
    pub fn get_next_unsafe(&mut self) -> UntypedPointer {
        if self.unused.is_empty() {
            let ptr = self.last;
            self.last += 1;
            ptr
        } else {
            self.unused.pop_front().unwrap()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListArea {
    pub last: UntypedPointer,
    pub area: Vec<Cons>,
}

impl Default for ListArea {
    fn default() -> Self {
        Self {
            last: 0,
            area: (0..LIST_AREA_SIZE).map(|_| Cons::default()).collect(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Environment {
    pub prev: TypedPointer,
    pub data: std::collections::HashMap<TypedPointer, TypedPointer>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            prev: ConstSymbol::NIL,
            data: std::collections::HashMap::with_capacity(MAX_ENV_CAPACITY),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EnvironmentTable {
    pub last: UntypedPointer,
    pub area: Vec<Environment>,
}

impl Default for EnvironmentTable {
    fn default() -> Self {
        Self {
            last: 0,
            area: (0..ENV_TABLE_SIZE)
                .map(|_| Environment::default())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VirtualMachine {
    pub registers: RegisterArea,
    pub stack: StackArea,
    pub atoms: AtomTable,
    pub numbers: NumberTable,
    pub lists: ListArea,
    pub environments: EnvironmentTable,

    pub atom_index: Trie<String, usize>,
}
