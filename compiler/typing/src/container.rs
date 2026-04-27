//! Used to represent type, uses a inner-node method to store each modifier from outside to inside.
//! Outside being the most outside modifier and inside being the raw type
//!
//!
//! # Example
//! For example the type `s32*[][32]` would be stored as:
//! ```
//! - Array(size: 32)
//! -	- Pointer(array pointer: true)
//! -	-	- Raw (signed 32 bit integer)
//! ```

use crate::{
    constraints::{TypeConstraintContainer, feature::FeatureFlag},
    raw::InformationRawType,
};

/// The main container for types
#[derive(Clone)]
pub enum Type {
    /// Represents an array of a given size and of type of the inner type container within
    Array { size: usize, inner: Box<Type> },

    /// Represents a pointer of the given inner type that is potentially an pointer-based array
    Pointer { is_array: bool, inner: Box<Type> },

    /// Represents a real raw type. A raw type is a concrete type that can be simply lowered.
    Raw { raw: InformationRawType },

    /// Represents a generic type argument.
    /// A special kind of argument that passes a type parameter type as a type.
    /// It must follow the constraints given by the type parameter
    GenericTypeParam {
        constraints: TypeConstraintContainer,
    },
}

impl Type {
    /// Is the type real and doesn't rely on generic type arguments.
    pub fn is_real(&self) -> bool {
        match self {
            Self::Array { size: _, inner } => inner.is_real(),
            Self::Pointer { is_array: _, inner } => inner.is_real(),
            Self::Raw { .. } => true,
            Self::GenericTypeParam { .. } => false,
        }
    }

    pub fn has_feature_flag(&self, _flag: &FeatureFlag) -> bool {
        false
    }

    pub fn has_field(&self, _name: u64, _t: Type) -> bool {
        false
    }

    pub fn has_method(&self, _name: u64, _returntype: Type, _args: Vec<Type>) -> bool {
        false
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Array { size, inner },
                Self::Array {
                    size: size2,
                    inner: inner2,
                },
            ) => size == size2 && inner == inner2,

            (
                Self::Pointer { is_array, inner },
                Self::Pointer {
                    is_array: is_array2,
                    inner: inner2,
                },
            ) => is_array == is_array2 && inner == inner2,

            (Self::Raw { raw }, Self::Raw { raw: raw2 }) => raw == raw2,
            (
                Self::GenericTypeParam { constraints },
                Self::GenericTypeParam {
                    constraints: constraints2,
                },
            ) => constraints == constraints2,

            _ => false,
        }
    }
}
