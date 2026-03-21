//! The typing system used all across the compiler in all levels of AstoIR and AST.

use crate::{references::TypeReference, tree::Type};

pub mod tree;
pub mod raw;
pub mod structs;
pub mod enums;
pub mod references;
pub mod utils;
pub mod storage;
pub mod stated;

/// A function contained within a type.
pub type TypedFunction = (Vec<TypeReference>, Option<TypeReference>);

pub type RawTypeReference = usize;

/// Represents a basic type that has a size. 
pub trait SizedType {
	/// Obtains the size of the type. The `compacted_size` parameter determines if the compacted size should be returned or not
	fn get_size(&self, t: &Type, compacted_size: bool) -> usize;
}
