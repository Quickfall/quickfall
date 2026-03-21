/// A parsed complete type information
#[derive(Debug, PartialEq, Clone)]
#[deprecated(note = "Will be replaced by compiler_typing")]
pub struct CompleteType {
	pub base_type: u64, 
	pub sizes: Vec<usize>,
	pub types: Vec<u64>,

	pub pointer: bool,
	pub pointer_array: bool,
	pub array_sz: usize
}

/// The node-based typing system of Quickfall. Allows for very specific types. Only used for AST since types aren't resolved
#[derive(Clone)]
pub enum ASTType {
	/// A generic type node. Represents a classic type.
	/// 0: The raw type name
	/// 1: The type parameters
	/// 2: The size specifiers
	Generic(String, Vec<Box<ASTType>>, Vec<usize>), // Potential lowering to base-sized

	/// A pointer type node. Represents a pointer version
	/// 0: Is the pointer a poiner of arrays
	/// 1: Inner type
	Pointer(bool, Box<ASTType>),

	/// An array type node. Represents an array version
	/// 0: The size of the array
	/// 1: Inner type
	Array(usize, Box<ASTType>)
}
