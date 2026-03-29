//! Variable related lowering

use astoir_hir::{nodes::HIRNode};
use astoir_mir::{blocks::{MIRBlockVariableSSAHint, MIRBlockVariableType, refer::MIRBlockReference}, builder::{build_stack_alloc, build_store}, vals::{base::BaseMIRValue, refer::MIRVariableReference}};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};
use compiler_typing::SizedType;

use crate::{MIRLoweringContext, lower_hir_type, values::lower_hir_value};

pub fn lower_hir_variable_declaration(block_id: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	if let HIRNode::VarDeclaration { variable, var_type, default_val } = *node {
		let func = ctx.mir_ctx.block_to_func[&block_id];
		let local_ctx = ctx.hir_ctx.function_contexts[func].as_ref().unwrap();

		if local_ctx.is_eligible_for_ssa(variable) {
			if default_val.is_some() {
				let val = lower_hir_value(block_id, default_val.unwrap(), ctx)?;
	
				ctx.mir_ctx.blocks[block_id].variables.insert(variable, MIRBlockVariableSSAHint { kind: MIRBlockVariableType::SSA, hint: Some(val) });
			} else {
				ctx.mir_ctx.blocks[block_id].variables.insert(variable, MIRBlockVariableSSAHint { kind: MIRBlockVariableType::SSA, hint: None });
			}
		} else {
			let lowered = lower_hir_type(ctx, var_type)?;

			// TODO: allow build_stack_allow to allocate non-raw types
			let ptr = build_stack_alloc(&mut ctx.mir_ctx, lowered.get_size(&lowered, false, &ctx.hir_ctx.type_storage), lowered)?;
		
			ctx.mir_ctx.blocks[block_id].variables.insert(variable, MIRBlockVariableSSAHint { kind: MIRBlockVariableType::Pointer, hint: Some(ptr.clone().into()) });

			if default_val.is_some() {
				let val = lower_hir_value(block_id, default_val.unwrap(), ctx)?;
			
				build_store(&mut ctx.mir_ctx, ptr.clone(), val)?;
			}
		}

		return Ok(true)
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}

pub fn lower_hir_variable_reference(block: MIRBlockReference, node: &Box<HIRNode>, ctx: &MIRLoweringContext) -> BaseResult<MIRVariableReference> {
	if let HIRNode::VariableReference { index, is_static: _ } = &**node { // TODO: add support for static variables
		return ctx.mir_ctx.blocks[block].get_variable_ref(*index)
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}


/// Lowers the HIR variable reference as if to obtain it's value. Requires a load
pub fn lower_hir_variable_reference_value(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<BaseMIRValue> {
	let ptr = lower_hir_variable_reference(block, &node, ctx)?;
	
	let read = ptr.read(block, &mut ctx.mir_ctx)?;

	return Ok(read);
}

pub fn lower_hir_variable_assignment(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	if let HIRNode::VarAssigment { variable, val } = *node {
		let variable_ref = ctx.mir_ctx.blocks[block].get_variable_ref(variable)?;
 
		let val = lower_hir_value(block, val, ctx)?;

		variable_ref.write(block, &mut ctx.mir_ctx, val)?;
		return Ok(true);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}