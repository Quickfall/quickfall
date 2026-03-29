use astoir_hir::{nodes::HIRNode};
use astoir_mir::{blocks::{refer::MIRBlockReference}, builder::{build_bitwise_not, build_comp_eq, build_comp_ge, build_comp_gt, build_comp_le, build_comp_lt, build_comp_neg}, vals::int::MIRIntValue};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};
use lexer::toks::comp::ComparingOperator;

use crate::{MIRLoweringContext, values::lower_hir_value};

pub fn lower_hir_boolean_operator(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<MIRIntValue> {
	if let HIRNode::BooleanOperator { left, right, operator } = *node {
		let a = lower_hir_value(block, left, ctx)?.as_int()?;
		let b = lower_hir_value(block, right, ctx)?.as_int()?;

		let val = match operator {
			ComparingOperator::Equal => build_comp_eq(&mut ctx.mir_ctx, a, b)?,
			ComparingOperator::NotEqual => build_comp_neg(&mut ctx.mir_ctx, a, b)?,
			ComparingOperator::Lower => build_comp_lt(&mut ctx.mir_ctx, a, b)?,
			ComparingOperator::LowerEqual => build_comp_le(&mut ctx.mir_ctx, a, b)?,
			ComparingOperator::Higher => build_comp_gt(&mut ctx.mir_ctx, a, b)?,
			ComparingOperator::HigherEqual => build_comp_ge(&mut ctx.mir_ctx, a, b)?
		};

		return Ok(val);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}

pub fn lowering_hir_boolean_condition(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<MIRIntValue> {
	if let HIRNode::BooleanCondition { value, negation } = *node {
		let mut val = lower_hir_value(block, value, ctx)?.as_int()?;

		if negation {
			val = build_bitwise_not(&mut ctx.mir_ctx, val)?;
		}

		return Ok(val);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}