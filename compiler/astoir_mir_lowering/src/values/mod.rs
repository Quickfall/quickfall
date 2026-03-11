use astoir_hir::{ctx::HIRContext, nodes::HIRNode};
use astoir_mir::{blocks::MIRBlock, vals::base::BaseMIRValue};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{values::{booleans::{lower_hir_boolean_operator, lowering_hir_boolean_condition}, consts::lower_hir_literal}, vars::lower_hir_variable_reference_value};

pub mod consts;
pub mod booleans;

pub fn lower_hir_value(block: &mut MIRBlock, node: Box<HIRNode>, ctx: &HIRContext) -> BaseResult<BaseMIRValue> {
	match *node {
		HIRNode::IntegerLiteral { .. } | HIRNode::StringLiteral { .. } => return lower_hir_literal(block, ctx, node),
		HIRNode::VariableReference { .. } => return lower_hir_variable_reference_value(block, node),
		HIRNode::BooleanCondition { .. } => return Ok(lowering_hir_boolean_condition(block, node, ctx)?.into()),
		HIRNode::BooleanOperator { .. } => return Ok(lower_hir_boolean_operator(block, node, ctx)?.into()),

		_ => return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
	}
}