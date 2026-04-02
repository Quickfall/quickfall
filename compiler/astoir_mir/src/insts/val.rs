use diagnostics::{DiagnosticResult, builders::make_invalid_instruction_held_val};

use crate::vals::base::BaseMIRValue;

pub struct InstructionValue {
	pub val: Option<BaseMIRValue>	
}

impl InstructionValue {
	pub fn new(val: Option<BaseMIRValue>) -> Self {
		return InstructionValue { val }
	}
	
	pub fn get(self) -> DiagnosticResult<BaseMIRValue> {
		match self.val {
			Some(v) => return Ok(v),
			None => return Err(make_invalid_instruction_held_val().into())
		}
	}
}