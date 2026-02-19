use std::{collections::HashMap};

use crate::ast::tree::ASTTreeNode;

#[derive(Debug)]
pub struct ParserCtx {
	pub map: HashMap<String, Box<ASTTreeNode>>, 
	pub iter_order: Vec<String>
}

impl ParserCtx {
	pub fn new() -> Self {
		return ParserCtx { map: HashMap::new(), iter_order: Vec::new() }
	}

	pub fn insert(&mut self, name: String, node: Box<ASTTreeNode>) -> bool {
		if !node.is_tree_permissible() {
			return false;
		}

		self.iter_order.push(name.clone());

		self.map.insert(name, node);
		return true;
	}

}