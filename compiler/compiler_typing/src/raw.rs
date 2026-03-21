//! The raw type declarations

/// The raw types. Are also named generics
pub enum RawType {
	Integer(usize, bool),
	Floating(usize, bool),
	FixedPoint(usize, usize, bool),

	Boolean,

	Pointer,

	StaticString,

	

	SizedInteger(bool),
	SizedFloating(bool),
	SizedFixedPoint(bool)
}