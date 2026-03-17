use std::fmt::Display;

use astoir_typing::base::BaseType;
use compiler_errors::{IR_CASTING_ERROR, errs::{BaseResult, base::BaseError}};

use crate::vals::base::{BaseMIRValue};

#[derive(Clone)]
pub struct MIRPointerValue {
	base: BaseMIRValue
}

impl MIRPointerValue {
	pub fn new(base: BaseMIRValue) -> BaseResult<Self> {
		if let BaseType::Pointer = &base.vtype.base {
			return Ok(MIRPointerValue { base: base.clone() })
		}

		return Err(BaseError::critical(IR_CASTING_ERROR!().to_string()))
	}
}

impl Into<BaseMIRValue> for MIRPointerValue {
	fn into(self) -> BaseMIRValue {
		return self.base;
	}
}

impl Display for MIRPointerValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "#{}", self.base.get_ssa_index())?;

		Ok(())
	}
}