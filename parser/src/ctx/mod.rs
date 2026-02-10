use std::{collections::HashMap};

use crate::ast::tree::ASTTreeNode;

pub struct ParserCtx {
	pub map: HashMap<String, Box<ASTTreeNode>> 
}

impl ParserCtx {
	pub fn new() -> Self {
		return ParserCtx { map: HashMap::new() }
	}

	pub fn insert(&mut self, name: String, node: Box<ASTTreeNode>) -> bool {
		if !node.is_tree_permissible() {
			return false;
		}

		self.map.insert(name, node);
		return true;
	}

}