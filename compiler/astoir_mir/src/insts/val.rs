use compiler_errors::errs::{BaseResult, base::BaseError};

use crate::vals::base::BaseMIRValue;

pub struct InstructionValue {
	val: Option<BaseMIRValue>	
}

impl InstructionValue {
	pub fn get(self) -> BaseResult<BaseMIRValue> {
		match self.val {
			Some(v) => return Ok(v),
			None => return Err(BaseError::err("Tried unpacking InstructionValue when contained null!".to_string()))
		}
	}
}