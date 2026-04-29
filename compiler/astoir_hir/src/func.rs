//! Definitions for HIR functions

use compiler_utils::{hash::HashedString, storage::Storage};
use typing::{constraints::TypeParameter, container::Type};

use crate::{context::local::BranchedContext, nodes::HIRNode};

pub struct HIRFunction {
	pub name: HashedString,

	pub type_parameters: Storage<TypeParameter>,

	pub return_type: Option<Type>,
	pub arguments: Vec<(HashedString, Type)>,

	pub ctx: Option<BranchedContext>,
	pub implementation: Option<Box<HIRNode>>
}

impl HIRFunction {
	pub fn new_shadow(name: String, return_type: Option<Type>, arguments: Vec<(HashedString, Type)>) -> Self {
		HIRFunction { name: HashedString::new(name), type_parameters: Storage::new(), return_type, arguments, ctx: None, implementation: None }
	}

	pub fn new_pre_full(name: String, return_type: Option<Type>, arguments: Vec<(HashedString, Type)>, ctx: BranchedContext) -> Self {
		HIRFunction { name: HashedString::new(name), type_parameters: Storage::new(), return_type, arguments, ctx: Some(ctx), implementation: None }
	}

	pub fn new_full(name: String, return_type: Option<Type>, arguments: Vec<(HashedString, Type)>, ctx: BranchedContext, implementation: Box<HIRNode>) -> Self {
		HIRFunction { name: HashedString::new(name), type_parameters: Storage::new(), return_type, arguments, ctx: Some(ctx), implementation: Some(implementation) }
	}
}