//! The definitions of the raw types

use crate::{
    FieldMethodType, TypeSizedHIR,
    constraints::feature::FeatureFlag,
    container::Type,
    enums::{ChildEnumContainer, ParentEnumContainer},
    structs::StructContainer,
};

/// Represents a real raw type. A raw type is a concrete type that can be simply lowered.
#[derive(Clone)]
pub enum RawType {
    Integer(bool, usize),
    Floating(bool, usize),
    FixedPoint(bool, usize, usize),

    StaticString,
    AnyPointer,

    Boolean,

    Struct(StructContainer),
    Enum(ParentEnumContainer),
    EnumChild(ChildEnumContainer),

    UnsizedInteger(bool),
    UnsizedFloating(bool),
    UnsizedFixedPoint(bool),
}

/// A RawType that stores additional information such as size parameters and type parameters
#[derive(Clone)]
pub struct InformationRawType {
    pub t: RawType,

    pub sizes: Vec<usize>,
    pub type_parameters: Vec<Box<Type>>,
}

impl RawType {
    pub fn is_numeric(&self) -> bool {
        match self {
            Self::Integer(_, _) => true,
            Self::Floating(_, _) => true,
            Self::FixedPoint(_, _, _) => true,

            Self::UnsizedInteger(_) => true,
            Self::UnsizedFloating(_) => true,
            Self::UnsizedFixedPoint(_) => true,
            _ => false,
        }
    }

    pub fn is_signed(&self) -> bool {
        match self {
            Self::Integer(signed, _) => *signed,
            Self::Floating(signed, _) => *signed,
            Self::FixedPoint(signed, _, _) => *signed,
            Self::UnsizedInteger(signed) => *signed,
            Self::UnsizedFloating(signed) => *signed,
            Self::UnsizedFixedPoint(signed) => *signed,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Self::Integer(_, _) => true,
            Self::UnsizedInteger(_) => true,
            _ => false,
        }
    }

    pub fn is_floating(&self) -> bool {
        match self {
            Self::Floating(_, _) => true,
            Self::UnsizedFloating(_) => true,
            _ => false,
        }
    }

    pub fn is_fixed(&self) -> bool {
        match self {
            Self::FixedPoint(_, _, _) => true,
            Self::UnsizedFixedPoint(_) => true,

            _ => false,
        }
    }

    pub fn is_noninteger(&self) -> bool {
        self.is_floating() || self.is_fixed()
    }

    pub fn is_cpu_supported(&self, information: &InformationRawType) -> bool {
        match self {
            Self::Floating(_, size) => {
                let log = size.ilog2();

                return (log >= 4 && log <= 7) || *size == 80;
            }

            Self::UnsizedFloating(_) => {
                let size = information.sizes[0];

                let log = size.ilog2();

                return (log >= 4 && log <= 7) || size == 80;
            }

            _ => true,
        }
    }

    pub fn is_stringlike(&self) -> bool {
        match self {
            Self::StaticString => true,

            _ => false,
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Self::StaticString => true,
            _ => false,
        }
    }

    pub fn has_math_operations(&self) -> bool {
        self.is_numeric()
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Self::Struct(_) => true,
            Self::Enum(_) => true,
            Self::EnumChild(_) => true,

            _ => false,
        }
    }

    pub fn has_feature_flag(&self, flag: &FeatureFlag, information: &InformationRawType) -> bool {
        match flag {
            FeatureFlag::Numeric => self.is_numeric(),
            FeatureFlag::Signed => self.is_signed(),
            FeatureFlag::Integer => self.is_integer(),
            FeatureFlag::Floating => self.is_floating(),
            FeatureFlag::Fixed => self.is_fixed(),
            FeatureFlag::NonInteger => self.is_noninteger(),
            FeatureFlag::CpuSupported => self.is_cpu_supported(information),
            FeatureFlag::StringLike => self.is_stringlike(),
            FeatureFlag::Static => self.is_static(),
            FeatureFlag::Struct => self.is_struct(),
            FeatureFlag::MathOperations => self.has_math_operations(),
        }
    }
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

impl FieldMethodType for RawType {
    fn has_field(&self, name: String, t: Type) -> bool {
        match self {
            Self::Struct(container) => {
                container.fields.has_key(&name) && container.fields.get(&name).unwrap().t == t
            }

            Self::EnumChild(container) => {
                container.fields.has_key(&name) && container.fields.get(&name).unwrap().t == t
            }

            _ => false,
        }
    }

    fn has_method(&self, name: String, return_type: Option<Type>, arguments: Vec<Type>) -> bool {
        match self {
            Self::Struct(container) => {
                if !container.functions.has_key(&name) {
                    return false;
                }

                let method = container.functions.get(&name).unwrap();

                return method.arguments == arguments && method.return_type == return_type;
            }

            Self::Enum(container) => {
                if !container.functions.has_key(&name) {
                    return false;
                }

                let method = container.functions.get(&name).unwrap();

                return method.arguments == arguments && method.return_type == return_type;
            }

            Self::EnumChild(container) => {
                if !container.parent.functions.has_key(&name) {
                    return false;
                }

                let method = container.parent.functions.get(&name).unwrap();

                return method.arguments == arguments && method.return_type == return_type;
            }

            _ => false,
        }
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

            (Self::Boolean, Self::Boolean) => true,
            (Self::StaticString, Self::StaticString) => true,
            (Self::AnyPointer, Self::StaticString) => true,
            (Self::StaticString, Self::AnyPointer) => true,
            (Self::AnyPointer, Self::AnyPointer) => true,

            (Self::UnsizedInteger(a), Self::UnsizedInteger(b)) => *a == *b,
            (Self::UnsizedFloating(a), Self::UnsizedFloating(b)) => a == b,
            (Self::UnsizedFixedPoint(a), Self::UnsizedFixedPoint(b)) => a == b,

            _ => false,
        }
    }
}

impl Eq for RawType {}
