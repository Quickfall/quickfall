//! The typing tree declarations. Allows for types such as an array of pointer arrays.

use compiler_errors::errs::{BaseResult, base::BaseError};
use crate::{RawTypeReference, SizedType, StructuredType, TypedFunction, raw::RawType, references::TypeReference, storage::{TypeStorage}, utils::get_pointer_size};

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
/// The node-based typing system of Quickfall. Allows for very specific types.
pub enum Type {
	/// A generic type node. Represents a classic type.
	/// 0: The raw type index
	/// 1: The type parameters
	/// 2: The size specifiers
	Generic(RawTypeReference, Vec<Box<Type>>, Vec<usize>), // Potential lowering to base-sized

	/// A generic type node but lowered. Represents a concrete type.
	GenericLowered(RawType),

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
	pub fn can_transmute(&self, other: &Type, storage: &TypeStorage) -> bool {
		match (self, other) {
			(Self::Pointer(is_array, _), Self::Pointer(is_array_2, _)) => {
				return *is_array == *is_array_2;
			},

			(Self::Array(size, base), Self::Array(size2, base2)) => {
				if size != size2 {
					return false;
				}

				return base.clone().can_transmute(&base2, storage);
			},

			// TODO: add generic transmutation checking when type storage implemented.
			(Self::Generic(raw_type, type_params, sizes), Self::Generic(raw_type2, type_params2, sizes2)) => {
				if type_params != type_params2 {
					return false;
				}

				return storage.types.vals[*raw_type].can_transmute(sizes.clone(), &storage.types.vals[*raw_type2], sizes2.clone())
			},

			(Self::GenericLowered(base), Self::Generic(rawtype, type_params, sizes)) => {
				if !type_params.is_empty() {
					return false;
				}

				return base.can_transmute(vec![], &storage.types.vals[*rawtype], sizes.clone())
			}

			(Self::GenericLowered(base), Self::GenericLowered(base2)) => {
				return base.can_transmute(vec![], base2, vec![]);
			}


			_ => false
		}
	}

	pub fn is_truly_eq(&self, other: &Type) -> bool {
		match (self, other) {
			(Self::Pointer(is_array, _), Self::Pointer(is_array_2, _)) => {
				return *is_array == *is_array_2;
			},

			(Self::Array(size, base), Self::Array(size2, base2)) => {
				if size != size2 {
					return false;
				}

				return base.is_truly_eq(base2);
			},

			(Self::Generic(raw_type, type_params, sizes), Self::Generic(raw_type2, type_params2, sizes2)) => {
				return raw_type == raw_type2 && type_params == type_params2 && sizes == sizes2;
			},

			(Self::GenericLowered(base), Self::GenericLowered(base2)) => {
				if *base == RawType::Pointer && *base2 == RawType::StaticString {
					return true;
				}

				return base == base2;
			}

			_ => false
		}
	}

	pub fn as_generic_lowered(&self) -> BaseResult<RawType> {
		match self {
			Type::GenericLowered(a) => return Ok(a.clone()),
			_ => return Err(BaseError::err("Not lowered generic".to_string()))
		}
	}	

	pub fn get_inner_type(&self) -> Box<Type> {
		match self {
			Type::Array(_, inner) => inner.clone(),
			Type::Pointer(_, inner) => inner.clone(),

			_ => {
				panic!("Error! Compiler tried using get_inner_type on bottom type! Returning bottom type incase!");
			}
		}
	}

	pub fn can_use_index_access(&self) -> bool {
		match self {
			Type::Array(_, _) => true,
			_ => false
		}
	}

	pub fn get_generic_info(&self) -> (Vec<Box<Type>>, Vec<usize>) {
		if let Type::Generic(_, types, sizes) = self {
			return (types.clone(), sizes.clone())
		}

		return self.get_inner_type().get_generic_info();
	}

	pub fn get_generic(&self, storage: &TypeStorage) -> RawType {
		if let Type::Generic(raw, _, _) = self {
			return storage.types.get_ind(*raw).clone();
		};

		if let Type::GenericLowered(raw) = self {
			return raw.clone();
		}

		return self.get_inner_type().get_generic(storage);
	}

	pub fn get_function(&self, storage: &TypeStorage, hash: u64) -> BaseResult<(usize, TypedFunction)> {
		return match self.get_generic(storage) {
			RawType::Struct(_, container) => Ok((container.get_function_hash(hash, storage)?, container.get_function(hash, storage)?)),
			RawType::Enum(container) => Ok((container.get_function_hash(hash, storage)?, container.get_function(hash, storage)?)),
			RawType::EnumEntry(container) => Ok((container.get_function_hash(hash, storage)?, container.get_function(hash, storage)?)),
			_ => Err(BaseError::err("This cannot contain functions!".to_string()))
		};
	}

	pub fn get_field(&self, storage: &TypeStorage, hash: u64) -> BaseResult<(usize, TypeReference)> {
		return match self.get_generic(storage) {
			RawType::Struct(_, container) => Ok((container.get_field_hash(hash, storage)?, container.get_field(hash, storage)?)),
			RawType::Enum(container) => Ok((container.get_field_hash(hash, storage)?, container.get_field(hash, storage)?)),
			RawType::EnumEntry(container) => Ok((container.get_field_hash(hash, storage)?, container.get_field(hash, storage)?)),
			_ => Err(BaseError::err("This cannot contain fields!".to_string()))
		}
	}

	pub fn get_fields(&self, storage: &TypeStorage) -> BaseResult<Vec<u64>> {
		return match self.get_generic(storage) {
			RawType::Struct(_, container) => Ok(container.get_fields(storage)),
			RawType::EnumEntry(container) => Ok(container.get_fields(storage)),
			_ => Err(BaseError::err("This cannot contain fields!".to_string()))
		}
	}
	
	pub fn get_functions(&self, storage: &TypeStorage) -> BaseResult<Vec<u64>> {
		return match self.get_generic(storage) {
			RawType::Struct(_, container) => Ok(container.get_functions(storage)),
			RawType::EnumEntry(container) => Ok(container.get_functions(storage)),
			RawType::Enum(container) => Ok(container.get_functions(storage)),
			_ => Err(BaseError::err("This cannot contain fields!".to_string()))
		}
	}

}

impl SizedType for Type {
	fn get_size(&self, t: &Type, compacted_size: bool, storage: &TypeStorage) -> usize {
		return match self {
			Self::Array(size, inner) => inner.clone().get_size(t, compacted_size, storage) * *size,
			Self::Pointer(_, _) => get_pointer_size(),
			Self::Generic(e, _, _) => storage.types.vals[*e].get_size(t, compacted_size, storage), 
			Self::GenericLowered(e) => e.get_size(t, compacted_size, storage)
		}
	}
}