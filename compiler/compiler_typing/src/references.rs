use diagnostics::unsure_panic;

use crate::tree::Type;

/// References a type from two states: resolved and unresolved. Allows for type parameters
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeReference {
	Resolved(Type),
	
	/// Respresents the index of the type parameter
	Unresolved(usize)
}

impl TypeReference {
	pub fn make_resolved(t: Type) -> Self {
		Self::Resolved(t)
	}

	pub fn make_unresolved(index: usize) -> Self {
		Self::Unresolved(index)
	}

	pub fn is_resolved(&self) -> bool {
		if let Self::Resolved(_) = self {
			return true;
		}

		return false;
	}

	/// Attempts to cast the type reference into a concrete Type. Will not try to resolve the type from the argument params
	pub fn as_resolved(self) -> Type {
		if let TypeReference::Resolved(val) = self {
			return val;
		}

		unsure_panic!("used as_resolved on a non resolved type")
	}

	/// Attempts to resolve the type reference into a concrete `Type`. 
	pub fn resolve(self, t: &Type) -> Type {
		match &self {
			TypeReference::Resolved(val) => return val.clone(),
			TypeReference::Unresolved(index) => {
				let concrete_info = t.get_generic_info();

				return concrete_info.0[*index].as_ref().clone()
			}
		}

		
	}
	
}