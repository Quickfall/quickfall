//! # Trait bounds
//! Trait bounds are used to select types with given features. These traits can be represented with a `!` prefix. They can also be used to exclude certain types with the given features.
//! 
//! Here's a list of every trait bound with their corresponding feature:
//! - `!numeric`: Is a numeric type
//! - `!signed`: Is a signed numeric type
//! - `!integer`: Is an integer type
//! - `!floating`: Is an floating type
//! - `!fixed`: Is a fixed point type
//! - `!noninteger`: Is a non integer type like floats or fixed point numbers
//! - `!cpusupported`: Is the type supported by the CPU.
//! - `!stringlike`: Is the type a string
//! - `!static`: Is the type supposed to be statically stored
//! 
//! # Examples
//! ```
//! struct test<A: !numeric ~!cpusupported> {
//! 	// A can now only be a numeric type and not supported by the CPU
//! }
//! ```

use std::fmt::Display;

use compiler_utils::hash;
use diagnostics::{MaybeDiagnostic, builders::make_bound_trait};

use crate::{storage::TypeStorage, tree::Type};

pub const TRAIT_NUMERIC: u64 = hash!("numeric");
pub const TRAIT_SIGNED: u64 = hash!("signed");
pub const TRAIT_INTEGER: u64 = hash!("integer");
pub const TRAIT_FLOATING: u64 = hash!("floating");
pub const TRAIT_FIXED: u64 = hash!("fixed");
pub const TRAIT_NON_INTEGER: u64 = hash!("noninteger");
pub const TRAIT_CPU_SUPPORTED: u64 = hash!("cpusupported");
pub const TRAIT_STRING: u64 = hash!("stringlike");
pub const TRAIT_STATIC: u64 = hash!("static");

#[derive(Clone)]
pub enum Trait {
	Numeric,
	Signed,
	Integer,
	Floating,
	Fixed,
	NonInteger,
	CpuSupported,
	String,
	Static
}

impl Display for Trait {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Trait::Numeric => "numeric",
			Trait::Signed => "signed",
			Trait::Integer => "integer",
			Trait::Floating => "floating",
			Trait::Fixed => "fixed",
			Trait::NonInteger => "noninteger",
			Trait::CpuSupported => "cpusupported",
			Trait::String => "stringlike",
			Trait::Static => "static"
		};

		write!(f, "!{}", s)
	}
}

pub enum TraitBoundMember {
	/// Selects a trait to require it
	Select(Trait),

	/// Excludes a trait. Types having this trait will not be accepted
	Exclude(Trait)
}

/// Represents the actual trait bound. Is used to make sure that the type is compatible
pub struct TraitBound {
	pub members: Vec<TraitBoundMember>
}

impl TraitBound {
	pub fn check(&self, t: &Type, storage: &TypeStorage) -> MaybeDiagnostic {
		for member in &self.members {
			match member {
				TraitBoundMember::Select(tt) => {
					if !t.as_generic(storage).has_trait(tt.clone(), t) {
						return Err(make_bound_trait(tt, t).into())
					}
				},

				TraitBoundMember::Exclude(tt) => {
					if t.as_generic(storage).has_trait(tt.clone(), t) {
						return Err(make_bound_trait(&format!("~{}", tt), t).into())
					}
				}
			}
		}

		return Ok(());
	}
}