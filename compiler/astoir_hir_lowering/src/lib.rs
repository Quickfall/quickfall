use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::var::lower_ast_variable_declaration;

pub mod literals;
pub mod var;
pub mod types;
pub mod values;
pub mod func;

pub fn lower_ast_body_node(context: &HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	match node.kind {
		ASTTreeNodeKind::VarDeclaration { .. } => return lower_ast_variable_declaration(context, curr_ctx, node),
		
		_ => return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
	}
}