//! The definitions of the raw types

use crate::{TypeSizedHIR, container::Type};

/// Represents a real raw type. A raw type is a concrete type that can be simply lowered.
#[derive(Clone)]
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

/// A RawType that stores additional information such as size parameters and type parameters
#[derive(Clone)]
pub struct InformationRawType {
    pub t: RawType,

    pub sizes: Vec<usize>,
    pub type_parameters: Vec<Box<Type>>,
}

impl InformationRawType {
    pub fn new(t: RawType) -> Self {
        InformationRawType {
            t,
            sizes: vec![],
            type_parameters: vec![],
        }
    }
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

impl PartialEq for InformationRawType {
    fn eq(&self, other: &Self) -> bool {
        self.sizes == other.sizes
            && self.t == other.t
            && self.type_parameters == other.type_parameters
    }
}

impl PartialEq for RawType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(a, b), RawType::Integer(c, d)) => *a == *c && *b == *d,
            (Self::Floating(a, b), RawType::Floating(c, d)) => *a == *c && *b == *d,
            (Self::FixedPoint(a, b, c), RawType::FixedPoint(d, e, f)) => {
                *a == *d && *b == *e && *c == *f
            }
            (Self::ExactPoint(a, b, c), RawType::ExactPoint(d, e, f)) => {
                *a == *d && *b == *e && *c == *f
            }

            (Self::Boolean, Self::Boolean) => true,
            (Self::StaticString, Self::StaticString) => true,
            (Self::AnyPointer, Self::StaticString) => true,
            (Self::StaticString, Self::AnyPointer) => true,
            (Self::AnyPointer, Self::AnyPointer) => true,

            (Self::UnsizedInteger(a), Self::UnsizedInteger(b)) => *a == *b,
            (Self::UnsizedFloating(a), Self::UnsizedFloating(b)) => a == b,
            (Self::UnsizedFixedPoint(a), Self::UnsizedFixedPoint(b)) => a == b,
            (Self::UnsizedExactPoint(a), Self::UnsizedExactPoint(b)) => a == b,

            _ => false,
        }
    }
}

impl Eq for RawType {}
