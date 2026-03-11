use astoir_typing::structs::StructTypeContainer;
use compiler_errors::errs::{BaseResult, base::BaseError};

use crate::vals::{float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue};

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
	PointerValue, // variables
	AnyValue,
	StructTypeValue(StructTypeContainer, usize)
}

impl BaseValueType {
	pub fn as_struct(&self) -> BaseResult<StructTypeContainer> {
		match self {
			BaseValueType::StructTypeValue(e, _) => return Ok(e.clone()),
			_ => return Err(BaseError::critical("Cannot use as_struct on a non struct type!".to_string()))
 		};
	}

	pub fn eq(&self, b: &BaseValueType) -> bool {
		match (self, b) {
			(BaseValueType::IntValue(a), BaseValueType::IntValue(b)) => {
				return a == b;
			},

			(BaseValueType::FloatValue(a), BaseValueType::FloatValue(b)) => {
				return a == b;
			},

			(BaseValueType::StructTypeValue(_, a), BaseValueType::StructTypeValue(_, b)) => {
				return a == b;
			},

			(BaseValueType::PointerValue, BaseValueType::PointerValue) => true,
			(BaseValueType::AnyValue, BaseValueType::AnyValue) => true,

			_ => return false
		}
	}
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

	pub fn as_ptr(&self) -> BaseResult<MIRPointerValue> {
		return Ok(MIRPointerValue::new(self.clone())?)
	}

	pub fn get_instruction(&self) -> usize {
		return self.val_index;
	}

}