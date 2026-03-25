//!
//! AST tree related definitions.
//! 

use std::collections::HashMap;

use compiler_typing::TypeParameterContainer;
use compiler_utils::{Position, hash::{HashedString, SelfHash}};
use lexer::{toks::{comp::ComparingOperator, math::MathOperator}};

use crate::types::ASTType;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclarationArgument {
    pub name: HashedString,
    pub argument_type: ASTType
}

impl FunctionDeclarationArgument {
    pub fn new(name: String, arg_type: ASTType) -> Self {
        FunctionDeclarationArgument { name: HashedString::new(name), argument_type: arg_type }
    }
}

/// The main AST node type in the AST parsing system.
#[derive(Debug, PartialEq, Clone)]
pub enum ASTTreeNodeKind {
    IntegerLit { val: i128, hash: u64 },
    StringLit(String),

	ThisStructParam,

	OperatorBasedConditionMember { lval: Box<ASTTreeNode>, rval: Box<ASTTreeNode>, operator: ComparingOperator },
	BooleanBasedConditionMember { val: Box<ASTTreeNode>, negate: bool },

	MathResult { lval: Box<ASTTreeNode>, rval: Box<ASTTreeNode>, operator: MathOperator, assigns: bool },

	VariableReference(HashedString),

	StructVariableInitializerValue { struct_type: ASTType, map: HashMap<SelfHash, Box<ASTTreeNode>> },
	ArrayVariableInitializerValueSameValue { size: usize, v: Box<ASTTreeNode> },
	ArrayVariableInitializerValue { vals: Vec<Box<ASTTreeNode>> },

	ArrayIndexAccess { val: Box<ASTTreeNode>, index: Box<ASTTreeNode> },

	StructLayoutDeclaration { name: HashedString, layout: bool, members: Vec<Box<ASTTreeNode>>, type_params: TypeParameterContainer },
	StructFieldMember { name: HashedString, member_type: ASTType },

    VarDeclaration { var_name: HashedString, var_type: ASTType, value: Option<Box<ASTTreeNode>> },
    VarValueChange { var: Box<ASTTreeNode>, value: Box<ASTTreeNode> },
	VarIncrement { var: Box<ASTTreeNode>, increment_by: Option<Box<ASTTreeNode>> }, // Default is by 1

	IfStatement { cond: Box<ASTTreeNode>, body: Vec<Box<ASTTreeNode>>, branches: Vec<Box<ASTTreeNode>>, depth: usize },
	IfElseStatement { cond: Option<Box<ASTTreeNode>>, body: Vec<Box<ASTTreeNode>> },
	ElseStatement { body: Vec<Box<ASTTreeNode>> },

	ReturnStatement { val: Option<Box<ASTTreeNode>> },

	StaticVariableDeclaration { name: HashedString, var_type: ASTType, val: Box<ASTTreeNode> },

	WhileBlock { cond: Box<ASTTreeNode>, body: Vec<Box<ASTTreeNode>> },
	ForBlock { initial_state: Box<ASTTreeNode>, cond: Box<ASTTreeNode>, increment: Box<ASTTreeNode>, body: Vec<Box<ASTTreeNode>> },

    FunctionCall { func: HashedString, args: Vec<Box<ASTTreeNode>>  },
    FunctionDeclaration { func_name: HashedString, args: Vec<FunctionDeclarationArgument>, body: Vec<Box<ASTTreeNode>>, return_type: Option<ASTType>, requires_this: bool },

	ShadowFunctionDeclaration { func_name: HashedString, args: Vec<FunctionDeclarationArgument>, return_type: Option<ASTType> },

	StructLRVariable { l: Box<ASTTreeNode>, r: Box<ASTTreeNode>,},
	StructLRFunction { l: Box<ASTTreeNode>, r: Box<ASTTreeNode>, }
}

impl ASTTreeNodeKind {
	pub fn is_function_call(&self) -> bool {
		return matches!(self, ASTTreeNodeKind::FunctionCall { .. } | ASTTreeNodeKind::StructLRFunction { .. } )
	}

	pub fn is_var_access(&self) -> bool {
		return matches!(self, ASTTreeNodeKind::VariableReference { .. } | ASTTreeNodeKind::StructLRVariable { .. })
	}

	pub fn is_tree_permissible(&self) -> bool {
		return matches!(self, ASTTreeNodeKind::FunctionDeclaration { .. } | ASTTreeNodeKind::StaticVariableDeclaration { .. } | ASTTreeNodeKind::ShadowFunctionDeclaration { .. }| ASTTreeNodeKind::StructLayoutDeclaration { .. })
	}

	pub fn get_tree_name(&self) -> Option<HashedString> {
		match self {
			ASTTreeNodeKind::FunctionDeclaration { func_name, args: _, body: _, return_type: _ , requires_this: _} => {
				return Some(HashedString::new(func_name.val.to_string()));
			},

			ASTTreeNodeKind::ShadowFunctionDeclaration { func_name, args: _, return_type: _ } => {
				return Some(HashedString::new(func_name.val.to_string()))
			}

			ASTTreeNodeKind::StaticVariableDeclaration { name, var_type: _, val: _ } => {
				return Some(HashedString::new(name.val.clone()));
			},

			ASTTreeNodeKind::StructLayoutDeclaration { name, layout: _, members: _, type_params: _ } => {
				return Some(HashedString::new(name.val.to_string()));
			},

			ASTTreeNodeKind::VarDeclaration { var_name, var_type: _, value: _ } => {
				return Some(HashedString::new(var_name.val.to_string()));
			},

			_ => return None
		}
	}
}

/// The complete AST tree node. Contains positions and more.
#[derive(Debug, PartialEq, Clone)]
pub struct ASTTreeNode {
	pub kind: ASTTreeNodeKind,
	pub start: Position,
	pub end: Position
}

impl ASTTreeNode {
	pub fn new(kind: ASTTreeNodeKind, start: Position, end: Position) -> Self {
		return ASTTreeNode { kind, start, end }
	}
}

#[macro_export]
macro_rules! make_node {
	($kind:expr, $s:expr, $e:expr) => {
		Box::new(ASTTreeNode::new($kind, $s.pos.clone(), $e.get_end_pos()))
	};
}