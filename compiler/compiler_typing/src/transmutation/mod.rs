//! Handling for type transmutations here.

use crate::{storage::TypeStorage, tree::Type};

pub mod array;

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

			(Self::Reference(inner), Self::Pointer(array, inner2)) => {
				return !*array && inner.can_transmute(inner2, storage);
			}

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

				return raw_type.can_transmute(sizes.clone(), raw_type2, sizes2.clone())
			},

			(Self::GenericLowered(base), Self::Generic(rawtype, type_params, sizes)) => {
				if !type_params.is_empty() {
					return false;
				}

				return base.can_transmute(vec![], rawtype, sizes.clone())
			},

			(Self::Generic(rawtype, type_params, sizes), Self::GenericLowered(base)) => {
				if !type_params.is_empty() {
					return false;
				}

				return base.can_transmute(vec![], rawtype, sizes.clone())
			}

			(Self::GenericLowered(base), Self::GenericLowered(base2)) => {
				return base.can_transmute(vec![], base2, vec![]);
			}


			_ => false
		}
	}
}