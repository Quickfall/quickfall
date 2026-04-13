//! Declarations for enum-kind types.

use std::{collections::HashMap};

use compiler_utils::{hash::{HashedString}, utils::indexed::IndexStorage};
use diagnostics::{DiagnosticResult, builders::{make_cannot_find_type_field, make_cannot_find_type_function, make_cannot_find_type_pos, make_enum_parent_fields}};

use crate::{RawTypeReference, SizedType, StructuredType, TypeParamType, TypeParameterContainer, TypedFunction, raw::RawType, references::TypeReference, storage::TypeStorage, tree::Type};

/// The container for the parent type of enum.
/// 
/// # Safety
/// This struct guarantees that every contained entry is of type RawType::EnumEntry
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawEnumTypeContainer {
	pub self_ref: usize,
	pub type_params: TypeParameterContainer,
	pub functions: IndexStorage<TypedFunction>,
	pub entries: HashMap<HashedString, RawType>
}

impl RawEnumTypeContainer {
	pub fn new(self_ref: usize, type_params: TypeParameterContainer) -> Self {
		RawEnumTypeContainer { self_ref, entries: HashMap::new(), type_params, functions: IndexStorage::new() }
	}

	pub fn append_entry(&mut self, name: HashedString, fields: Vec<(u64, TypeReference)>) {
		let mut entry_container = RawEnumEntryContainer::new(self.self_ref, fields, self.type_params.clone());
		entry_container.child = self.entries.len();

		self.entries.insert(name, RawType::EnumEntry(entry_container));
	}

	#[must_use = "Must set the diagnostic position beforehand"]
	pub fn get_entry(&self, name: HashedString) -> DiagnosticResult<RawType> {
		if let Some(v) = self.entries.get(&name) {
			return Ok(v.clone());
		}

		return Err(make_cannot_find_type_pos(&format!("{}::{}", self.self_ref, name.val)).into())
	}

	pub fn get_hint_type(&self) -> RawType {
		RawType::make_hint(self.entries.len())
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawEnumEntryContainer {
	pub parent: RawTypeReference,
	pub type_params: TypeParameterContainer,
	pub child: usize,
	pub fields: IndexStorage<TypeReference>
}

impl RawEnumEntryContainer {
	pub fn new(parent: RawTypeReference, fields: Vec<(u64, TypeReference)>, type_params: TypeParameterContainer) -> Self {
		let mut storage = IndexStorage::new();

		for field in fields {
			let _ = storage.append(field.0, field.1);
		}

		RawEnumEntryContainer { parent, fields: storage, child: 0, type_params }
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
	fn get_field(&self, _hash: u64, _storage: &TypeStorage) -> DiagnosticResult<TypeReference> {
		return Err(make_enum_parent_fields().into())
	}

	fn get_field_hash(&self, _hash: u64, _storage: &TypeStorage) -> DiagnosticResult<usize> {
		return Err(make_enum_parent_fields().into())
	}

	fn get_fields(&self, _storage: &TypeStorage) -> Vec<u64> {
		return vec![];
	}

	fn get_functions(&self, _storage: &TypeStorage) -> Vec<u64> {
		return self.functions.entry_keys.clone();
	}

	fn get_function(&self, hash: u64, _storage: &TypeStorage) -> DiagnosticResult<TypedFunction> {
		let k = match self.functions.get_index(hash) {
			Some(v) => v,
			None => return Err(make_cannot_find_type_function(&hash, &"unnamed".to_string()).into())
		};

		return Ok(self.functions.vals[k].clone())
	}

	fn get_function_hash(&self, hash: u64, _storage: &TypeStorage) -> DiagnosticResult<usize> {
		let k = match self.functions.get_index(hash) {
			Some(v) => v,
			None => return Err(make_cannot_find_type_function(&hash, &"unnamed".to_string()).into())
		};

		return Ok(k);
	}
}

impl TypeParamType for RawEnumTypeContainer {
	fn has_type_param(&self, param: &HashedString) -> bool {
		self.type_params.contains_key(param)
	}

	fn get_type_param_ind(&self, param: &HashedString) -> usize {
		self.type_params[param]
	}
}

impl StructuredType for RawEnumEntryContainer {
	fn get_field(&self, hash: u64, _storage: &TypeStorage) -> DiagnosticResult<TypeReference> {
		let k = match self.fields.get_index(hash) {
			Some(v) => v,
			None => return Err(make_cannot_find_type_field(&hash, &"unamed".to_string()).into())
		};

		return Ok(self.fields.vals[k].clone());
	} 

	fn get_field_hash(&self, hash: u64, _storage: &TypeStorage) -> DiagnosticResult<usize> {
		let k = match self.fields.get_index(hash) {
			Some(v) => v,
			None => return Err(make_cannot_find_type_field(&hash, &"unamed".to_string()).into())
		};

		return Ok(k);
	}

	fn get_fields(&self, _storage: &TypeStorage) -> Vec<u64> {
		return self.fields.entry_keys.clone();
	}

	fn get_functions(&self, storage: &TypeStorage) -> Vec<u64> {
		if let RawType::Enum(container) = &storage.types.vals[self.parent] {
			return container.get_functions(storage);
		}

		panic!("Parent type of enum entry was not an enum!");
	}

	fn get_function(&self, hash: u64, storage: &TypeStorage) -> DiagnosticResult<TypedFunction> {
		if let RawType::Enum(container) = &storage.types.vals[self.parent] {
			return container.get_function(hash, storage);
		}

		panic!("Parent type of enum entry was not an enum!");
	}

	fn get_function_hash(&self, hash: u64, storage: &TypeStorage) -> DiagnosticResult<usize> {
		if let RawType::Enum(container) = &storage.types.vals[self.parent] {
			return container.get_function_hash(hash, storage);
		}

		panic!("Parent type of enum entry was not an enum!");
	}
}

impl TypeParamType for RawEnumEntryContainer {
	fn has_type_param(&self, param: &HashedString) -> bool {
		self.type_params.contains_key(param)
	}

	fn get_type_param_ind(&self, param: &HashedString) -> usize {
		self.type_params[param]
	}
}