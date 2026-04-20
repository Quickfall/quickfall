//! Definitions for entries

use std::fmt::Display;

use crate::{GlobalStorageIdentifier};

#[derive(Clone, Debug)]
pub enum GlobalStorageEntryType<T, R, F, I> {
	Function(F, I),
	ImplLessFunction(F),
	StaticVariable(T),

	StructFunction(F, I, GlobalStorageIdentifier),
	
	Type(R)
}

#[derive(Debug)]
pub struct GlobalStorageEntry<T, R, F, I> {
	pub entry_type: GlobalStorageEntryType<T, R, F, I>,
	pub parent_index: usize
}

impl<T, R, F, I>  Display for GlobalStorageEntryType<T, R, F, I>  {
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