//! A constraint represents bounds that a type must follow in order to be accepted inside of a type parameter

use crate::constraints::feature::FeatureFlag;

pub mod bound;
pub mod feature;

pub enum TypeConstraintEntry {
    Feature { exclude: bool, feature: FeatureFlag },
}
