//! Definitions for scope entry keys

use compiler_utils::hash::HashedString;
use typing::raw::RawType;

pub struct EntryKey {
	pub name: HashedString,

	/// This is to represent struct functions basically
	pub linked_type: Option<RawType>
}

impl EntryKey {
	/// Creates a new key for the given name with no types associated to it.
	pub fn new(name: HashedString) -> Self {
		EntryKey { name, linked_type: None }
	}

	/// Creates a new key for the given name with the given type associated to it
	pub fn new_linked(name: HashedString, linked: RawType) -> Self {
		EntryKey { name, linked_type: Some(linked) }
	}
}