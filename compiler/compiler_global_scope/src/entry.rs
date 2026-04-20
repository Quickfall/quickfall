//! Definitions for entries

use std::fmt::Display;

use crate::GlobalStorageIdentifier;

#[derive(Clone, Debug)]
pub enum GlobalStorageEntryType<T, R> {
	Function { descriptor_ind: usize, impl_ind: usize }, 
	ImplLessFunction(usize),
	StructFunction { descriptor_ind: usize, impl_ind: usize, struct_type: GlobalStorageIdentifier },

	StaticVariable(T),

	TypeAlias(T), 
	Type(R)
}

#[derive(Debug)]
pub struct GlobalStorageEntry<T, R> {
	pub entry_type: GlobalStorageEntryType<T, R>,
	pub parent_index: usize
}

impl<T, R>  Display for GlobalStorageEntryType<T, R>  {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::Function { .. } => "function",
			Self::ImplLessFunction(_) => "function",
			Self::StructFunction { .. } => "function",
			Self::StaticVariable(_) => "static variable",
			Self::Type(_) => "type",
			Self::TypeAlias(_) => "type (alias)"
 		};

		write!(f, "{}", s)?;

		Ok(())
	}	
}