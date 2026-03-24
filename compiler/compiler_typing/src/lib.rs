//! The typing system used all across the compiler in all levels of AstoIR and AST.

use std::collections::HashMap;

use compiler_errors::errs::BaseResult;
use compiler_utils::{hash::HashedString, utils::indexed::IndexStorage};

use crate::{references::TypeReference, storage::TypeStorage, tree::Type};

pub mod tree;
pub mod raw;
pub mod structs;
pub mod enums;
pub mod references;
pub mod utils;
pub mod storage;
pub mod stated;

/// A function contained within a type.
pub type TypedFunction = (Vec<(u64, TypeReference)>, Option<TypeReference>);

pub type TypeParameterContainer = HashMap<HashedString, usize>;

pub type RawTypeReference = usize;

/// Represents a basic type that has a size. 
pub trait SizedType {
	/// Obtains the size of the type. The `compacted_size` parameter determines if the compacted size should be returned or not
	fn get_size(&self, t: &Type, compacted_size: bool, storage: &TypeStorage) -> usize;
}

/// Represents types that can contain functions and more
pub trait StructuredType {
	fn get_function(&self, hash: u64) -> BaseResult<TypedFunction>;
	fn get_function_hash(&self, hash: u64) -> BaseResult<usize>;
	fn get_field(&self, hash: u64) -> BaseResult<TypeReference>;
	fn get_field_hash(&self, hash: u64) -> BaseResult<usize>;
}