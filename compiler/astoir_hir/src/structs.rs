//! AstoIR HIR structures related to HIR nodes

use crate::nodes::HIRNode;

#[derive(Debug)]
pub enum StructLRUStep {
	FunctionCall { func: usize, args: Vec<Box<HIRNode>> },
	VariableStep { variable: usize }
}

#[derive(Debug)]
pub enum HIRIfBranch {
	IfBranch { cond: Box<HIRNode>, body: Vec<Box<HIRNode>> },
	ElseIfBranch { cond: Box<HIRNode>, body: Vec<Box<HIRNode>> },
	ElseBranch { body: Vec<Box<HIRNode>> }
}

#[derive(Debug)]
pub struct HIRStructContainer {
	pub function_impls: Vec<Box<HIRNode>>
}

