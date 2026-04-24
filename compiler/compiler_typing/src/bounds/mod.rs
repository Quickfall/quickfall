//! Bounds are a feature in Quickfall allowing you to restrict what kind of types you want within a generic.
//! There are two kinds of bounds:
//! - Trait bound: Mostly internal, is used to cleanly handle different types
//! - Normal bound: Allows to select which kind of type you want

use std::fmt::Display;

use crate::bounds::{normal::NormalBound, traits::TraitBound};

pub mod normal;
pub mod traits;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TypeSelector {
    Bound(NormalBound),
    Trait(TraitBound),
}

impl Display for TypeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bound(bound) => write!(f, "{}", bound)?,
            Self::Trait(t) => write!(f, "{}", t)?,
        };

        Ok(())
    }
}
