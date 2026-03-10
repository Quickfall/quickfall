use compiler_errors::{IR_CASTING_ERROR, errs::{BaseResult, base::BaseError}};

use crate::vals::base::{BaseMIRValue, BaseValueType};

pub struct MIRIntValue {
	base: BaseMIRValue,
	pub size: usize,
}

impl MIRIntValue {
	pub fn new(base: BaseMIRValue) -> BaseResult<Self> {
		if let BaseValueType::IntValue(e) = &base.vtype {
			return Ok(MIRIntValue { base: base.clone(), size: *e })
		}

		return Err(BaseError::critical(IR_CASTING_ERROR!().to_string()))
	}
}

impl Into<BaseMIRValue> for MIRIntValue {
	fn into(self) -> BaseMIRValue {
		return self.base;
	}
}