use std::fmt::Display;

use compiler_errors::{IR_CASTING_ERROR, errs::{BaseResult, base::BaseError}};

use crate::vals::base::{BaseMIRValue};

#[derive(Clone)]
pub struct MIRIntValue {
	pub base: BaseMIRValue,
	pub signed: bool,
	pub size: usize,
}

impl MIRIntValue {
	pub fn new(base: BaseMIRValue) -> BaseResult<Self> {
		if base.vtype.base.is_integer() | base.vtype.base.is_bool() {
			return Ok(MIRIntValue { base: base.clone(), size: base.vtype.base.get_size()?, signed: base.vtype.base.is_signed() })
		}

		return Err(BaseError::critical(IR_CASTING_ERROR!().to_string()))
	}
}

impl Into<BaseMIRValue> for MIRIntValue {
	fn into(self) -> BaseMIRValue {
		return self.base;
	}
}

impl Display for MIRIntValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "#{}", self.base.get_ssa_index())?;

		Ok(())
	}
}