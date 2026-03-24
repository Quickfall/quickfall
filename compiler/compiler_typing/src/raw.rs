//! The raw type declarations

use std::hash::Hash;

use crate::{SizedType, enums::{RawEnumEntryContainer, RawEnumTypeContainer}, storage::TypeStorage, structs::{LoweredStructTypeContainer, RawStructTypeContainer}, tree::Type, utils::get_pointer_size};

/// The raw types. Are also named generics
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RawType {
	Integer(usize, bool),
	Floating(usize, bool),
	FixedPoint(usize, usize, bool),

	Boolean,

	Pointer,

	StaticString,

	Struct(bool, RawStructTypeContainer),
	
	Enum(RawEnumTypeContainer),
	EnumEntry(RawEnumEntryContainer),

	LoweredStruct(bool, LoweredStructTypeContainer),

	SizedInteger(bool),
	SizedFloating(bool),
	SizedFixedPoint(bool)
}

impl RawType {
	/// Make a hint type.
	pub fn make_hint(hint_amount: usize) -> RawType {
		let bits = (hint_amount as f64).log2().ceil() as usize;

		return RawType::Integer(bits, false)
	}

	pub fn get_type_params_count(&self, storage: &TypeStorage) -> usize {
		match self {
			RawType::Enum(container) => container.type_params.len(),
			RawType::EnumEntry(container) => storage.types.vals[container.parent].get_type_params_count(storage),

			_ => 0
		}
	}

	pub fn is_signed(&self) -> bool {
		match self {
			Self::Integer(_, signed) => *signed,
			Self::Floating(_, signed) => *signed,
			Self::FixedPoint(_, _, signed) => *signed,

			_ => false
		}
	}

	pub fn is_sized(&self) -> bool {
		match self {
			Self::SizedInteger(_) => true,
			Self::SizedFloating(_) => true,
			Self::SizedFixedPoint(_) => true,

			_ => false
		}
	}

	pub fn can_transmute(&self, self_size: Vec<usize>, b: &RawType, b_sizes: Vec<usize>) -> bool {
		match (self, b) {
			(Self::Integer(_, _), Self::Integer(_, _)) => true,
			(Self::SizedInteger(_), Self::Integer(_, _)) => true,
			(Self::Integer(_, _), Self::SizedInteger(_)) => true,

			(Self::Floating(_, _), Self::Integer(_, _)) => true,
			(Self::Integer(_, _), Self::Floating(_, _)) => true,

			(Self::Floating(_, _), Self::SizedInteger(_)) => true,
			(Self::SizedInteger(_), Self::Floating(_, _)) => true,

			_ => false
		}
	}
}

impl SizedType for RawType {
	fn get_size(&self, t: &Type, compacted_size: bool, storage: &TypeStorage) -> usize {
		match self {
			RawType::Integer(size, _) => *size,
			RawType::Floating(size, _) => *size,
			RawType::FixedPoint(size_a, size_b, _) => *size_a + size_b,
			RawType::Boolean => {
				if compacted_size {
					return 1;
				}

				return 8;
			},

			RawType::Pointer => return get_pointer_size(),

			RawType::StaticString => return 0, // TODO: make sure  we don't need this

			RawType::Struct(_, container) => return container.get_size(t, compacted_size, storage),
			RawType::Enum(container) => return container.get_size(t, compacted_size, storage),
			RawType::EnumEntry(container) => return container.get_size(t, compacted_size, storage),

			_ => return 0
		}
	}
}

impl Hash for RawType {
	fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
		match self {
			Self::Integer(a, b) => {
				hasher.write_usize(0);
				hasher.write_usize(*a);
				hasher.write_u8(*b as u8);
			},

			Self::Floating(a, b) => {
				hasher.write_usize(1);
				hasher.write_usize(*a);
				hasher.write_u8(*b as u8);
			},

			Self::FixedPoint(a, b, c) => {
				hasher.write_usize(2);
				hasher.write_usize(*a);
				hasher.write_usize(*b);
				hasher.write_u8(*c as u8);
			},

			Self::Boolean => hasher.write_usize(3),
			Self::Pointer => hasher.write_usize(4),
			Self::StaticString => hasher.write_usize(5),
			
			_ => panic!("Unhashable type {:#?}", self)
		}
	}
}