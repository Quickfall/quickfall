//! Definitions for types that are compacted. This means every type after the HIR

use crate::{base::BaseType, complete::ComplexType};

#[derive(Clone, Debug)]
pub struct CompactedType {
	pub base: BaseType,
	pub array: bool,
	pub pointer: bool,
	pub pointer_array: bool
}

impl PartialEq for CompactedType {
	fn eq(&self, other: &Self) -> bool {
		return self.base.is_equal(&other.base) && self.array == other.array && self.pointer == other.pointer && self.pointer_array == other.pointer_array;
	}
}

impl CompactedType {
	pub fn can_transmute(&self, other: &CompactedType) -> bool {
		return self.base.can_transmute_into(&other.base) && self.array == other.array && self.pointer == other.pointer && self.pointer_array == other.pointer_array;
	}
}

impl From<ComplexType> for CompactedType {
	fn from(value: ComplexType) -> Self {
		let array = value.is_array();
		let concrete = value.get_concrete();

		return CompactedType { base: concrete.base.clone(), array, pointer: concrete.pointer, pointer_array: concrete.pointer_array }
	}
}

impl From<BaseType> for CompactedType {
	fn from(value: BaseType) -> Self {
		return CompactedType { base: value, array: false, pointer: false, pointer_array: false }
	}
}
