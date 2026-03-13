use astoir_typing::{compacted::CompactedType};
use compiler_errors::errs::{BaseResult};

use crate::vals::{float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue};

/// Represents a basic value in the MIR.
#[derive(Clone)]
pub struct BaseMIRValue {
	val_index: usize,
	pub vtype: CompactedType
}

impl BaseMIRValue {
	#[deprecated(note = "This is meant for internal purposes, always use builders to safely create this!")]
	pub fn new(val_index: usize, vtype: CompactedType) -> Self {
		return BaseMIRValue { val_index, vtype }
	}

	pub fn as_int(&self) -> BaseResult<MIRIntValue> {
		return Ok(MIRIntValue::new(self.clone())?);
	}

	pub fn as_float(&self) -> BaseResult<MIRFloatValue> {
		return Ok(MIRFloatValue::new(self.clone())?)
	}

	pub fn as_ptr(&self) -> BaseResult<MIRPointerValue> {
		return Ok(MIRPointerValue::new(self.clone())?)
	}

	pub fn get_instruction(&self) -> usize {
		return self.val_index;
	}

}