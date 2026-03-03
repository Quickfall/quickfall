//!
//! AST tree related definitions.
//! 

use compiler_utils::Position;
use lexer::{toks::{comp::ComparingOperator, math::MathOperator}};
use compiler_utils::hash::{TypeHash, WithHash};

use crate::types::CompleteType;

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
pub enum ASTTreeNodeKind {
    IntegerLit { val: i128, hash: u64 },
    StringLit(String),

	OperatorBasedConditionMember { lval: Box<ASTTreeNode>, rval: Box<ASTTreeNode>, operator: ComparingOperator },
	BooleanBasedConditionMember { val: Box<ASTTreeNode>, negate: bool },

	MathResult { lval: Box<ASTTreeNode>, rval: Box<ASTTreeNode>, operator: MathOperator, assigns: bool },

	VariableReference(WithHash<String>),

	StructLayoutDeclaration { name: WithHash<String>, layout: bool, members: Vec<Box<ASTTreeNode>> },
	StructFieldMember { name: WithHash<String>, member_type: CompleteType },

    VarDeclaration { var_name: WithHash<String>, var_type: CompleteType, value: Option<Box<ASTTreeNode>> },
    VarValueChange { var: Box<ASTTreeNode>, value: Box<ASTTreeNode> },
	VarIncrement { var: Box<ASTTreeNode>, increment_by: Option<Box<ASTTreeNode>> }, // Default is by 1

	IfStatement { cond: Box<ASTTreeNode>, body: Vec<Box<ASTTreeNode>>, branches: Vec<Box<ASTTreeNode>>, depth: usize },
	IfElseStatement { cond: Option<Box<ASTTreeNode>>, body: Vec<Box<ASTTreeNode>> },
	ElseStatement { body: Vec<Box<ASTTreeNode>> },

	ReturnStatement { val: Option<Box<ASTTreeNode>> },

	StaticVariableDeclaration { name: WithHash<String>, var_type: CompleteType, val: Box<ASTTreeNode> },

	WhileBlock { cond: Box<ASTTreeNode>, body: Vec<Box<ASTTreeNode>> },
	ForBlock { initial_state: Box<ASTTreeNode>, cond: Box<ASTTreeNode>, increment: Box<ASTTreeNode>, body: Vec<Box<ASTTreeNode>> },

    FunctionCall { func: WithHash<String>, args: Vec<Box<ASTTreeNode>>  },
    FunctionDeclaration { func_name: WithHash<String>, args: Vec<FunctionDeclarationArgument>, body: Vec<Box<ASTTreeNode>>, return_type: Option<CompleteType> },

	ShadowFunctionDeclaration { func_name: WithHash<String>, args: Vec<FunctionDeclarationArgument>, return_type: Option<CompleteType> },

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

	pub fn get_tree_name(&self) -> Option<WithHash<String>> {
		match self {
			ASTTreeNodeKind::FunctionDeclaration { func_name, args: _, body: _, return_type: _ } => {
				return Some(WithHash::new(func_name.val.to_string()));
			},

			ASTTreeNodeKind::ShadowFunctionDeclaration { func_name, args: _, return_type: _ } => {
				return Some(WithHash::new(func_name.val.to_string()))
			}

			ASTTreeNodeKind::StaticVariableDeclaration { name, var_type: _, val: _ } => {
				return Some(WithHash::new(name.val.clone()));
			},

			ASTTreeNodeKind::StructLayoutDeclaration { name, layout: _, members: _ } => {
				return Some(WithHash::new(name.val.to_string()));
			},

			ASTTreeNodeKind::VarDeclaration { var_name, var_type: _, value: _ } => {
				return Some(WithHash::new(var_name.val.to_string()));
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