use astoir_hir::{nodes::HIRNode};
use astoir_mir::{blocks::{refer::MIRBlockReference}, vals::base::BaseMIRValue};
use astoir_typing::compacted::CompactedType;
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, math::lower_hir_math_operation, values::{booleans::{lower_hir_boolean_operator, lowering_hir_boolean_condition}, consts::lower_hir_literal}, vars::lower_hir_variable_reference_value};

pub mod consts;
pub mod booleans;

pub fn lower_hir_value(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext, expected: Option<CompactedType>) -> BaseResult<BaseMIRValue> {
	match *node {
		HIRNode::IntegerLiteral { .. } | HIRNode::StringLiteral { .. } => return lower_hir_literal(node, ctx, expected),
		HIRNode::VariableReference { .. } => return lower_hir_variable_reference_value(block, node, ctx, expected),
		HIRNode::BooleanCondition { .. } => return Ok(lowering_hir_boolean_condition(block, node, ctx)?.into()),
		HIRNode::BooleanOperator { .. } => return Ok(lower_hir_boolean_operator(block, node, ctx)?.into()),
		HIRNode::MathOperation { .. } => return Ok(lower_hir_math_operation(block, node, ctx, expected)?),

		_ => return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
	}
}