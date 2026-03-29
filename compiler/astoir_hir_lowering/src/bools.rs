use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode};
use compiler_errors::{IR_FIND_TYPE, IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};
use compiler_typing::{storage::BOOLEAN_TYPE, tree::Type};

use crate::values::lower_ast_value;

pub fn lower_ast_boolean_condition(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::BooleanBasedConditionMember { val, negate } = node.kind.clone() {
		let hir_value = lower_ast_value(context, curr_ctx, val)?;

		return Ok(Box::new(HIRNode::BooleanCondition { value: hir_value, negation: negate }))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_operator_condition(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::OperatorBasedConditionMember { lval, rval, operator } = node.kind.clone() {
		let left_value = lower_ast_value(context, curr_ctx, lval)?;
		
		let right_value = match lower_ast_value(context, curr_ctx, rval)?.use_as(context, curr_ctx, left_value.get_node_type(context, curr_ctx).unwrap()) {
			Ok(v) => Box::new(v),
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		return Ok(Box::new(HIRNode::BooleanOperator { left: left_value, right: right_value, operator }))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_condition(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	let start = &node.start.clone();
	let end = &node.end.clone();

	let hir_value = lower_ast_value(context, curr_ctx, node)?;

	let bool_type = match context.type_storage.types.get_index(BOOLEAN_TYPE) {
		Some(v) => v,
		None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_TYPE!().to_string(), start, end))
	};

	let val = match hir_value.use_as(context, curr_ctx, Type::Generic(bool_type, vec![], vec![])) {
		Ok(v) => v,
		Err(e) => return Err(CompilerError::from_base(e, start, end))
	};

	return Ok(Box::new(val));
}