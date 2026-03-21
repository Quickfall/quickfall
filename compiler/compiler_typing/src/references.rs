use crate::tree::Type;

/// References a type from two states: resolved and unresolved. Allows for type parameters
#[derive(Clone)]
pub enum TypeReference {
	Resolved(Type),
	
	/// Respresents the index of the type parameter
	Unresolved(usize)
}
