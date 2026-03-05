use ast::{tree::{ASTTreeNode, ASTTreeNodeKind}};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext, VariableKind, get_variable}, nodes::HIRNode};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::types::lower_ast_type;

pub fn lower_ast_variable_declaration(context: &HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::VarDeclaration { var_name, var_type, value } = node.kind.clone() {
		let lowered = match lower_ast_type(context, var_type) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		let name_ind = match curr_ctx.introduce_variable(var_name.hash, lowered.clone()) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		return Ok(Box::new(HIRNode::VarDeclaration { variable: name_ind, var_type: lowered, default_val: None}))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_variable_reference(context: &HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::VariableReference(str) = node.kind.clone() {
		let var = match get_variable(context, curr_ctx, str.hash) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		if var.0 == VariableKind::STATIC {
			return Ok(Box::new(HIRNode::VariableReference { index: var.2, is_static: true }))
		} 

		return Ok(Box::new(HIRNode::VariableReference { index: var.2, is_static: false }))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}