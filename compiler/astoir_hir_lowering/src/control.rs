use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::{lower_ast_body, math::lower_ast_math_operation, values::lower_ast_value, var::lower_ast_variable_declaration};

pub fn lower_ast_for_block(context: &HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::ForBlock { initial_state, cond, increment, body } = node.kind.clone() {
		let branch = curr_ctx.start_branch();

		let initial = lower_ast_variable_declaration(context, curr_ctx, initial_state)?;
		let condition = lower_ast_value(context, curr_ctx, cond)?;
		let incrementation = lower_ast_math_operation(context, curr_ctx, increment, true)?;

		let body = lower_ast_body(context, curr_ctx, body)?;

		curr_ctx.end_branch(branch);

		return Ok(Box::new(HIRNode::ForBlock { initial_state:initial, condition, incrementation, body }));
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}