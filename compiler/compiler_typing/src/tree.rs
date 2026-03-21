//! The typing tree declarations. Allows for types such as an array of pointer arrays.

use compiler_errors::errs::{BaseResult, base::BaseError};

use crate::RawTypeReference;

#[derive(Clone, PartialEq)]
/// The node-based typing system of Quickfall. Allows for very specific types.
pub enum Type {
	/// A generic type node. Represents a classic type.
	/// 0: The raw type index
	/// 1: The type parameters
	/// 2: The size specifiers
	Generic(RawTypeReference, Vec<Box<Type>>, Vec<usize>), // Potential lowering to base-sized

	/// A pointer type node. Represents a pointer version
	/// 0: Is the pointer a poiner of arrays
	/// 1: Inner type
	Pointer(bool, Box<Type>),

	/// An array type node. Represents an array version
	/// 0: The size of the array
	/// 1: Inner type
	Array(usize, Box<Type>)
}

impl Type {
	/// Checks if the type tree can be transmuted into another one. Transmutation is the process used by the typing system to see if 
	/// a variable can automatically be casted into another type.
	/// 
	/// # Note
	/// This function uses recursion to go down the type tree and check `can_transmute` on every node.
	pub fn can_transmute(&self, other: &Type) -> bool {
		match (self, other) {
			(Self::Pointer(is_array, _), Self::Pointer(is_array_2, _)) => {
				return *is_array == *is_array_2;
			},

			(Self::Array(size, base), Self::Array(size2, base2)) => {
				if size != size2 {
					return false;
				}

				return base.clone().can_transmute(&base2);
			},

			// TODO: add generic transmutation checking when type storage implemented.
			(Self::Generic(raw_type, type_params, sizes), Self::Generic(raw_type2, type_params2, sizes2)) => {
				return raw_type == raw_type2 && type_params == type_params2 && sizes == sizes2;
			},

			_ => false
		}
	}

	pub fn get_inner_type(&self) -> BaseResult<Box<Type>> {
		match self {
			Type::Array(_, inner) => Ok(inner.clone()),
			Type::Pointer(_, inner) => Ok(inner.clone()),

			_ => Err(BaseError::err("Cannot gather inner type.".to_string()))
		}
	}

	pub fn get_generic_info(&self) -> BaseResult<(Vec<Box<Type>>, Vec<usize>)> {
		if let Type::Generic(_, types, sizes) = self {
			return Ok((types.clone(), sizes.clone()))
		}

		return self.get_inner_type()?.get_generic_info();
	}
}