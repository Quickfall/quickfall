//! Variable related lowering

use std::f32::consts::E;

use astoir_hir::{nodes::HIRNode};
use astoir_mir::{blocks::{MIRBlockVariableSSAHint, MIRBlockVariableType, refer::MIRBlockReference}, vals::{base::BaseMIRValue, refer::MIRVariableReference}};
use astoir_typing::compacted::CompactedType;
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, values::lower_hir_value};

pub fn lower_hir_variable_declaration(block_id: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	if let HIRNode::VarDeclaration { variable, var_type, default_val } = *node {
		let lowered = CompactedType::from(var_type);

		if default_val.is_some() {
			let val = lower_hir_value(block_id, default_val.unwrap(), ctx, Some(lowered.clone()))?;

			if val.vtype.can_transmute(&lowered) {
				// TODO: allow transmutation here
			}

			ctx.mir_ctx.blocks[block_id].variables.insert(variable, MIRBlockVariableSSAHint { kind: MIRBlockVariableType::SSA, hint: Some(val) });
		} else {
			ctx.mir_ctx.blocks[block_id].variables.insert(variable, MIRBlockVariableSSAHint { kind: MIRBlockVariableType::SSA, hint: None });
		}

		//let ptr = build_stack_alloc(&mut ctx.mir_ctx, lowered.base.get_size()?, lowered)?;
		
		//ctx.mir_ctx.blocks[block_id].variables.insert(variable, MIRBlockVariableSSAHint { kind: MIRBlockVariableType::Pointer, hint: Some(ptr.clone().into()) });

		//if default_val.is_some() {
		//	let val = lower_hir_value(block_id, default_val.unwrap(), ctx)?;
		//
		//	build_store(&mut ctx.mir_ctx, ptr.clone(), val)?;
		//}

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
pub fn lower_hir_variable_reference_value(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext, expected: Option<CompactedType>) -> BaseResult<BaseMIRValue> {
	let ptr = lower_hir_variable_reference(block, &node, ctx)?;
	
	let read = ptr.read(block, &mut ctx.mir_ctx)?;

	if expected.is_some() {
		let expected = expected.unwrap();

		if !read.vtype.can_transmute(&expected) {
			return Err(BaseError::err("Cannot transmute to given type!".to_string()));
		}

		
	}

	return Ok(ptr.read(block, &mut ctx.mir_ctx)?);
}

pub fn lower_hir_variable_assignment(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	if let HIRNode::VarAssigment { variable, val } = *node {
		let variable_ref = ctx.mir_ctx.blocks[block].get_variable_ref(variable)?;
 
		let hint = ctx.mir_ctx.ssa_hints.get_hint(variable_ref.get_hint())?;

		let val = lower_hir_value(block, val, ctx, Some(hint.get_type()?))?;

		variable_ref.write(block, &mut ctx.mir_ctx, val)?;
		return Ok(true);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}