//! The typing system used all across the compiler in all levels of AstoIR and AST.

use crate::tree::Type;

pub mod tree;
pub mod raw;
pub mod structs;
pub mod enums;

/// A function contained within a type.
pub type TypedFunction = (Vec<TypeReference>, Option<TypeReference>);

pub type RawTypeReference = usize;

/// References a type from two states: resolved and unresolved. Allows for type parameters
#[derive(Clone)]
pub enum TypeReference {
	Resolved(Type),
	
	/// Respresents the index of the type parameter
	Unresolved(usize)
}