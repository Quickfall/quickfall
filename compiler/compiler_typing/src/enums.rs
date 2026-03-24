//! Declarations for enum-kind types.

use std::{collections::HashMap};

use compiler_errors::{ENUM_PARENT_FIELD, IR_FIND_ELEMENT, IR_FIND_TYPE, errs::{BaseResult, base::BaseError}};
use compiler_utils::{hash::{HashedString}, utils::indexed::IndexStorage};

use crate::{RawTypeReference, SizedType, StructuredType, TypeParameterContainer, TypedFunction, raw::RawType, references::TypeReference, storage::TypeStorage, tree::Type};

/// The container for the parent type of enum.
/// 
/// # Safety
/// This struct guarantees that every contained entry is of type RawType::EnumEntry
#[derive(Clone, Debug, PartialEq)]
pub struct RawEnumTypeContainer {
	self_ref: usize,
	pub type_params: TypeParameterContainer,
	pub functions: IndexStorage<TypedFunction>,
	entries: HashMap<HashedString, RawType>
}

impl RawEnumTypeContainer {
	pub fn new(self_ref: usize, type_params: TypeParameterContainer) -> Self {
		RawEnumTypeContainer { self_ref, entries: HashMap::new(), type_params, functions: IndexStorage::new() }
	}

	pub fn append_entry(&mut self, name: HashedString, fields: Vec<(u64, TypeReference)>) {
		let entry_container = RawEnumEntryContainer::new(self.self_ref, fields);

		self.entries.insert(name, RawType::EnumEntry(entry_container));
	}

	pub fn get_entry(&self, name: HashedString) -> BaseResult<RawType> {
		if let Some(v) = self.entries.get(&name) {
			return Ok(v.clone());
		}

		return Err(BaseError::err(IR_FIND_TYPE!().to_string()))
	}
}

impl SizedType for RawEnumTypeContainer {
	fn get_size(&self, t: &Type, compacted_size: bool, storage: &TypeStorage) -> usize {
		let mut entry_size = 0;

		if compacted_size {
			let raw = t.get_generic(storage);

			if let RawType::EnumEntry(container) = raw {
				entry_size = container.get_size(t, compacted_size, storage);
			}
		}
		else {
			for entry in &self.entries {
				entry_size = entry_size.max(entry.1.get_size(t, compacted_size, storage));
			}
		}

		let hint = RawType::make_hint(self.entries.len());

		return hint.get_size(t, compacted_size, storage) + entry_size;
	}
}

/// The container for enum entries.
#[derive(Clone, Debug, PartialEq)]
pub struct RawEnumEntryContainer {
	pub parent: RawTypeReference,
	pub fields: IndexStorage<TypeReference>
}

impl RawEnumEntryContainer {
	pub fn new(parent: RawTypeReference, fields: Vec<(u64, TypeReference)>) -> Self {
		let mut storage = IndexStorage::new();

		for field in fields {
			let _ = storage.append(field.0, field.1);
		}

		RawEnumEntryContainer { parent, fields: storage }
	}
}

impl SizedType for RawEnumEntryContainer {
	fn get_size(&self, t: &Type, compacted_size: bool, storage: &TypeStorage) -> usize {
		let mut size = 0;

		for tt in &self.fields.vals {
			size += tt.clone().resolve(t).get_size(t, compacted_size, storage);
		}

		return size;
	}
}

impl StructuredType for RawEnumTypeContainer {
	fn get_field(&self, _hash: u64, _storage: &TypeStorage) -> BaseResult<TypeReference> {
		return Err(BaseError::err(ENUM_PARENT_FIELD!().to_string()))
	}

	fn get_field_hash(&self, _hash: u64, _storage: &TypeStorage) -> BaseResult<usize> {
		return Err(BaseError::err(ENUM_PARENT_FIELD!().to_string()))

	}

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
}

impl StructuredType for RawEnumEntryContainer {
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

	fn get_function(&self, hash: u64, storage: &TypeStorage) -> BaseResult<TypedFunction> {
		if let RawType::Enum(container) = &storage.types.vals[self.parent] {
			return container.get_function(hash, storage);
		}

		panic!("Parent type of enum entry was not an enum!");
	}

	fn get_function_hash(&self, hash: u64, storage: &TypeStorage) -> BaseResult<usize> {
		if let RawType::Enum(container) = &storage.types.vals[self.parent] {
			return container.get_function_hash(hash, storage);
		}

		panic!("Parent type of enum entry was not an enum!");
	}
}