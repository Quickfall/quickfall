//! Feature constraints are a way to restrict a type to if it has or has not a given feature (eg: is an integer type)
//! Here's a list of feature constraints:
//! - !numeric
//! - !signed
//! - !integer
//! - !floating
//! - !fixed
//! - !noninteger
//! - !cpusupported
//! - !stringlike
//! - !static
//! - !mathoperations
//! - !struct
//!
//! Additionally the exclude variant can be used by replacing the ! by a #.
//!
//! # Examples
//! ```
//! struct test<A: !numeric !mathoperations> {
//! 	A myFirstNumber // Here number will always be a numeric type WITH mathematical operations instead of custom operators
//! }
//! ```
//!

use std::fmt::Display;

use crate::constraints::TypeConstraint;

#[derive(Clone)]
pub enum FeatureFlag {
    /// Is the type a numeric type (holds a number directly)
    Numeric,

    /// Is the type a signed numeric type
    Signed,

    /// Is the type an integer
    Integer,

    /// Is the type a floating point number
    Floating,

    /// Is the type a fixed point number
    Fixed,

    /// Is the type a non integer number (floating + fixed)
    NonInteger,

    /// Is the type natively supported by the CPU
    CpuSupported,

    /// Is the type like a string
    StringLike,

    /// Is the type static
    Static,

    /// Does the type use native math operations instead of custom operators
    MathOperations,

    /// Is the type a structure like type
    Struct,
}

/// An entry for the feature flag container
#[derive(Clone)]
pub struct FeatureFlagEntry {
    pub flag: FeatureFlag,
    pub exclude: bool,
}

#[derive(Clone)]
pub struct FeatureConstraint {
    pub entries: Vec<FeatureFlagEntry>,
}

impl FeatureConstraint {
    pub fn new() -> Self {
        FeatureConstraint { entries: vec![] }
    }

    pub fn append(&mut self, entry: FeatureFlagEntry) {
        self.entries.push(entry);
    }
}

impl TypeConstraint for FeatureConstraint {
    fn fits(&self, t: crate::container::Type) -> bool {
        for entry in &self.entries {
            let b = t.has_feature_flag(&entry.flag);

            if b == entry.exclude {
                return false;
            }
        }

        return true;
    }
}

impl Display for FeatureFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Numeric => "numeric",
            Self::Signed => "signed",
            Self::Integer => "integer",
            Self::Floating => "floating",
            Self::Fixed => "fixed",
            Self::NonInteger => "noninteger",
            Self::CpuSupported => "cpusupported",
            Self::StringLike => "stringlike",
            Self::Static => "static",
            Self::MathOperations => "mathoperations",
            Self::Struct => "struct",
        };

        write!(f, "{}", s)?;
        Ok(())
    }
}

impl Display for FeatureFlagEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.exclude {
            write!(f, "#")?;
        } else {
            write!(f, "!")?;
        }

        write!(f, "{}", self.flag)?;

        Ok(())
    }
}

impl Display for FeatureConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.entries {
            write!(f, "{} ", entry)?;
        }

        Ok(())
    }
}
