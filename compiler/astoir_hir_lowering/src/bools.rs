use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::values::lower_ast_value;

pub fn lower_ast_boolean_condition(context: &HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::BooleanBasedConditionMember { val, negate } = node.kind.clone() {
		let hir_value = lower_ast_value(context, curr_ctx, val)?;

		return Ok(Box::new(HIRNode::BooleanCondition { value: hir_value, negation: negate }))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_operator_condition(context: &HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::OperatorBasedConditionMember { lval, rval, operator } = node.kind.clone() {
		let left_value = lower_ast_value(context, curr_ctx, lval)?;
		let right_value = lower_ast_value(context, curr_ctx, rval)?;

		return Ok(Box::new(HIRNode::BooleanOperator { left: left_value, right: right_value, operator }))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}