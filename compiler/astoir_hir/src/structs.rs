//! AstoIR HIR structures related to HIR nodes

use crate::nodes::HIRNode;

pub enum StructLRUStep {
	FunctionCall { func: usize, args: Vec<Box<HIRNode>> },
	VariableStep { variable: usize }
}

pub enum HIRIfBranch {
	IfBranch { cond: Box<HIRNode>, body: Vec<Box<HIRNode>> },
	ElseIfBranch { cond: Box<HIRNode>, body: Vec<Box<HIRNode>> },
	ElseBranch { body: Vec<Box<HIRNode>> }
}