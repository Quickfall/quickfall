//! Bounds are a feature in Quickfall allowing you to restrict what kind of types you want within a generic.
//! There are two kinds of bounds:
//! - Trait bound: Mostly internal, is used to cleanly handle different types
//! - Normal bound: Allows to select which kind of type you want

pub mod normal;
pub mod traits;
