use std::collections::HashMap;

use compiler_errors::errs::{BaseResult, base::BaseError};

use crate::vals::{base::{BaseValueType}, consts::MIRConstantValue};


/// A hint on a given value, contains constants or pointer types for example
#[derive(Clone)]
pub enum MIRValueHint {
	Constant(MIRConstantValue),
	Pointer(BaseValueType),
	Value(BaseValueType)
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

	pub fn as_pointer(&self) -> BaseResult<BaseValueType> {
		match self {
			MIRValueHint::Pointer(e) => Ok(e.clone()),
			_ => Err(BaseError::critical("Cannot use as_pointer on a non pointer!".to_string()))
		}
	}

	pub fn as_value(&self) -> BaseResult<BaseValueType> {
		match self {
			MIRValueHint::Value(e) => Ok(e.clone()),
			_ => Err(BaseError::critical("Cannot use as_value on a non value!".to_string()))
		}
	}

	pub fn from_ptr(val: BaseValueType) -> Self {
		return MIRValueHint::Pointer(val)
	}

}

impl Into<MIRValueHint> for MIRConstantValue {
	fn into(self) -> MIRValueHint {
		return MIRValueHint::Constant(self)
	}
}

impl Into<MIRValueHint> for BaseValueType {
	fn into(self) -> MIRValueHint {
		return MIRValueHint::Value(self)
	}
}

pub struct HintStorage {
	map: HashMap<usize, usize>,
	pub vec: Vec<MIRValueHint>
}

impl HintStorage {
	pub fn new() -> Self {
		HintStorage { map: HashMap::new(), vec: vec![] }
	}

	pub fn append_hint(&mut self, inst_index: usize, hint: MIRValueHint) -> usize {
		let ind = self.vec.len();

		self.map.insert(inst_index, ind);
		self.vec.push(hint);

		return ind;
	}

	pub fn get_hint(&self, inst_index: usize) -> BaseResult<MIRValueHint> {
		if !self.map.contains_key(&inst_index) {
			return Err(BaseError::err("Cannot find hint!".to_string()));
		}

		return Ok(self.vec[self.map[&inst_index]].clone());
	}

}