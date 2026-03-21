use compiler_errors::errs::BaseResult;
use compiler_utils::utils::indexed::IndexStorage;

use crate::{TypeReference, TypedFunction, tree::Type};

/// Container for structure types
#[derive(Clone)]
pub struct RawStructTypeContainer {
	pub fields: IndexStorage<TypeReference>,
	pub functions: IndexStorage<TypedFunction>
}

impl RawStructTypeContainer {
	pub fn get_size(&self, t: Type, compacted_size: usize) -> BaseResult<usize> {
		let mut size = 0;

		for field in &self.fields.vals {
			let base = field.clone().resolve(&t)?;

			
		}

		return Ok(size)
	}
}