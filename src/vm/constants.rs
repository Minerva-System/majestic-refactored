use super::types::*;

#[non_exhaustive]
pub struct ConstSymbol;

impl ConstSymbol {
    pub const NIL: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 0,
    };
    pub const T: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 1,
    };
    pub const PRIM: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 2,
    };
    pub const LIT: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 3,
    };
    pub const CLOSURE: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 4,
    };
    pub const ERROR: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 5,
    };
    pub const FN: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 6,
    };
    pub const AMPERSAND: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 7,
    };
    pub const APPLY: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 8,
    };
    pub const MACRO: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 9,
    };
    pub const MAC: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 10,
    };
    pub const QUOTE: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 11,
    };
    pub const UNQUOTE: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 12,
    };
    pub const UNQUOTE_SPLICE: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 13,
    };
    pub const QUASIQUOTE: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 14,
    };
    pub const DO: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 15,
    };
    pub const INTEGER: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 16,
    };
    pub const FLOAT: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 17,
    };
    pub const FRACTION: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 18,
    };
    pub const COMPLEX: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 19,
    };
    pub const VECTOR: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 20,
    };
    pub const SETQ: TypedPointer = TypedPointer {
        tag: DataType::Atom,
        value: 21,
    };

    // Built-in literals, used on evaluator
    pub const DONE: TypedPointer = TypedPointer {
        tag: DataType::BuiltInLiteral,
        value: 0,
    };
    pub const EVAL_ARGS: TypedPointer = TypedPointer {
        tag: DataType::BuiltInLiteral,
        value: 1,
    };
    pub const ACCUMULATE_ARG: TypedPointer = TypedPointer {
        tag: DataType::BuiltInLiteral,
        value: 2,
    };
    pub const ACCUMULATE_LAST_ARG: TypedPointer = TypedPointer {
        tag: DataType::BuiltInLiteral,
        value: 3,
    };
    pub const EVAL_ASSIGN: TypedPointer = TypedPointer {
        tag: DataType::BuiltInLiteral,
        value: 4,
    };
}
