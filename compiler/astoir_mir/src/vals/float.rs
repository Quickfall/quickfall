use std::fmt::Display;

use compiler_errors::{IR_CASTING_ERROR, errs::{BaseResult, base::BaseError}};
use compiler_typing::raw::RawType;

use crate::vals::base::{BaseMIRValue};

#[derive(Clone)]
pub struct MIRFloatValue {
	base: BaseMIRValue,
	pub signed: bool,
	pub size: usize
}

impl MIRFloatValue {
	pub fn new(base: BaseMIRValue) -> BaseResult<Self> {
		if let RawType::Floating(size, signed) = base.vtype.as_generic_lowered()? {

			return Ok(MIRFloatValue { base: base.clone(), size, signed })
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