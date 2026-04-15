//! The global HIR storage, basically stores types, functions, and more

use std::{collections::HashMap, fmt::Display, hash::Hash};

use compiler_typing::tree::Type;
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, MaybeDiagnostic, builders::{make_already_in_scope, make_cannot_find}};

use crate::{ctx::HIRFunction, nodes::HIRNode};

pub type GlobalStorageIdentifier = usize;

#[derive(Clone)]
pub enum GlobalStorageEntryType {
	Function(HIRFunction, Box<HIRNode>),
	ImplLessFunction(HIRFunction),
	StaticVariable(Type),

	StructFunction(HIRFunction, Box<HIRNode>, GlobalStorageIdentifier),
	
	Type(Type)
}

/// Represents a key to a global storage entry. Potentially allows for namespaces later on
pub struct EntryKey {
	pub name_hash: u64
}

impl Hash for EntryKey {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		state.write_u64(self.name_hash);
	}
}

impl PartialEq for EntryKey {
	fn eq(&self, other: &Self) -> bool {
		self.name_hash == other.name_hash
	}
}

impl Eq for EntryKey {}

pub struct GlobalStorageEntry {
	pub entry_type: GlobalStorageEntryType,
	pub parent_index: usize
}

pub struct GlobalScopeStorage {
	pub entry_to_ind: HashMap<EntryKey, usize>,
	pub entries: Vec<GlobalStorageEntry>,
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
impl GlobalScopeStorage {
	pub fn new() -> Self {
		GlobalScopeStorage { entry_to_ind: HashMap::new(), entries: vec![] }
	}

	pub fn append<K: DiagnosticSpanOrigin>(&mut self, name: EntryKey, entry: GlobalStorageEntryType, origin: &K) -> MaybeDiagnostic {
		if self.entry_to_ind.contains_key(&name) {
			return Err(make_already_in_scope(origin, &name.name_hash).into())
		}

		let parent_index = self.entries.len();

		let entry = GlobalStorageEntry { entry_type: entry, parent_index };

		self.entries.push(entry);
		self.entry_to_ind.insert(name, parent_index);

		Ok(())
	}

	pub fn get_base<K: DiagnosticSpanOrigin>(&mut self, name: EntryKey, origin: &K) -> DiagnosticResult<GlobalStorageEntryType> {
		if !self.entry_to_ind.contains_key(&name) {
			return Err(make_cannot_find(origin, &name.name_hash).into());
		}

		return Ok(self.entries[self.entry_to_ind[&name]].entry_type.clone())
	}
}

impl Display for GlobalStorageEntryType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::Function(_, _) => "function",
			Self::ImplLessFunction(_) => "function",
			Self::StructFunction(_, _, _) => "function",
			Self::StaticVariable(_) => "static variable",
			Self::Type(_) => "type"
 		};

		write!(f, "{}", s)?;

		Ok(())
	}	
}