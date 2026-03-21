//! The raw type declarations

use crate::{SizedType, enums::{RawEnumEntryContainer, RawEnumTypeContainer}, structs::RawStructTypeContainer};

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
	fn get_size(&self, compacted_size: bool) -> usize {
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

			RawType::Pointer => {
				if cfg!(target_pointer_width = "32") {
					return 32;
				} else if cfg!(target_pointer_width = "64") {
					return 64
				} else {
					return 0;
				}
			},

			RawType::StaticString => return 0, // TODO: make sure  we don't need this

			_ => return 0
		}
	}
}