use astoir_hir::{nodes::HIRNode};
use astoir_mir::{blocks::{refer::MIRBlockReference}, vals::base::BaseMIRValue};
use compiler_errors::{EXPECTED_VAL_FUNC, IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, funcs::lower_hir_function_call, math::lower_hir_math_operation, values::{booleans::{lower_hir_boolean_operator, lowering_hir_boolean_condition}, consts::lower_hir_literal}, vars::lower_hir_variable_reference_value};

pub mod consts;
pub mod booleans;

pub fn lower_hir_value(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<BaseMIRValue> {
	match *node {
		HIRNode::IntegerLiteral { .. } | HIRNode::StringLiteral { .. } => return lower_hir_literal(node, ctx),
		HIRNode::VariableReference { .. } => return lower_hir_variable_reference_value(block, node, ctx),
		HIRNode::BooleanCondition { .. } => return Ok(lowering_hir_boolean_condition(block, node, ctx)?.into()),
		HIRNode::BooleanOperator { .. } => return Ok(lower_hir_boolean_operator(block, node, ctx)?.into()),
		HIRNode::MathOperation { .. } => return Ok(lower_hir_math_operation(block, node, ctx)?),
		HIRNode::FunctionCall { .. } => {
			let res = lower_hir_function_call(block, node, ctx)?;

			if res.is_none() {
				return Err(BaseError::err(EXPECTED_VAL_FUNC!().to_string()));
			}

			return Ok(res.unwrap());
		}

		_ => return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
	}
}