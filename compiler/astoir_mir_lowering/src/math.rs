use astoir_hir::{ctx::HIRContext, nodes::HIRNode};
use astoir_mir::{blocks::MIRBlock, builder::{build_int_add, build_int_div, build_int_mul, build_int_sub}, vals::base::{BaseMIRValue, BaseValueType}};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};
use lexer::toks::math::MathOperator;

use crate::values::lower_hir_value;
 
pub fn lower_hir_math_operation(block: &mut MIRBlock, node: Box<HIRNode>, ctx: &HIRContext) -> BaseResult<BaseMIRValue> {
	if let HIRNode::MathOperation { left, right, operation, assignment } = *node {
		let left = lower_hir_value(block, left, ctx)?;
		let right = lower_hir_value(block, right, ctx)?;

		match left.vtype {
			BaseValueType::IntValue(_) => return lower_hir_math_operation_int(block, left, right, operation, ctx),

			_ => {}
		}

	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}

pub fn lower_hir_math_operation_int(block: &mut MIRBlock, left: BaseMIRValue, right: BaseMIRValue, operator: MathOperator, ctx: &HIRContext) -> BaseResult<BaseMIRValue> {
	let left = left.as_int()?;
	let right = right.as_int()?;

	let res = match operator {
		MathOperator::ADD => build_int_add(block, left, right, false)?,
		MathOperator::SUBSTRACT => build_int_sub(block, left, right, false)?,
		MathOperator::MULTIPLY => build_int_mul(block, left, right, false)?,
		MathOperator::DIVIDE => build_int_div(block, left, right, false)?
	};

	return Ok(res.into());
}