//! Variable related lowering

use astoir_hir::{ctx::HIRBranchedContext, nodes::HIRNode};
use astoir_mir::{blocks::{MIRBlock, MIRBlockVariableSSAHint, MIRBlockVariableType}, builder::{build_load, build_stack_alloc, build_store}, vals::{base::BaseMIRValue, ptr::MIRPointerValue}};
use astoir_typing::compacted::CompactedType;
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, values::lower_hir_value};

pub fn lower_hir_variable_declaration(block: &mut MIRBlock, node: Box<HIRNode>, ctx: &mut MIRLoweringContext, branched: &HIRBranchedContext) -> BaseResult<bool> {
	if let HIRNode::VarDeclaration { variable, var_type, default_val } = *node {
		let lowered = CompactedType::from(var_type);

		if branched.is_eligible_for_ssa(variable) {
			if default_val.is_some() {
				let val = lower_hir_value(block, default_val.unwrap(), ctx)?;

				block.variables.insert(variable, MIRBlockVariableSSAHint { kind: MIRBlockVariableType::SSA, hint: Some(val) });
			} else {
				block.variables.insert(variable, MIRBlockVariableSSAHint { kind: MIRBlockVariableType::SSA, hint: None });
			}

			return Ok(true);
		}

		let ptr = build_stack_alloc(&mut ctx.mir_ctx, block, lowered.base.get_size()?, lowered)?;
		
		if block.ctx.pointer_vals.len() != variable {
			return Err(BaseError::err("lower_hir_variable_declaration possibly skipped a variable!".to_string()));
		}
		
		block.ctx.pointer_vals[variable] = ptr.clone();

		if default_val.is_some() {
			let val = lower_hir_value(block, default_val.unwrap(), ctx)?;

			build_store(&mut ctx.mir_ctx, block, ptr.clone(), val)?;
		}

		return Ok(true)
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}

pub fn lower_hir_variable_reference(block: &mut MIRBlock, node: &Box<HIRNode>) -> BaseResult<MIRPointerValue> {
	if let HIRNode::VariableReference { index, is_static: _ } = &**node { // TODO: add support for static variables
		if block.ctx.pointer_vals.len() >= *index {
			return Err(BaseError::err("Tried getting an invalid pointer in lower_hir_variable_reference".to_string()))
		}

		return Ok(block.ctx.pointer_vals[*index].clone())
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}


/// Lowers the HIR variable reference as if to obtain it's value. Requires a load
pub fn lower_hir_variable_reference_value(block: &mut MIRBlock, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<BaseMIRValue> {
	let ptr = lower_hir_variable_reference(block, &node)?;

	let val = build_load(&mut ctx.mir_ctx, block, ptr)?;
	
	return Ok(val);
}

pub fn lower_hir_variable_assignment(block: &mut MIRBlock, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	if let HIRNode::VarAssigment { variable, val } = *node {
		let val = lower_hir_value(block, val, ctx)?;

		if block.ctx.pointer_vals.len() >= variable {
			return Err(BaseError::err("Tried getting an invalid pointer in lower_hir_variable_reference".to_string()))
		}

		let ptr = block.ctx.pointer_vals[variable].clone();

		build_store(&mut ctx.mir_ctx, block, ptr, val)?;
		return Ok(true);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))

}