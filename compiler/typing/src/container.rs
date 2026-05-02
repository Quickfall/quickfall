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

use std::hash::Hash;

use crate::{
    FieldMethodType, TypeTransmutation,
    constraints::{TypeConstraintContainer, feature::FeatureFlag},
    raw::{InformationRawType, RawType},
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
    pub fn make_raw(t: RawType) -> Type {
        Type::Raw {
            raw: InformationRawType::new(t),
        }
    }

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

    pub fn is_ptr(&self) -> bool {
        match self {
            Self::Pointer { .. } => true,
            Self::Raw { raw } => raw.t == RawType::AnyPointer,
            _ => self.get_last().is_ptr(),
        }
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

    pub fn is_array(&self) -> bool {
        match self {
            Self::Array { .. } => true,
            _ => false,
        }
    }
}

impl FieldMethodType for Type {
    fn has_field(&self, name: String, t: Type) -> bool {
        match self {
            Self::Raw { raw } => raw.t.has_field(name, t),
            Self::GenericTypeParam {
                constraints,
                name: _,
            } => constraints.contain_field(name, t),

            Self::Pointer { is_array: _, inner } => inner.has_field(name, t),

            _ => false,
        }
    }

    fn get_fields(&self) -> Vec<(compiler_utils::hash::HashedString, Type)> {
        match self {
            Self::Raw { raw } => raw.t.get_fields(),
            Self::GenericTypeParam {
                constraints,
                name: _,
            } => constraints.get_fields(),
            Self::Pointer { is_array: _, inner } => inner.get_fields(),

            _ => vec![],
        }
    }

    fn get_methods(&self) -> Vec<(compiler_utils::hash::HashedString, Vec<Type>, Option<Type>)> {
        match self {
            Self::Raw { raw } => raw.t.get_methods(),
            Self::GenericTypeParam {
                constraints,
                name: _,
            } => constraints.get_functions(),
            Self::Pointer { is_array: _, inner } => inner.get_methods(),

            _ => vec![],
        }
    }

    fn has_method(&self, name: String, return_type: Option<Type>, arguments: Vec<Type>) -> bool {
        match self {
            Self::Raw { raw } => raw.t.has_method(name, return_type, arguments),
            Self::GenericTypeParam {
                constraints,
                name: _,
            } => constraints.contain_function(name, return_type, arguments),

            Self::Pointer { is_array: _, inner } => inner.has_method(name, return_type, arguments),

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

impl Eq for Type {}

impl Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        match self {
            Self::Raw { raw } => {
                h.write_usize(0);
                raw.hash(h);
            }

            Self::GenericTypeParam {
                constraints: _,
                name,
            } => {
                h.write_usize(1);
                name.hash(h);
            }

            Self::Array { size, inner } => {
                h.write_usize(2);
                h.write_usize(*size);
                inner.hash(h);
            }

            Self::Pointer { is_array, inner } => {
                h.write_usize(3);
                h.write_u8(*is_array as u8);
                inner.hash(h);
            }
        }
    }
}
