use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::{HIRNode, HIRNodeKind}};
use compiler_errors::{INVALID_EXPR, IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};
use compiler_typing::{raw::RawType, tree::Type};
use diagnostics::{DiagnosticResult, builders::make_index_usage};

use crate::{values::lower_ast_value, var::lower_ast_variable_reference};

pub fn lower_ast_array_index_access(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::ArrayIndexAccess { val, index } = node.kind {
		let array = lower_ast_value(context, curr_ctx, val)?;

		if !array.get_node_type(context, curr_ctx).unwrap().can_use_index_access() {
			return Err(make_index_usage(&node, &array.get_node_type(context, curr_ctx).unwrap()).into())
		}

		let index = Box::new(lower_ast_value(context, curr_ctx, index)?.use_as(context, curr_ctx, Type::GenericLowered(RawType::Integer(32, false)), &node, None)?);

		return Ok(Box::new(HIRNode::new(HIRNodeKind::ArrayIndexAccess { val: array, index }, &node.start, &node.end)));
	}

	panic!("Invalid node type")
}

pub fn lower_ast_array_modify(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::ArrayIndexModifiy { array, index, val } = node.kind {
		let array = lower_ast_variable_reference(context, curr_ctx, array, true)?;

		let index = Box::new(lower_ast_value(context, curr_ctx, index)?.use_as(context, curr_ctx, Type::GenericLowered(RawType::Integer(32, false)), &node, None)?);

		let new_val = Box::new(lower_ast_value(context, curr_ctx, val)?.use_as(context, curr_ctx, *array.get_node_type(context, curr_ctx).unwrap().get_inner_type(), &node, None)?);

		return Ok(Box::new(HIRNode::new(HIRNodeKind::ArrayIndexModify { array, index, new_val }, &node.start, &node.end)));
	}

	panic!("Invalid node type")
}