//!
//! AST tree related definitions.
//! 

use lexer::toks::{comp::ComparingOperator, math::MathOperator};
use utils::hash::{TypeHash, WithHash};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclarationArgument {
    pub name: WithHash<String>,
    pub argument_type: TypeHash
}

impl FunctionDeclarationArgument {
    pub fn new(name: String, arg_type: TypeHash) -> Self {
        FunctionDeclarationArgument { name: WithHash::new(name), argument_type: arg_type }
    }
}

/// The main AST node type in the AST parsing system.
#[derive(Debug, PartialEq, Clone)]
pub enum ASTTreeNode {
    IntegerLit(i64),
    StringLit(String),

	OperatorBasedConditionMember { lval: Box<ASTTreeNode>, rval: Box<ASTTreeNode>, operator: ComparingOperator },
	BooleanBasedConditionMember { val: Box<ASTTreeNode>, negate: bool },

	MathResult { lval: Box<ASTTreeNode>, rval: Box<ASTTreeNode>, operator: MathOperator, assigns: bool },

	VariableReference(WithHash<String>),

	StructLayoutDeclaration { name: WithHash<String>, layout: bool, members: Vec<Box<ASTTreeNode>> },
	StructFieldMember { name: WithHash<String>, member_type: TypeHash },

    VarDeclaration { var_name: WithHash<String>, var_type: TypeHash, value: Option<Box<ASTTreeNode>> },
    VarValueChange { var: Box<ASTTreeNode>, value: Box<ASTTreeNode> },
	VarIncrement { var: Box<ASTTreeNode>, increment_by: Option<Box<ASTTreeNode>> }, // Default is by 1

	IfStatement { cond: Box<ASTTreeNode>, body: Vec<Box<ASTTreeNode>>, else_statement: Option<Box<ASTTreeNode>>, depth: usize },
	IfElseStatement { cond: Option<Box<ASTTreeNode>>, body: Vec<Box<ASTTreeNode>>, else_statement: Option<Box<ASTTreeNode>> },
	ElseStatement { body: Vec<Box<ASTTreeNode>> },

	ReturnStatement { val: Option<Box<ASTTreeNode>> },

	WhileBlock { cond: Box<ASTTreeNode>, body: Vec<Box<ASTTreeNode>> },
	ForBlock { initial_state: Box<ASTTreeNode>, cond: Box<ASTTreeNode>, increment: Box<ASTTreeNode>, body: Vec<Box<ASTTreeNode>> },

    FunctionCall { func: WithHash<String>, args: Vec<Box<ASTTreeNode>>  },
    FunctionDeclaration { func_name: WithHash<String>, args: Vec<FunctionDeclarationArgument>, body: Vec<Box<ASTTreeNode>>, returnType: Option<TypeHash> },
	
	StructLRVariable { l: Box<ASTTreeNode>, r: Box<ASTTreeNode>,},
	StructLRFunction { l: Box<ASTTreeNode>, r: Box<ASTTreeNode>, }
}

impl ASTTreeNode {
	pub fn is_function_call(&self) -> bool {
		return matches!(self, ASTTreeNode::FunctionCall { .. } | ASTTreeNode::StructLRFunction { .. } )
	}

	pub fn is_var_access(&self) -> bool {
		return matches!(self, ASTTreeNode::VariableReference { .. } | ASTTreeNode::StructLRVariable { .. })
	}

	pub fn is_tree_permissible(&self) -> bool {
		return matches!(self, ASTTreeNode::FunctionDeclaration { .. } | ASTTreeNode::StructLayoutDeclaration { .. })
	}

	pub fn get_tree_name(&self) -> Option<WithHash<String>> {
		match self {
			ASTTreeNode::FunctionDeclaration { func_name, args, body, returnType } => {
				return Some(WithHash::new(func_name.val.to_string()));
			},

			ASTTreeNode::StructLayoutDeclaration { name, layout, members } => {
				return Some(WithHash::new(name.val.to_string()));
			},

			ASTTreeNode::VarDeclaration { var_name, var_type, value } => {
				return Some(WithHash::new(var_name.val.to_string()));
			},

			_ => return None
		}
	}

}