use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::{HIRNode, HIRNodeKind}};
use compiler_errors::{IR_FIND_TYPE, IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};
use compiler_typing::{storage::BOOLEAN_TYPE, tree::Type};
use diagnostics::{DiagnosticResult, builders::make_expected_simple_error};

use crate::values::lower_ast_value;

pub fn lower_ast_boolean_condition(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::BooleanBasedConditionMember { val, negate } = node.kind.clone() {
		let hir_value = lower_ast_value(context, curr_ctx, val)?;

		return Ok(Box::new(HIRNode::new(HIRNodeKind::BooleanCondition { value: hir_value, negation: negate }, &node.start, &node.end)));
	}

	panic!("Invalid node type")
}

pub fn lower_ast_operator_condition(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::OperatorBasedConditionMember { lval, rval, operator } = node.kind.clone() {
		let left_value = lower_ast_value(context, curr_ctx, lval)?;
		
		let right_value = Box::new(lower_ast_value(context, curr_ctx, rval)?.use_as(context, curr_ctx, left_value.get_node_type(context, curr_ctx).unwrap(), &node, None)?);

		return Ok(Box::new(HIRNode::new(HIRNodeKind::BooleanOperator { left: left_value, right: right_value, operator }, &node.start, &node.end)))
	}

	panic!("Invalid node type")
}

pub fn lower_ast_condition(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
	let start = &node.start.clone();
	let end = &node.end.clone();

	let hir_value = lower_ast_value(context, curr_ctx, node)?;

	let bool_type = match context.type_storage.types.get_index(BOOLEAN_TYPE) {
		Some(v) => v,
		None => panic!("bool type not found")
	};

	let val = hir_value.use_as(context, curr_ctx, Type::Generic(bool_type, vec![], vec![]), &node, None)?;

	return Ok(Box::new(val));
}