//! Declarations for enum-kind types.

use std::{collections::HashMap};

use compiler_errors::{IR_FIND_TYPE, errs::{BaseResult, base::BaseError}};
use compiler_utils::{hash::{HashedString}, utils::indexed::IndexStorage};

use crate::{RawTypeReference, raw::RawType, tree::Type};

/// The container for the parent type of enum.
/// 
/// # Safety
/// This struct guarantees that every contained entry is of type RawType::EnumEntry
#[derive(Clone)]
pub struct RawEnumTypeContainer {
	self_ref: usize,
	entries: HashMap<HashedString, RawType>
}

impl RawEnumTypeContainer {
	pub fn new(self_ref: usize) -> Self {
		RawEnumTypeContainer { self_ref, entries: HashMap::new() }
	}

	pub fn append_entry(&mut self, name: HashedString, fields: Vec<(u64, Type)>) {
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

/// The container for enum entries.
#[derive(Clone)]
pub struct RawEnumEntryContainer {
	pub parent: RawTypeReference,
	pub fields: IndexStorage<Type>
}

impl RawEnumEntryContainer {
	pub fn new(parent: RawTypeReference, fields: Vec<(u64, Type)>) -> Self {
		let mut storage = IndexStorage::new();

		for field in fields {
			let _ = storage.append(field.0, field.1);
		}

		RawEnumEntryContainer { parent, fields: storage }
	}
}