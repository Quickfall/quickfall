use compiler_utils::utils::indexed::IndexStorage;

use crate::{SizedType, TypeReference, TypedFunction, tree::Type};

/// Container for structure types
#[derive(Clone)]
pub struct RawStructTypeContainer {
	pub fields: IndexStorage<TypeReference>,
	pub functions: IndexStorage<TypedFunction>
}

impl SizedType for RawStructTypeContainer {
	fn get_size(&self, t: &Type, compacted_size: bool) -> usize {
		let mut size = 0;

		for field in &self.fields.vals {
			let base = field.clone().resolve(&t);

			size += base.get_size(t, compacted_size);
		}

		return size
	}
}