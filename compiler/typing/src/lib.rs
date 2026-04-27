use crate::container::Type;

pub mod constraints;
pub mod container;
pub mod raw;
pub mod structs;

pub trait TypeSizedHIR {
    fn has_concrete_size(&self) -> bool;
}

pub trait TypeParameterContaining {
    /// Gets the type for the given type parameter
    fn get_type_param_type(&self, param: String) -> Type;
    fn has_param_type(&self, param: String) -> bool;
}
