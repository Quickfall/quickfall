//! The raw type declarations

use crate::structs::RawStructTypeContainer;

/// The raw types. Are also named generics
pub enum RawType {
	Integer(usize, bool),
	Floating(usize, bool),
	FixedPoint(usize, usize, bool),

	Boolean,

	Pointer,

	StaticString,

	Struct(bool, RawStructTypeContainer),

	SizedInteger(bool),
	SizedFloating(bool),
	SizedFixedPoint(bool)
}