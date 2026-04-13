use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{blocks::{refer::MIRBlockReference}, builder::{build_float_add, build_float_div, build_float_mul, build_float_sub, build_int_add, build_int_div, build_int_mul, build_int_sub}, vals::base::BaseMIRValue};
use compiler_typing::raw::RawType;
use compiler_utils::operators::MathOperator;
use diagnostics::{DiagnosticResult, builders::make_math_operation_req_assign, unsure_panic};

use crate::{MIRLoweringContext, values::lower_hir_value, vars::lower_hir_variable_reference};
 
pub fn lower_hir_math_operation(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> DiagnosticResult<BaseMIRValue> {
	if let HIRNodeKind::MathOperation { left, right, operation, assignment } = node.clone().kind {
		if assignment && !left.is_variable_reference() {
			return Err(make_math_operation_req_assign(&*node).into())
		}			

		let ptr;

		if assignment {
			ptr = Some(lower_hir_variable_reference(block, &left, ctx)?);
		} else {
			ptr = None
		}

		let left_val = lower_hir_value(block, left, ctx)?;
		let right_val = lower_hir_value(block, right, ctx)?;
				

		let val = match left_val.vtype.get_generic(&ctx.hir_ctx.type_storage) {
			RawType::Integer(_, _) | RawType::FixedPoint(_, _, _) => lower_hir_math_operation_int(left_val, right_val, operation, ctx)?,
			RawType::Floating(_, _) => lower_hir_math_operation_float(left_val, right_val, operation, ctx)?,

			_  => unsure_panic!("Cannot use lower_hir_math_operator on this given value kind!")
		};

		if assignment {
			let v = ptr.unwrap();

			v.write(block, &mut ctx.mir_ctx, val.clone(), &ctx.hir_ctx.type_storage)?;
		}

		return Ok(val)
	}

	panic!("Invalid node")
}

pub fn lower_hir_math_operation_int(left: BaseMIRValue, right: BaseMIRValue, operator: MathOperator, ctx: &mut MIRLoweringContext) -> DiagnosticResult<BaseMIRValue> {
	let left = left.as_int()?;
	let right = right.as_int()?;

	let signed = left.signed;

	let res = match operator {
		MathOperator::ADD => build_int_add(&mut ctx.mir_ctx, left, right, signed)?,
		MathOperator::SUBSTRACT => build_int_sub(&mut ctx.mir_ctx, left, right, signed)?,
		MathOperator::MULTIPLY => build_int_mul(&mut ctx.mir_ctx, left, right, signed)?,
		MathOperator::DIVIDE => build_int_div(&mut ctx.mir_ctx, left, right, signed)?
	};

	return Ok(res.into());
}

pub fn lower_hir_math_operation_float(left: BaseMIRValue, right: BaseMIRValue, operator: MathOperator, ctx: &mut MIRLoweringContext) -> DiagnosticResult<BaseMIRValue> {
	let left = left.as_float()?;
	let right = right.as_float()?;

	let signed = left.signed;

	let res = match operator {
		MathOperator::ADD => build_float_add(&mut ctx.mir_ctx, left, right, signed)?,
		MathOperator::SUBSTRACT => build_float_sub(&mut ctx.mir_ctx, left, right, signed)?,
		MathOperator::MULTIPLY => build_float_mul(&mut ctx.mir_ctx, left, right, signed)?,
		MathOperator::DIVIDE => build_float_div(&mut ctx.mir_ctx, left, right, signed)?
	};

	return Ok(res.into());
}