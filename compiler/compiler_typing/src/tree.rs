//! The typing tree declarations. Allows for types such as an array of pointer arrays.

use crate::RawType;

/// The node-based typing system of Quickfall. Allows for very specific types.
pub enum Type {
	/// A generic type node. Represents a classic type.
	/// 0: The raw type index
	/// 1: The type parameters
	/// 2: The size specifiers
	Generic(RawType, Vec<Box<Type>>, Vec<usize>), // Potential lowering to base-sized

	/// A pointer type node. Represents a pointer version
	/// 0: Is the pointer a poiner of arrays
	/// 1: Inner type
	Pointer(bool, Box<Type>),

	/// An array type node. Represents an array version
	/// 0: The size of the array
	/// 1: Inner type
	Array(usize, Box<Type>)
}