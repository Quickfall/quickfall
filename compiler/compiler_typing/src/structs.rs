use std::ops::Index;

use compiler_utils::utils::indexed::IndexStorage;

use crate::{TypedFunction, tree::Type};

/// Container for structure types
pub struct RawStructTypeContainer {
	pub fields: IndexStorage<Type>,
	pub functions: IndexStorage<TypedFunction>
}