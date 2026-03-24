use ast::{tree::{ASTTreeNode, ASTTreeNodeKind}};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext, VariableKind, get_variable}, nodes::HIRNode};
use compiler_errors::{IR_INVALID_NODE_TYPE, VARIABLE_REQ_VALUE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::{types::lower_ast_type, values::lower_ast_value};

pub fn lower_ast_variable_declaration(context: &HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::VarDeclaration { var_name, var_type, value} = node.kind.clone() {
		let lowered = match lower_ast_type(context, var_type) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		let name_ind = match curr_ctx.introduce_variable(var_name.hash, lowered.clone(), value.is_some()) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		let default_val;

		if value.is_some() {
			let hir_val = match lower_ast_value(context, curr_ctx, value.unwrap())?.use_as(context, curr_ctx, lowered.clone()) {
				Ok(v) => Box::new(v),
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			};

			default_val = Some(hir_val);
		} else {
			default_val = None;
		}

	

		return Ok(Box::new(HIRNode::VarDeclaration { variable: name_ind, var_type: lowered, default_val}))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_variable_reference(context: &HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>, requires_value: bool) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::VariableReference(str) = node.kind.clone() {		
		let var = match get_variable(context, curr_ctx, str.hash) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		if var.0 == VariableKind::STATIC {
			return Ok(Box::new(HIRNode::VariableReference { index: var.2, is_static: true }))
		} 

		if requires_value {
			if !curr_ctx.has_variable_value(var.2) {
				return Err(CompilerError::from_ast(ErrorKind::Error, VARIABLE_REQ_VALUE!().to_string(), &node.start, &node.end))
			}
		}

		return Ok(Box::new(HIRNode::VariableReference { index: var.2, is_static: false }))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_variable_assign(context: &HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::VarValueChange { var, value } = node.kind.clone() {
		let value = lower_ast_value(context, curr_ctx, value)?;
		let variable_reference = lower_ast_variable_reference(context, curr_ctx, var, false)?;

		let var = match variable_reference.as_variable_reference() {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		if !var.1 {
			curr_ctx.introduce_variable_assign(var.0);
		}

		return Ok(Box::new(HIRNode::VarAssigment { variable: var.0, val: value }))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}