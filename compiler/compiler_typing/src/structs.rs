use compiler_errors::{IR_FIND_ELEMENT, errs::{BaseResult, base::BaseError}};
use compiler_utils::utils::indexed::IndexStorage;

use crate::{SizedType, StructuredType, TypeParameterContainer, TypeReference, TypedFunction, storage::TypeStorage, tree::Type};

/// Container for structure types
#[derive(Clone, Debug)]
pub struct RawStructTypeContainer {
	pub fields: IndexStorage<TypeReference>,
	pub type_params: TypeParameterContainer,
	pub functions: IndexStorage<TypedFunction>
}

impl SizedType for RawStructTypeContainer {
	fn get_size(&self, t: &Type, compacted_size: bool, storage: &TypeStorage) -> usize {
		let mut size = 0;

		for field in &self.fields.vals {
			let base = field.clone().resolve(&t);

			size += base.get_size(t, compacted_size, storage);
		}

		return size
	}
}

impl StructuredType for RawStructTypeContainer {
	fn get_function(&self, hash: u64, _storage: &TypeStorage) -> BaseResult<TypedFunction> {
		let k = match self.functions.get_index(hash) {
			Some(v) => v,
			None => return Err(BaseError::err(IR_FIND_ELEMENT!().to_string()))
		};

		return Ok(self.functions.vals[k].clone())
	}

	fn get_function_hash(&self, hash: u64, _storage: &TypeStorage) -> BaseResult<usize> {
		let k = match self.functions.get_index(hash) {
			Some(v) => v,
			None => return Err(BaseError::err(IR_FIND_ELEMENT!().to_string()))
		};

		return Ok(k);
	}

	fn get_field(&self, hash: u64, _storage: &TypeStorage) -> BaseResult<TypeReference> {
		let k = match self.fields.get_index(hash) {
			Some(v) => v,
			None => return Err(BaseError::err(IR_FIND_ELEMENT!().to_string()))
		};

		return Ok(self.fields.vals[k].clone());
	} 

	fn get_field_hash(&self, hash: u64, _storage: &TypeStorage) -> BaseResult<usize> {
		let k = match self.fields.get_index(hash) {
			Some(v) => v,
			None => return Err(BaseError::err(IR_FIND_ELEMENT!().to_string()))
		};

		return Ok(k);
	}
}