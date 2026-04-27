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
    FieldMethodType,
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
        name: String,
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

    pub fn get_last(&self) -> Type {
        match self {
            Self::Array { size: _, inner } => inner.get_last(),
            Self::Pointer { is_array: _, inner } => inner.get_last(),
            Self::Raw { .. } => self.clone(),
            Self::GenericTypeParam { .. } => self.clone(),
        }
    }

    /// # Warn
    /// This will panic if not checked properly
    pub fn get_raw(&self) -> InformationRawType {
        if let Self::Raw { raw } = self.get_last() {
            return raw;
        }

        panic!("last node was not a raw but used get_raw")
    }

    /// # Warn
    /// This will panic if not checked properly
    pub fn get_next(&self) -> Type {
        match self {
            Self::Array { size: _, inner } => *inner.clone(),
            Self::Pointer { is_array: _, inner } => *inner.clone(),
            _ => panic!("tried using get_next on the last element"),
        }
    }

    pub fn has_feature_flag(&self, flag: &FeatureFlag) -> bool {
        match self {
            Self::Raw { raw } => raw.t.has_feature_flag(flag, &raw),
            Self::GenericTypeParam {
                name: _,
                constraints,
            } => constraints.has_feature_flag(flag),
            _ => {
                if flag == &FeatureFlag::CpuSupported || flag == &FeatureFlag::Static {
                    self.get_next().has_feature_flag(flag)
                } else {
                    false
                }
            }
        }
    }
}

impl FieldMethodType for Type {
    fn has_field(&self, name: String, t: Type) -> bool {
        match self {
            Self::Raw { .. } => false, // TODO: add raw impl
            Self::GenericTypeParam {
                constraints,
                name: _,
            } => constraints.contain_field(name, t),

            Self::Pointer { is_array: _, inner } => inner.has_field(name, t),

            _ => false,
        }
    }

    fn has_method(&self, name: String, returntype: Option<Type>, args: Vec<Type>) -> bool {
        match self {
            Self::Raw { .. } => false, // TODO: add raw impl
            Self::GenericTypeParam {
                constraints,
                name: _,
            } => constraints.contain_function(name, returntype, args),

            Self::Pointer { is_array: _, inner } => inner.has_method(name, returntype, args),

            _ => false,
        }
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
                Self::GenericTypeParam { name, constraints },
                Self::GenericTypeParam {
                    name: name2,
                    constraints: constraints2,
                },
            ) => name == name2 && constraints == constraints2,

            _ => false,
        }
    }
}
