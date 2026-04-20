use std::collections::HashMap;

use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, MaybeDiagnostic, builders::{make_already_in_scope, make_cannot_find, make_expected_simple_error}};

use crate::{entry::{GlobalStorageEntry, GlobalStorageEntryType}, key::EntryKey};

pub mod entry;
pub mod key;

pub type GlobalStorageIdentifier = usize;


#[derive(Debug)]
pub struct GlobalScopeStorage<T, R> {
	pub entry_to_ind: HashMap<EntryKey, usize>,
	pub entries: Vec<GlobalStorageEntry<T, R>>,

	pub descriptor_counter: usize,
	pub impl_counter: usize
}


/// The global storage for every element inside of the scope.
/// 
/// This stores the following:
/// - Functions (with or without implementations)
/// - Static variables
/// - Struct functions
/// - Types
/// 
/// # Safety
/// The `GlobalScopeStorage` enforces correctness for global scope types and strictly allows only one entry per name. globally.
impl<T: Clone, R: Clone> GlobalScopeStorage<T, R> {
	pub fn new() -> Self {
		GlobalScopeStorage { entry_to_ind: HashMap::new(), entries: vec![], descriptor_counter: 0, impl_counter: 0 }
	}

	pub fn append<K: DiagnosticSpanOrigin>(&mut self, name: EntryKey, entry: GlobalStorageEntryType<T, R>, origin: &K) -> MaybeDiagnostic {
		if self.entry_to_ind.contains_key(&name) {
			return Err(make_already_in_scope(origin, &name.name_hash).into())
		}

		let parent_index = self.entries.len();

		let entry = GlobalStorageEntry { entry_type: entry, parent_index };

		self.entries.push(entry);
		self.entry_to_ind.insert(name, parent_index);

		Ok(())
	}

	pub fn get_base<K: DiagnosticSpanOrigin>(&self, name: EntryKey, origin: &K) -> DiagnosticResult<GlobalStorageEntryType<T, R>> {
		if !self.entry_to_ind.contains_key(&name) {
			return Err(make_cannot_find(origin, &name.name_hash).into());
		}

		return Ok(self.entries[self.entry_to_ind[&name]].entry_type.clone())
	}

	pub fn get_type<K: DiagnosticSpanOrigin>(&self, name: EntryKey, origin: &K) -> DiagnosticResult<R> {
		let base = self.get_base(name, origin)?;

		return match base {
			GlobalStorageEntryType::Type(t) => Ok(t.clone()),
			_ => Err(make_expected_simple_error(origin, &"type".to_string(), &base).into())
		};
	}

	pub fn get_static_variable<K: DiagnosticSpanOrigin>(&self, name: EntryKey, origin: &K) -> DiagnosticResult<T> {
		let base = self.get_base(name, origin)?;

		return match base {
			GlobalStorageEntryType::StaticVariable(t) => Ok(t.clone()),
			_ => Err(make_expected_simple_error(origin, &"static variable".to_string(), &base).into())
		};
	}

	pub fn get_function_base<K: DiagnosticSpanOrigin>(&self, name: EntryKey, origin: &K) -> DiagnosticResult<usize> {
		let base = self.get_base(name, origin)?;

		return match base {
			GlobalStorageEntryType::Function { descriptor_ind, impl_ind: _ } => Ok(descriptor_ind),
			GlobalStorageEntryType::ImplLessFunction(descriptor_ind) => Ok(descriptor_ind),
			GlobalStorageEntryType::StructFunction { descriptor_ind, impl_ind: _, struct_type: _} => Ok(descriptor_ind),

			_ => Err(make_expected_simple_error(origin, &"function".to_string(), &base).into())
		};
	}

	pub fn get_function_impl<K: DiagnosticSpanOrigin>(&self, name: EntryKey, origin: &K) -> DiagnosticResult<usize> {
		let base = self.get_base(name, origin)?;

		return match base {
			GlobalStorageEntryType::Function { descriptor_ind: _, impl_ind } => Ok(impl_ind),
			GlobalStorageEntryType::StructFunction { descriptor_ind: _, impl_ind, struct_type: _ } => Ok(impl_ind),
			
			_ => Err(make_expected_simple_error(origin, &"function with implementation", &base).into())
		};
	}

	pub fn get_implless_function<K: DiagnosticSpanOrigin>(&self, name: EntryKey, origin: &K) -> DiagnosticResult<usize> {
		let base = self.get_base(name, origin)?;

		return match base {
			GlobalStorageEntryType::ImplLessFunction(descriptor_ind) => Ok(descriptor_ind),
			
			_ => Err(make_expected_simple_error(origin, &"function without implementation", &base).into())
		}
	}

	pub fn get_exact_function<K: DiagnosticSpanOrigin>(&self, name: EntryKey, origin: &K) -> DiagnosticResult<(usize, usize)> {
		let base = self.get_base(name, origin)?;

		return match base {
			GlobalStorageEntryType::Function { descriptor_ind, impl_ind} => Ok((descriptor_ind, impl_ind)),
			
			_ => Err(make_expected_simple_error(origin, &"function", &base).into())
		}
	}

	pub fn get_exact_struct_function<K: DiagnosticSpanOrigin>(&self, name: EntryKey, origin: &K) -> DiagnosticResult<(usize, usize, R)> {
		let base = self.get_base(name, origin)?;

		return match base {
			GlobalStorageEntryType::StructFunction { descriptor_ind, impl_ind, struct_type } => {
				if let GlobalStorageEntryType::Type(t) = self.entries[struct_type].entry_type.clone() {
					Ok((descriptor_ind, impl_ind, t))
				} else {
					Err(make_expected_simple_error(origin, &"type", &self.entries[0].entry_type).into())
				}
			},

			_ => Err(make_expected_simple_error(origin, &"struct function", &base).into())
		}
	}
}
