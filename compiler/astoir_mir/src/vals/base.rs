use compiler_errors::errs::BaseResult;

use crate::vals::{float::MIRFloatValue, int::MIRIntValue};

/// Represents a basic value in the MIR.
#[derive(Clone)]
pub struct BaseMIRValue {
	val_index: usize,
	pub vtype: BaseValueType
}

#[derive(Clone)]
pub enum BaseValueType {
	IntValue(usize),
	FloatValue(usize),
	FixedValue(usize), // fixed point
	PointerValue { size: usize, t: Box<BaseValueType> }, // variables
}

impl BaseMIRValue {
	#[deprecated(note = "This is meant for internal purposes, always use builders to safely create this!")]
	pub fn new(val_index: usize, vtype: BaseValueType) -> Self {
		return BaseMIRValue { val_index, vtype }
	}

	pub fn as_int(&self) -> BaseResult<MIRIntValue> {
		return Ok(MIRIntValue::new(self.clone())?);
	}

	pub fn as_float(&self) -> BaseResult<MIRFloatValue> {
		return Ok(MIRFloatValue::new(self.clone())?)
	}
 
}