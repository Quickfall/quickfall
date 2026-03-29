use astoir_hir::{nodes::HIRNode};
use astoir_mir::{blocks::refer::MIRBlockReference, builder::{build_static_array_const, build_static_array_one_const}, vals::base::BaseMIRValue};
use compiler_errors::{EXPECTED_VAL_FUNC, IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, arrays::lower_hir_aray_index_access, funcs::lower_hir_function_call, math::lower_hir_math_operation, values::{booleans::{lower_hir_boolean_operator, lowering_hir_boolean_condition}, consts::lower_hir_literal, structs::lower_hir_struct_init}, vars::{lower_hir_variable_reference, lower_hir_variable_reference_value}};

pub mod consts;
pub mod booleans;
pub mod structs;

pub fn lower_hir_value(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<BaseMIRValue> {
	match *node {
		HIRNode::IntegerLiteral { .. } | HIRNode::StringLiteral { .. } => return lower_hir_literal(node, ctx),
		HIRNode::VariableReference { .. } => return lower_hir_variable_reference_value(block, node, ctx),
		HIRNode::ReferenceGrab { val } => return Ok(lower_hir_variable_reference(block, &val, ctx)?.as_pointer_ref()?.into()),
		HIRNode::PointerGrab { val } => return Ok(lower_hir_variable_reference(block, &val, ctx)?.as_pointer_ref()?.into()),
		HIRNode::BooleanCondition { .. } => return Ok(lowering_hir_boolean_condition(block, node, ctx)?.into()),
		HIRNode::BooleanOperator { .. } => return Ok(lower_hir_boolean_operator(block, node, ctx)?.into()),
		HIRNode::MathOperation { .. } => return Ok(lower_hir_math_operation(block, node, ctx)?),
		HIRNode::ArrayIndexAccess { .. } => return Ok(lower_hir_aray_index_access(block, node, ctx)?),
		HIRNode::StructVariableInitializerValue { .. } => return Ok(lower_hir_struct_init(block, node, ctx)?.into()),
		HIRNode::ArrayVariableInitializerValue { .. } | HIRNode::ArrayVariableInitializerValueSameValue { .. } => lower_array_init(block, node, ctx),
		HIRNode::FunctionCall { .. } => {
			let res = lower_hir_function_call(block, node, ctx)?;

			if res.is_none() {
				return Err(BaseError::err(EXPECTED_VAL_FUNC!().to_string()));
			}

			return Ok(res.unwrap());
		}

		_ => return Err(BaseError::err(format!(IR_INVALID_NODE_TYPE!(), node)))
	}
}

pub fn lower_array_init(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<BaseMIRValue> {
	match *node {
		HIRNode::ArrayVariableInitializerValue { vals } => {

			let mut v = vec![];

			for val in vals {
				v.push(lower_hir_value(block, val, ctx)?)
			}

			return Ok(build_static_array_const(&mut ctx.mir_ctx, v)?.into())
		},

		HIRNode::ArrayVariableInitializerValueSameValue { size, val } => {
			let v = lower_hir_value(block, val, ctx)?;

			return Ok(build_static_array_one_const(&mut ctx.mir_ctx, v, size)?.into());
		},

		_ => return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
	}
}