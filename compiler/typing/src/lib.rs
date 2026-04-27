use crate::{constraints::TypeConstraintContainer, container::Type};

pub mod constraints;
pub mod container;
pub mod enums;
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
}
