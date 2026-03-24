use std::fmt::Display;

use compiler_errors::errs::{BaseResult};
use compiler_typing::tree::Type;

use crate::vals::{float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue};

/// Represents a basic value in the MIR.
#[derive(Clone, Debug)]
pub struct BaseMIRValue {
	val_index: usize,
	pub vtype: Type
}

impl BaseMIRValue {
	pub fn new(val_index: usize, vtype: Type) -> Self {
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

	pub fn get_ssa_index(&self) -> usize {
		return self.val_index;
	}
}

impl PartialEq for BaseMIRValue {
	fn eq(&self, other: &Self) -> bool {
		return self.val_index == other.val_index && self.vtype == other.vtype;
	}
}

impl Display for BaseMIRValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "#{}", self.val_index)?;

		Ok(())
	}
}