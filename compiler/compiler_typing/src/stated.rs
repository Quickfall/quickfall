//! Declarations for stated types.

use compiler_errors::{IR_TRANSMUTATION, errs::{BaseResult, base::BaseError}};

use crate::tree::Type;

/// Represents a variable type. Can either be inferred or fully enforced
#[derive(Debug, Clone)]
pub struct StatedType {
	pub raw_type: Type,
	pub inferred: bool
}

impl StatedType {
	pub fn make_inferred(raw: Type) -> Self {
		StatedType { raw_type: raw, inferred: true }
	}

	pub fn make_enforced(raw: Type) -> Self {
		StatedType { raw_type: raw, inferred: false }
	}

	/// Infers the contained type based on the given required type.
	/// 
	/// # Usage
	/// This function should be used each time the type is checked
	/// 
	/// # Transmutation & Inference
	/// The role of this function is to handle type inference. Whenever a type that is compatible with the inferred type is passed, the enforced type will become the required one. 
	/// This will be used with things like constants to avoid using the numerical type suffix (eg: `_s32`)
	pub fn infer(&mut self, raw_other: &Type) -> BaseResult<()> {
		if !self.raw_type.can_transmute(raw_other) {
			return Err(BaseError::err(IR_TRANSMUTATION!().to_string()));
		}

		if self.inferred {
			self.raw_type = raw_other.clone();
			self.inferred = false;
		}

		return Ok(())
	}
}