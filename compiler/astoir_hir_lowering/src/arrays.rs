use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode};
use compiler_errors::{INVALID_EXPR, IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};
use compiler_typing::{raw::RawType, tree::Type};

use crate::{values::lower_ast_value, var::lower_ast_variable_reference};

pub fn lower_ast_array_index_access(context: &mut HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::ArrayIndexAccess { val, index } = node.kind {
		let array = lower_ast_value(context, curr_ctx, val)?;

		if !array.get_node_type(context, curr_ctx).unwrap().can_use_index_access() {
			return Err(CompilerError::from_ast(ErrorKind::Error, INVALID_EXPR!().to_string(), &node.start, &node.end));
		}

		let index = match lower_ast_value(context, curr_ctx, index)?.use_as(context, curr_ctx, Type::GenericLowered(RawType::Integer(32, false))) {
			Ok(v) => Box::new(v),
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		return Ok(Box::new(HIRNode::ArrayIndexAccess { val: array, index }));
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end));
}

pub fn lower_ast_array_modify(context: &mut HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::ArrayIndexModifiy { array, index, val } = node.kind {
		let array = lower_ast_variable_reference(context, curr_ctx, array, true)?;

		let index = match lower_ast_value(context, curr_ctx, index)?.use_as(context, curr_ctx, Type::GenericLowered(RawType::Integer(32, false))) {
			Ok(v) => Box::new(v),
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		let new_val = match lower_ast_value(context, curr_ctx, val)?.use_as(context, curr_ctx, *array.get_node_type(context, curr_ctx).unwrap().get_inner_type()) {
			Ok(v) => Box::new(v),
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		return Ok(Box::new(HIRNode::ArrayIndexModify { array, index, new_val }))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end));
}