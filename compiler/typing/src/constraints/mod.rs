//! A constraint represents bounds that a type must follow in order to be accepted inside of a type parameter

use crate::{
    constraints::{bound::BoundConstraint, feature::FeatureConstraint},
    container::Type,
};

pub mod bound;
pub mod feature;

#[derive(Clone)]
pub struct TypeConstraintContainer {
    pub feature_constraint: FeatureConstraint,
    pub bound_constraint: Vec<BoundConstraint>,
}

impl TypeConstraintContainer {
    pub fn new() -> Self {
        TypeConstraintContainer {
            feature_constraint: FeatureConstraint::new(),
            bound_constraint: vec![],
        }
    }
}

impl TypeConstraint for TypeConstraintContainer {
    fn fits(&self, t: Type) -> bool {
        if !self.feature_constraint.fits(t.clone()) {
            return false;
        }

        for bound in &self.bound_constraint {
            if !bound.fits(t.clone()) {
                return false;
            }
        }

        return true;
    }
}

/// Describes a constraint
pub trait TypeConstraint {
    /// Checks whenever the type fits the bound constraint
    fn fits(&self, t: Type) -> bool;
}
