use compiler_errors::{IR_CASTING_ERROR, errs::{BaseResult, base::BaseError}};

use crate::vals::base::{BaseMIRValue, BaseValueType};

pub struct MIRPointerValue {
	base: BaseMIRValue,
	pub t: BaseValueType,
	pub size: usize
}

impl MIRPointerValue {
	pub fn new(base: BaseMIRValue) -> BaseResult<Self> {
		if let BaseValueType::PointerValue { size, t } = &base.vtype {
			return Ok(MIRPointerValue { base: base.clone(), t: t.as_ref().clone(), size: *size })
		}

		return Err(BaseError::critical(IR_CASTING_ERROR!().to_string()))
	}
}

impl Into<BaseMIRValue> for MIRPointerValue {
	fn into(self) -> BaseMIRValue {
		return self.base;
	}
}