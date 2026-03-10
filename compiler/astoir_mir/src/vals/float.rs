use compiler_errors::{IR_CASTING_ERROR, errs::{BaseResult, base::BaseError}};

use crate::vals::base::{BaseMIRValue, BaseValueType};

#[derive(Clone)]
pub struct MIRFloatValue {
	base: BaseMIRValue,
	pub size: usize
}

impl MIRFloatValue {
	pub fn new(base: BaseMIRValue) -> BaseResult<Self> {
		if let BaseValueType::IntValue(e) = &base.vtype {
			return Ok(MIRFloatValue { base: base.clone(), size: *e })
		}

		return Err(BaseError::critical(IR_CASTING_ERROR!().to_string()))
	}
}

impl Into<BaseMIRValue> for MIRFloatValue {
	fn into(self) -> BaseMIRValue {
		return self.base;
	}
}