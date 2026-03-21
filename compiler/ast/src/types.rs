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