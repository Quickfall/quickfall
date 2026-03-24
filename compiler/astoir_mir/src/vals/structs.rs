use std::fmt::Display;

use compiler_errors::{IR_CASTING_ERROR, errs::{BaseResult, base::BaseError}};
use compiler_typing::raw::RawType;

use crate::vals::base::BaseMIRValue;

pub struct MIRStructValue {
	pub base: BaseMIRValue,
	pub t: RawType,
}

impl MIRStructValue {
	pub fn new(base: BaseMIRValue) -> BaseResult<Self> {
		if let RawType::LoweredStruct(_, _) = base.vtype.clone().as_generic_lowered()? {
			return Ok(MIRStructValue { base: base.clone(), t: base.vtype.clone().as_generic_lowered()? })
		}

		return Err(BaseError::critical(IR_CASTING_ERROR!().to_string()))
	}
}

impl Into<BaseMIRValue> for MIRStructValue {
	fn into(self) -> BaseMIRValue {
		return self.base;
	}
}

impl Display for MIRStructValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "#{}", self.base.get_ssa_index())?;

		Ok(())
	}
}
