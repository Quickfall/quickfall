//! Variable related lowering

use astoir_hir::{ctx::HIRBranchedContext, nodes::HIRNode};
use astoir_mir::{blocks::{MIRBlock, MIRBlockVariableSSAHint, MIRBlockVariableType}, builder::{build_stack_alloc, build_store}, vals::{base::BaseMIRValue, refer::MIRVariableReference}};
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
		
		block.variables.insert(variable, MIRBlockVariableSSAHint { kind: MIRBlockVariableType::Pointer, hint: Some(ptr.clone().into()) });

		if default_val.is_some() {
			let val = lower_hir_value(block, default_val.unwrap(), ctx)?;

			build_store(&mut ctx.mir_ctx, block, ptr.clone(), val)?;
		}

		return Ok(true)
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}

pub fn lower_hir_variable_reference(block: &mut MIRBlock, node: &Box<HIRNode>) -> BaseResult<MIRVariableReference> {
	if let HIRNode::VariableReference { index, is_static: _ } = &**node { // TODO: add support for static variables
		return block.get_variable_ref(*index)
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}


/// Lowers the HIR variable reference as if to obtain it's value. Requires a load
pub fn lower_hir_variable_reference_value(block: &mut MIRBlock, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<BaseMIRValue> {
	let ptr = lower_hir_variable_reference(block, &node)?;
	
	return Ok(ptr.read(block, &mut ctx.mir_ctx)?);
}

pub fn lower_hir_variable_assignment(block: &mut MIRBlock, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	if let HIRNode::VarAssigment { variable, val } = *node {
		let val = lower_hir_value(block, val, ctx)?;

		let variable_ref = block.get_variable_ref(variable)?;
 
		variable_ref.write(block, &mut ctx.mir_ctx, val)?;
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))

}