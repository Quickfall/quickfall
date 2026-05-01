//! The definitions of the raw types

use std::hash::Hash;

use compiler_utils::hash::HashedString;

use crate::{
    FieldMethodType, TypeParameterContaining, TypeSizedHIR, TypeTransmutation,
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
#[derive(Clone, Hash)]
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

    pub fn get_raw_require_type_parameters(&self) -> usize {
        match self {
            Self::UnsizedInteger(_) => 1,
            Self::UnsizedFloating(_) => 1,
            Self::UnsizedFixedPoint(_) => 2,

            _ => 0,
        }
    }

    pub fn lower(self, size_params: Vec<usize>) -> RawType {
        match self {
            Self::UnsizedInteger(signed) => Self::Integer(signed, size_params[0]),
            Self::UnsizedFloating(signed) => Self::Floating(signed, size_params[0]),
            Self::UnsizedFixedPoint(signed) => {
                Self::FixedPoint(signed, size_params[0], size_params[1])
            }

            _ => self,
        }
    }
}

impl TypeParameterContaining for RawType {
    fn get_param_types(&self) -> Vec<String> {
        match self {
            Self::Struct(container) => container.get_param_types(),
            Self::Enum(container) => container.get_param_types(),
            Self::EnumChild(container) => container.get_param_types(),

            _ => vec![],
        }
    }

    fn append_type_parameter(
        &mut self,
        param: String,
        constraint: crate::constraints::TypeConstraintContainer,
    ) {
        match self {
            Self::Struct(container) => container.append_type_parameter(param, constraint),
            Self::Enum(container) => container.append_type_parameter(param, constraint),

            _ => panic!("Cannot add type parameter here"),
        }
    }

    fn has_param_type(&self, param: String) -> bool {
        match self {
            Self::Struct(container) => container.has_param_type(param),
            Self::Enum(container) => container.has_param_type(param),
            Self::EnumChild(container) => container.has_param_type(param),

            _ => false,
        }
    }

    fn get_type_param_type(&self, param: String) -> Type {
        match self {
            Self::Struct(container) => container.get_type_param_type(param),
            Self::Enum(container) => container.get_type_param_type(param),
            Self::EnumChild(container) => container.get_type_param_type(param),

            _ => panic!("cannot get param type from type"),
        }
    }
}

impl Hash for RawType {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        match self {
            Self::Integer(a, b) => {
                h.write_usize(1);
                h.write_u8(*a as u8);
                h.write_usize(*b);
            }

            Self::Floating(a, b) => {
                h.write_usize(2);
                h.write_u8(*a as u8);
                h.write_usize(*b);
            }

            Self::FixedPoint(a, b, c) => {
                h.write_usize(3);
                h.write_u8(*a as u8);
                h.write_usize(*b);
                h.write_usize(*c);
            }

            Self::StaticString => {
                h.write_usize(4);
            }

            Self::AnyPointer => {
                h.write_usize(5);
            }

            Self::Boolean => {
                h.write_usize(6);
            }

            Self::Struct(container) => {
                h.write_usize(7);
                container.name.hash(h);
            }

            Self::Enum(container) => {
                h.write_usize(8);
                container.name.val.hash(h);
            }

            Self::EnumChild(container) => {
                h.write_usize(9);
                h.write_usize(container.child_index);
                container.parent.name.val.hash(h);
            }

            Self::UnsizedInteger(signed) => {
                h.write_usize(10);
                h.write_u8(*signed as u8);
            }

            Self::UnsizedFloating(signed) => {
                h.write_usize(11);
                h.write_u8(*signed as u8);
            }

            Self::UnsizedFixedPoint(signed) => {
                h.write_usize(12);
                h.write_u8(*signed as u8);
            }
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
    fn get_fields(&self) -> Vec<(compiler_utils::hash::HashedString, Type)> {
        let mut fields = vec![];
        match self {
            Self::Struct(container) => {
                for key in &container.fields.keys {
                    fields.push((
                        HashedString::new(key.clone()),
                        container.fields.get_as_clone(key.clone()).unwrap().t,
                    ))
                }
            }

            Self::EnumChild(container) => {
                for key in &container.fields.keys {
                    fields.push((
                        HashedString::new(key.clone()),
                        container.fields.get_as_clone(key.clone()).unwrap().t,
                    ))
                }
            }

            _ => {}
        }

        fields
    }

    fn get_methods(&self) -> Vec<(HashedString, Vec<Type>, Option<Type>)> {
        let mut methods = vec![];

        match self {
            Self::Struct(container) => {
                for key in &container.functions.keys {
                    let func = container.functions.get_as_clone(key.clone()).unwrap();

                    methods.push((
                        func.name.clone(),
                        func.arguments.clone(),
                        func.return_type.clone(),
                    ));
                }
            }

            Self::Enum(container) => {
                for key in &container.functions.keys {
                    let func = container.functions.get_as_clone(key.clone()).unwrap();

                    methods.push((
                        func.name.clone(),
                        func.arguments.clone(),
                        func.return_type.clone(),
                    ));
                }
            }

            Self::EnumChild(container) => {
                for key in &container.parent.functions.keys {
                    let func = container
                        .parent
                        .functions
                        .get_as_clone(key.clone())
                        .unwrap();

                    methods.push((
                        func.name.clone(),
                        func.arguments.clone(),
                        func.return_type.clone(),
                    ));
                }
            }

            _ => {}
        }

        methods
    }

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
