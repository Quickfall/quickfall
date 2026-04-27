pub mod constraints;
pub mod container;
pub mod raw;

pub trait TypeSizedHIR {
    fn has_concrete_size(&self) -> bool;
}
