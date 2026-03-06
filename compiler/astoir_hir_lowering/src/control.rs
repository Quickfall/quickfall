use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode, structs::HIRIfBranch};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::{bools::lower_ast_condition, lower_ast_body, math::lower_ast_math_operation, var::lower_ast_variable_declaration};

pub fn lower_ast_for_block(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::ForBlock { initial_state, cond, increment, body } = node.kind.clone() {
		let branch = curr_ctx.start_branch();

		let initial = lower_ast_variable_declaration(context, curr_ctx, initial_state)?;
		let condition = lower_ast_condition(context, curr_ctx, cond)?;

		let incrementation = lower_ast_math_operation(context, curr_ctx, increment, true)?;

		let body = lower_ast_body(context, curr_ctx, body, false)?;

		curr_ctx.end_branch(branch);

		return Ok(Box::new(HIRNode::ForBlock { initial_state:initial, condition, incrementation, body }));
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_while_block(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::WhileBlock { cond, body } = node.kind.clone() {
		let condition = lower_ast_condition(context, curr_ctx, cond)?;

		let branch = curr_ctx.start_branch();

		let body = lower_ast_body(context, curr_ctx, body, true)?;

		curr_ctx.end_branch(branch);

		return Ok(Box::new(HIRNode::WhileBlock { condition, body }));
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_if_statement_branch(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<HIRIfBranch> {
	match node.kind {
		ASTTreeNodeKind::IfElseStatement { cond, body } => {
			let condition = lower_ast_condition(context, curr_ctx, cond.unwrap())?;
			let body = lower_ast_body(context, curr_ctx, body, true)?;

			return Ok(HIRIfBranch::ElseIfBranch { cond: condition, body })
		},

		ASTTreeNodeKind::ElseStatement { body } => {
			let body = lower_ast_body(context, curr_ctx, body, true)?;

			return Ok(HIRIfBranch::ElseBranch { body })
		},

		ASTTreeNodeKind::IfStatement { cond, body, branches: _, depth: _} => {
			let condition = lower_ast_condition(context, curr_ctx, cond)?;
			let body = lower_ast_body(context, curr_ctx, body, true)?;

			return Ok(HIRIfBranch::IfBranch { cond: condition, body })
		},

		_ => return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
	}
}

pub fn lower_ast_if_statement(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::IfStatement { cond: _, body: _, branches, depth: _ } = node.kind.clone() {
		let mut hir_branches = vec![];

		hir_branches.push(lower_ast_if_statement_branch(context, curr_ctx, node)?);
		
		for b in branches {
			hir_branches.push(lower_ast_if_statement_branch(context, curr_ctx, b)?);
		}

		return Ok(Box::new(HIRNode::IfStatement { branches: hir_branches }))	
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}