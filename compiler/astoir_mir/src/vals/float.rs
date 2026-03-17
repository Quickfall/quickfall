use std::fmt::Display;

use compiler_errors::{IR_CASTING_ERROR, errs::{BaseResult, base::BaseError}};

use crate::vals::base::{BaseMIRValue};

#[derive(Clone)]
pub struct MIRFloatValue {
	base: BaseMIRValue,
	pub signed: bool,
	pub exponent: usize,
	pub fraction: usize,
}

impl MIRFloatValue {
	pub fn new(base: BaseMIRValue) -> BaseResult<Self> {
		if base.vtype.base.is_floating() {
			let sizes = base.vtype.base.get_floating_size()?;

			return Ok(MIRFloatValue { base: base.clone(), exponent: sizes.0, fraction: sizes.1, signed: base.vtype.base.is_signed() })
		}

		return Err(BaseError::critical(IR_CASTING_ERROR!().to_string()))
	}
}

impl Into<BaseMIRValue> for MIRFloatValue {
	fn into(self) -> BaseMIRValue {
		return self.base;
	}
}

impl Display for MIRFloatValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "#{}", self.base.get_ssa_index())?;

		Ok(())
	}
}