use compiler_utils::hash::HashedString;

use crate::{constraints::TypeConstraintContainer, container::Type};

pub mod constraints;
pub mod container;
pub mod enums;
pub mod fmt;
pub mod raw;
pub mod structs;

pub trait TypeSizedHIR {
    fn has_concrete_size(&self) -> bool;
}

pub trait TypeParameterContaining {
    /// Gets the type for the given type parameter
    fn get_type_param_type(&self, param: String) -> Type;

    /// Checks if the given type parameter exists in this container
    fn has_param_type(&self, param: String) -> bool;

    /// Appends a new type parameter into the given container.
    fn append_type_parameter(&mut self, param: String, constraint: TypeConstraintContainer);

    fn get_param_types(&self) -> Vec<String>;
}

pub trait FieldMethodType {
    fn has_field(&self, name: String, t: Type) -> bool;
    fn has_method(&self, name: String, return_type: Option<Type>, arguments: Vec<Type>) -> bool;
    fn get_fields(&self) -> Vec<(HashedString, Type)>;
    fn get_methods(&self) -> Vec<(HashedString, Vec<Type>, Option<Type>)>;
}
