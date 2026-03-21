//! Declarations for enum-kind types.

use std::collections::HashMap;

use compiler_utils::{hash::SelfHash, utils::indexed::IndexStorage};

use crate::tree::Type;

/// The container for the parent type of enum.
pub struct RawEnumTypeContainer {
	pub entries: HashMap<SelfHash, Type>
}

/// The container for enum entries.
pub struct RawEnumEntryContainer {
	pub fields: IndexStorage<Type>
}