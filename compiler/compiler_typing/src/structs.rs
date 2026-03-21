use compiler_utils::utils::indexed::IndexStorage;

use crate::{TypeReference, TypedFunction};

/// Container for structure types
#[derive(Clone)]
pub struct RawStructTypeContainer {
	pub fields: IndexStorage<TypeReference>,
	pub functions: IndexStorage<TypedFunction>
}