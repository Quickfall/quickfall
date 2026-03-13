use astoir_hir::{ctx::HIRContext, nodes::HIRNode};
use astoir_mir::{blocks::MIRBlock, builder::{build_float_add, build_float_div, build_float_mul, build_float_sub, build_int_add, build_int_div, build_int_mul, build_int_sub, build_store}, ctx, vals::base::BaseMIRValue};
use astoir_typing::base::BaseType;
use compiler_errors::{IR_INVALID_NODE_TYPE, IR_REQ_VARIABLE_ASSIGN, errs::{BaseResult, base::BaseError}};
use lexer::toks::math::MathOperator;

use crate::{values::lower_hir_value, vars::lower_hir_variable_reference};
 
pub fn lower_hir_math_operation(block: &mut MIRBlock, node: Box<HIRNode>, ctx: &HIRContext) -> BaseResult<BaseMIRValue> {
	if let HIRNode::MathOperation { left, right, operation, assignment } = *node {
		if assignment && !left.is_variable_reference() {
			return Err(BaseError::err(IR_REQ_VARIABLE_ASSIGN!().to_string()))
		}			

		let ptr;

		if assignment {
			ptr = Some(lower_hir_variable_reference(block, &left)?);
		} else {
			ptr = None
		}

		let left_val = lower_hir_value(block, left, ctx)?;
		let right_val = lower_hir_value(block, right, ctx)?;
				

		let val = match left_val.vtype.base {
			BaseType::NumericIntegerType(_, _) => lower_hir_math_operation_int(block, left_val, right_val, operation, ctx)?,
			BaseType::FloatingNumberType(_, _, _) => lower_hir_math_operation_float(block, left_val, right_val, operation, ctx)?,

			// TODO: see if fixed point are needed or do they automatically fallback to int

			_ => return Err(BaseError::err("Cannot use lower_hir_math_operation on this given value kind!".to_string()))
		};

		if assignment {
			build_store(block, ptr.unwrap(), val)?;
		}
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}

pub fn lower_hir_math_operation_int(block: &mut MIRBlock, left: BaseMIRValue, right: BaseMIRValue, operator: MathOperator, ctx: &HIRContext) -> BaseResult<BaseMIRValue> {
	let left = left.as_int()?;
	let right = right.as_int()?;

	let signed = left.signed;

	let res = match operator {
		MathOperator::ADD => build_int_add(block, left, right, signed)?,
		MathOperator::SUBSTRACT => build_int_sub(block, left, right, signed)?,
		MathOperator::MULTIPLY => build_int_mul(block, left, right, signed)?,
		MathOperator::DIVIDE => build_int_div(block, left, right, signed)?
	};

	return Ok(res.into());
}

pub fn lower_hir_math_operation_float(block: &mut MIRBlock, left: BaseMIRValue, right: BaseMIRValue, operator: MathOperator, ctx: &HIRContext) -> BaseResult<BaseMIRValue> {
	let left = left.as_float()?;
	let right = right.as_float()?;

	let signed = left.signed;

	let res = match operator {
		MathOperator::ADD => build_float_add(block, left, right, signed)?,
		MathOperator::SUBSTRACT => build_float_sub(block, left, right, signed)?,
		MathOperator::MULTIPLY => build_float_mul(block, left, right, signed)?,
		MathOperator::DIVIDE => build_float_div(block, left, right, signed)?
	};

	return Ok(res.into());
}