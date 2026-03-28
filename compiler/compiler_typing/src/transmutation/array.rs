//! Handles transmutation for arrays.
//! 
//! # Behavior
//! Instead of doing a rough transmutation of the array type, Quickfall will try to transmute the types inside of the array if they can
//! be transmutated without a `Cast` node. If that transmutation can happen, Quickfall will directly try to.

use crate::{storage::TypeStorage, tree::Type};

/// Utility function to determinate if an array type can be transmutated by the inner type. 
/// This determines if we use the array strategy or a cast node.
pub fn can_transmute_inner(array_type: &Type, new_type: &Type, storage: &TypeStorage) -> bool {
	if !array_type.can_use_index_access() || !new_type.can_use_index_access() {
		panic!("Either ones of the types sent when using can_transmute_inner were not actual array types!")
	}

	return array_type.get_inner_type().can_transmute(&new_type.get_inner_type(), storage)
}