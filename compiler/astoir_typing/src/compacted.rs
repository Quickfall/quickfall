//! Definitions for types that are compacted. This means every type after the HIR

use crate::{base::BaseType, complete::ComplexType};

#[derive(Clone)]
pub struct CompactedType {
	pub base: BaseType,
	pub array: bool,
	pub pointer: bool,
	pub pointer_array: bool
}

impl From<ComplexType> for CompactedType {
	fn from(value: ComplexType) -> Self {
		let array = value.is_array();
		let concrete = value.get_concrete();

		return CompactedType { base: concrete.base.clone(), array, pointer: concrete.pointer, pointer_array: concrete.pointer_array }
	}
}
