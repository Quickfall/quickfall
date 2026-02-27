//! The context definitions for the AstoIR HIR layer.

use std::collections::HashMap;

use compiler_utils::hash::SelfHash;

pub struct HIRBranchContext {
	pub hash_to_ind: HashMap<SelfHash, usize>,
	// TODO: add types there
}

impl HIRBranchContext {
	pub fn new() -> Self {
		HIRBranchContext { hash_to_ind: HashMap::new() }
	}
}