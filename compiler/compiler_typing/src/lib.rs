//! The typing system used all across the compiler in all levels of AstoIR and AST.

use std::collections::HashMap;

use compiler_global_scope::GlobalScopeStorage;
use compiler_utils::{hash::HashedString};
use diagnostics::DiagnosticResult;

use crate::{raw::RawType, references::TypeReference, storage::TypeStorage, tree::Type};

pub mod tree;
pub mod raw;
pub mod structs;
pub mod enums;
pub mod references;
pub mod utils;
pub mod storage;
pub mod transmutation;
pub mod bounds;

pub type TypedGlobalScope = GlobalScopeStorage<Type, RawType>;

/// A function contained within a type.
pub type TypedFunction = (Vec<(u64, TypeReference)>, Option<TypeReference>);
pub type TypedResolvedFunction = (Vec<(u64, Type)>, Option<Type>);

pub type TypeParameterContainer = HashMap<HashedString, usize>;

pub type RawTypeReference = usize;

/// Represents a basic type that has a size. 
pub trait SizedType {
	/// Obtains the size of the type. The `compacted_size` parameter determines if the compacted size should be returned or not
	fn get_size(&self, t: &Type, compacted_size: bool, storage: &TypedGlobalScope) -> usize;
}

/// Represents a type that contains type parameters
pub trait TypeParamType {
	/// Checks if the type contains the given type parameter
	fn has_type_param(&self, param: &HashedString) -> bool;
	
	/// Gets the raw index for the given type parameter
	fn get_type_param_ind(&self, param: &HashedString) -> usize;
}

/// Represents types that can contain functions and more
pub trait StructuredType {
	#[must_use = "Must set the diagnostic position beforehand"]
	fn get_function(&self, hash: u64, storage: &TypedGlobalScope) -> DiagnosticResult<TypedFunction>;

	#[must_use = "Must set the diagnostic position beforehand"]
	fn get_function_hash(&self, hash: u64, storage: &TypedGlobalScope) -> DiagnosticResult<usize>;

	#[must_use = "Must set the diagnostic position beforehand"]
	fn get_field(&self, hash: u64, storage: &TypedGlobalScope) -> DiagnosticResult<TypeReference>;

	#[must_use = "Must set the diagnostic position beforehand"]
	fn get_field_hash(&self, hash: u64, storage: &TypedGlobalScope) -> DiagnosticResult<usize>;

	#[must_use = "Must set the diagnostic position beforehand"]
	fn get_fields(&self, storage: &TypedGlobalScope) -> Vec<u64>;

	#[must_use = "Must set the diagnostic position beforehand"]
	fn get_functions(&self, storage: &TypedGlobalScope) -> Vec<u64>;
}