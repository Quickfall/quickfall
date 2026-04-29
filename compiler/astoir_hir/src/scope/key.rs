//! Definitions for scope entry keys

use std::{fmt::Display, hash::Hash};

use compiler_utils::hash::HashedString;
use typing::raw::RawType;

pub struct EntryKey {
    pub name: HashedString,

    /// This is to represent struct functions basically
    pub linked_type: Option<RawType>,
}

impl EntryKey {
    /// Creates a new key for the given name with no types associated to it.
    pub fn new(name: HashedString) -> Self {
        EntryKey {
            name,
            linked_type: None,
        }
    }

    /// Creates a new key for the given name with the given type associated to it
    pub fn new_linked(name: HashedString, linked: RawType) -> Self {
        EntryKey {
            name,
            linked_type: Some(linked),
        }
    }
}

impl Hash for EntryKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(1257877); // Key magic

        self.name.val.hash(state);

        if self.linked_type.is_none() {
            state.write_usize(0);
        } else {
            self.linked_type.clone().unwrap().hash(state);
        }
    }
}

impl PartialEq for EntryKey {
    fn eq(&self, other: &Self) -> bool {
        self.name.val == other.name.val && self.linked_type == other.linked_type
    }
}
impl Eq for EntryKey {}

impl Display for EntryKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.val)
    }
}
