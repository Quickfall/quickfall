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
		if base.vtype.base.is_integer() {
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