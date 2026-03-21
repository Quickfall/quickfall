//! The raw type declarations

use crate::{SizedType, enums::{RawEnumEntryContainer, RawEnumTypeContainer}, structs::RawStructTypeContainer, tree::Type, utils::get_pointer_size};

/// The raw types. Are also named generics
#[derive(Clone)]
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

	SizedInteger(bool),
	SizedFloating(bool),
	SizedFixedPoint(bool)
}

impl SizedType for RawType {
	fn get_size(&self, t: &Type, compacted_size: bool) -> usize {
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

			RawType::Struct(_, container) => return container.get_size(t, compacted_size),

			_ => return 0
		}
	}
}