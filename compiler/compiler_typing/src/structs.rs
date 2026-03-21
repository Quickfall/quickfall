use compiler_utils::utils::indexed::IndexStorage;

use crate::{SizedType, TypeParameterContainer, TypeReference, TypedFunction, storage::TypeStorage, tree::Type};

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