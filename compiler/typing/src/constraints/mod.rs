//! A constraint represents bounds that a type must follow in order to be accepted inside of a type parameter

use compiler_utils::hash::HashedString;

use crate::{
    constraints::{
        bound::BoundConstraint,
        feature::{FeatureConstraint, FeatureFlag},
    },
    container::Type,
};

pub mod bound;
pub mod feature;

#[derive(Clone)]
pub struct TypeConstraintContainer {
    pub feature_constraint: FeatureConstraint,
    pub bound_constraint: Vec<BoundConstraint>,
}

#[derive(Clone)]
pub struct TypeParameter {
    pub name: String,
    pub constraint: TypeConstraintContainer,
}

impl TypeParameter {
    pub fn new(name: String, constraint: TypeConstraintContainer) -> Self {
        TypeParameter { name, constraint }
    }
}

impl TypeConstraintContainer {
    pub fn new() -> Self {
        TypeConstraintContainer {
            feature_constraint: FeatureConstraint::new(),
            bound_constraint: vec![],
        }
    }

    pub fn has_feature_flag(&self, flag: &FeatureFlag) -> bool {
        for entry in &self.feature_constraint.entries {
            if entry.exclude {
                continue;
            }

            if &entry.flag == flag {
                return true;
            }
        }

        return false;
    }

    pub fn contain_field(&self, field: String, t: Type) -> bool {
        let hash = HashedString::new(field).hash;

        for entry in &self.bound_constraint {
            for e in &entry.members {
                if e.1 {
                    continue;
                }

                match &e.0 {
                    bound::BoundConstraintMember::Field(a, b) => {
                        if hash == a.hash && &t == b {
                            return true;
                        }
                    }

                    _ => {}
                }
            }
        }

        return false;
    }
}

impl PartialEq for TypeConstraintContainer {
    fn eq(&self, other: &Self) -> bool {
        self.feature_constraint == other.feature_constraint
            && self.bound_constraint == other.bound_constraint
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
