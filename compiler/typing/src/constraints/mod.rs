//! A constraint represents bounds that a type must follow in order to be accepted inside of a type parameter

use crate::{constraints::feature::FeatureFlag, container::Type};

pub mod bound;
pub mod feature;

pub enum TypeConstraintEntry {
    Feature { exclude: bool, feature: FeatureFlag },
}

/// Describes a constraint
pub trait TypeConstraint {
    /// Checks whenever the type fits the bound constraint
    fn fits(&self, t: Type) -> bool;
}
