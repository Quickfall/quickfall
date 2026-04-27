//! The definitions of the raw types

use crate::TypeSizedHIR;

/// Represents a real raw type. A raw type is a concrete type that can be simply lowered.
pub enum RawType {
    Integer(bool, usize),
    Floating(bool, usize),
    FixedPoint(bool, usize, usize),
    ExactPoint(bool, usize, usize),

    StaticString,
    AnyPointer,

    Boolean,

    UnsizedInteger(bool),
    UnsizedFloating(bool),
    UnsizedFixedPoint(bool),
    UnsizedExactPoint(bool),
}

impl TypeSizedHIR for RawType {
    fn has_concrete_size(&self) -> bool {
        match self {
            Self::UnsizedInteger(_) => false,
            Self::UnsizedFloating(_) => false,
            Self::UnsizedFixedPoint(_) => false,
            Self::UnsizedExactPoint(_) => false,

            _ => true,
        }
    }
}
