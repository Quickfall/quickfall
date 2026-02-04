//!
//! AST tree related definitions.
//! 

use utils::hash::{TypeHash, WithHash};

use crate::ast::cond::operators::ConditionOperator;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclarationArgument {
    pub name: WithHash<String>,
    pub argumentType: TypeHash
}

impl FunctionDeclarationArgument {
    pub fn new(name: String, argType: TypeHash) -> Self {
        FunctionDeclarationArgument { name: WithHash::new(name), argumentType: argType }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTTreeNode {
    IntegerLit(i64),
    StringLit(String),

	OperatorBasedConditionMember { lval: Box<ASTTreeNode>, rval: Box<ASTTreeNode>, operator: ConditionOperator },
	BooleanBasedConditionMember { val: Box<ASTTreeNode>, negate: bool },

	VariableReference(WithHash<String>),

    VarDeclaration { varName: WithHash<String>, varType: TypeHash, value: Option<Box<ASTTreeNode>> },
    VarValueChange { var: Box<ASTTreeNode>, value: Box<ASTTreeNode> },

    Return { value: Option<Box<ASTTreeNode>> },

    FunctionCall { func: WithHash<String>, args: Vec<Box<ASTTreeNode>>  },
    FunctionDeclaration { funcName: WithHash<String>, args: Vec<FunctionDeclarationArgument>, body: Vec<Box<ASTTreeNode>> },
	
	StructLRVariable { l: Box<ASTTreeNode>, r: Box<ASTTreeNode>,},
	StructLRFunction { l: Box<ASTTreeNode>, r: Box<ASTTreeNode>, }
}

impl ASTTreeNode {
	pub fn is_function_call(&self) -> bool {
		return !matches!(self, ASTTreeNode::FunctionCall { .. } | ASTTreeNode::StructLRFunction { .. } )
	}

	pub fn is_var_access(&self) -> bool {
		return !matches!(self, ASTTreeNode::VariableReference { .. } | ASTTreeNode::StructLRVariable { .. })
	}

}