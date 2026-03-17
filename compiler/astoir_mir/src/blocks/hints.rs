use astoir_typing::compacted::CompactedType;
use compiler_errors::errs::{BaseResult, base::BaseError};

use crate::{blocks::MIRBlockVariableSSAHint, vals::{base::BaseMIRValue, consts::MIRConstantValue}};


/// A hint on a given value, contains constants or pointer types for example
#[derive(Clone)]
pub enum MIRValueHint {
	Constant(MIRConstantValue),
	Pointer(CompactedType),
	Value(CompactedType)
}

impl MIRValueHint {
	pub fn is_determined(&self) -> bool {
		if let &MIRValueHint::Constant(_) = self {
			return true;
		}

		return false;
	}

	pub fn as_const(&self) -> BaseResult<MIRConstantValue> {
		match self {
			MIRValueHint::Constant(e) => Ok(e.clone()),
			_ => Err(BaseError::critical("Cannot use as_const on a non const!".to_string()))
		}
	}

	pub fn get_type(&self) -> BaseResult<CompactedType> {
		match self {
			MIRValueHint::Pointer(e) => Ok(e.clone()),
			MIRValueHint::Value(e) => Ok(e.clone()),
			_ => Err(BaseError::critical("Cannot use get_type on an non typed hint".to_string()))
		}
	}

	pub fn as_pointer(&self) -> BaseResult<CompactedType> {
		match self {
			MIRValueHint::Pointer(e) => Ok(e.clone()),
			_ => Err(BaseError::critical("Cannot use as_pointer on a non pointer!".to_string()))
		}
	}

	pub fn as_value(&self) -> BaseResult<CompactedType> {
		match self {
			MIRValueHint::Value(e) => Ok(e.clone()),
			_ => Err(BaseError::critical("Cannot use as_value on a non value!".to_string()))
		}
	}

	pub fn from_ptr(val: CompactedType) -> Self {
		return MIRValueHint::Pointer(val)
	}

}

impl Into<MIRValueHint> for MIRConstantValue {
	fn into(self) -> MIRValueHint {
		return MIRValueHint::Constant(self)
	}
}

impl Into<MIRValueHint> for CompactedType {
	fn into(self) -> MIRValueHint {
		return MIRValueHint::Value(self)
	}
}

pub struct HintStorage {
	pub vec: Vec<MIRValueHint>,
}

impl HintStorage {
	pub fn new() -> Self {
		HintStorage { vec: vec![] }
	}


	/// Introduces a new SSA value hint. Returns the hint index. 
	/// # Usage
	/// Every single SSA value should have a hint on what it is. Furthermore, this hint index will be used to identify the different SSA values instead of raw instruction indexes.
	/// 
	/// # Globality
	/// Using hint indexes to represent different SSA values allows us to guarantee that SSA values will work on inner blocks.
	pub fn append_hint(&mut self, hint: MIRValueHint) -> usize {
		let ind = self.vec.len();

		self.vec.push(hint);

		return ind;
	}

	/// Gets the hint based on the hint index.
	pub fn get_hint(&self, hint_ind: usize) -> BaseResult<MIRValueHint> {
		if self.vec.len() <= hint_ind {
			return Err(BaseError::err("Invalid hint".to_string()))
		}

		return Ok(self.vec[hint_ind].clone())
	}
}