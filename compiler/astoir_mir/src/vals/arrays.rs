use std::fmt::Display;

use compiler_errors::{IR_CASTING_ERROR, errs::{BaseResult, base::BaseError}};
use compiler_typing::{tree::Type};

use crate::vals::base::BaseMIRValue;

pub struct MIRArrayValue {
	pub base: BaseMIRValue,
	pub size: usize
}

impl MIRArrayValue {
	pub fn new(base: BaseMIRValue) -> BaseResult<Self> {
		if let Type::Array(size, _) = base.vtype.clone() {
			return Ok(MIRArrayValue { base, size });
		}

		return Err(BaseError::err(IR_CASTING_ERROR!().to_string()))
	}
}

impl Into<BaseMIRValue> for MIRArrayValue {
	fn into(self) -> BaseMIRValue {
		return self.base;
	}
}

impl Display for MIRArrayValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "#{}", self.base.get_ssa_index())?;

		Ok(())
	}
}
