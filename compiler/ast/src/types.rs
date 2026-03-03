/// A parsed complete type information
#[derive(Debug, PartialEq, Clone)]
pub struct CompleteType {
	pub base_type: u64, 
	pub sizes: Vec<usize>,
	pub types: Vec<u64>,

	pub pointer: bool,
	pub array: bool
}