use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode};
use compiler_errors::{IR_INVALID_NODE_TYPE, UNUSED_VAR_ACCESS, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::values::lower_ast_value;

pub fn lower_ast_math_operation(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>, enforce_assign: bool) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::MathResult { lval, rval, operator, assigns } = node.kind.clone() {
		if enforce_assign && !assigns {
			return Err(CompilerError::from_ast(ErrorKind::Warn, UNUSED_VAR_ACCESS!().to_string(), &node.start, &node.end))
		}

		let left = lower_ast_value(context, curr_ctx, lval)?;

		let right = match lower_ast_value(context, curr_ctx, rval)?.use_as(context, curr_ctx, left.get_node_type(context, curr_ctx).unwrap()) {
			Ok(v) => Box::new(v),
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		return Ok(Box::new(HIRNode::MathOperation { left, right, operation: operator, assignment: assigns }))		
	} 

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}