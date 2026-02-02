//!
//! AST tree related definitions.
//! 

use utils::hash::{TypeHash, WithHash};

use crate::ast::cond::operators::ConditionOperator;

#[derive(Debug)]
pub struct FunctionDeclarationArgument {
    pub name: WithHash<String>,
    pub argumentType: TypeHash
}

impl FunctionDeclarationArgument {
    pub fn new(name: String, argType: TypeHash) -> Self {
        FunctionDeclarationArgument { name: WithHash::new(name), argumentType: argType }
    }
}

#[derive(Debug)]
pub enum ASTTreeNode {
    IntegerLit(i64),
    StringLit(String),

	ConditionSeperator,
	OperatorBasedConditionMember { lval: Box<ASTTreeNode>, rval: Box<ASTTreeNode>, operator: ConditionOperator },
	BooleanBasedConditionMember { val: Box<ASTTreeNode>, negate: bool },

	RepresentsElement { elementName: WithHash<String> },

    VarDeclaration { varName: WithHash<String>, varType: TypeHash, value: Option<Box<ASTTreeNode>> },
    VarValueChange { var: Box<ASTTreeNode>, value: Box<ASTTreeNode> },

    Return { value: Option<Box<ASTTreeNode>> },

    FunctionCall { func: Box<ASTTreeNode>, args: Vec<Box<ASTTreeNode>>  },
    FunctionDeclaration { funcName: WithHash<String>, args: Vec<FunctionDeclarationArgument>, body: Vec<Box<ASTTreeNode>> },
	
	StructBasedVariableRepresentation { steps: Vec<Box<ASTTreeNode>>, varName: WithHash<String> },
	StructBasedFunctionRepresentaztion { steps: Vec<Box<ASTTreeNode>>, funcName: WithHash<String> }
}