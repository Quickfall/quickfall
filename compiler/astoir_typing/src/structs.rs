use compiler_errors::{IR_FIND_ELEMENT, errs::{BaseResult, base::BaseError}};
use compiler_utils::utils::indexed::IndexStorage;

use crate::complete::ComplexType;

#[derive(Clone, Debug)]
pub struct StructTypeContainer {
	pub fields: IndexStorage<ComplexType>,
	pub functions: IndexStorage<(Option<ComplexType>, Vec<(u64, ComplexType)>)>
}

impl StructTypeContainer {
	pub fn new() -> Self {
		return StructTypeContainer { fields: IndexStorage::new(), functions: IndexStorage::new() }
	}

	pub fn get_field(&self, hash: u64) -> BaseResult<usize> {
		return match self.fields.get_index(hash) {
			Some(v) => Ok(v),
			None => Err(BaseError::err(IR_FIND_ELEMENT!().to_string()))
		}
	}

	pub fn get_function(&self, hash: u64) -> BaseResult<usize> {
		return match self.functions.get_index(hash) {
			Some(v) => Ok(v),
			None => Err(BaseError::err(IR_FIND_ELEMENT!().to_string()))
		}
	}
}
