use astoir_hir::{nodes::{HIRNode, HIRNodeKind}, structs::StructLRUStep};
use astoir_mir::{blocks::refer::MIRBlockReference, builder::build_field_pointer, vals::{base::BaseMIRValue, refer::MIRVariableReference}};
use diagnostics::DiagnosticResult;

use crate::MIRLoweringContext;

pub fn lower_hir_lru_step(block: MIRBlockReference, step: StructLRUStep, ctx: &mut MIRLoweringContext, curr: Option<BaseMIRValue>) -> DiagnosticResult<BaseMIRValue> {
	if let StructLRUStep::VariableStep { variable } = step {
		if curr.is_none() {
			return Ok(ctx.mir_ctx.blocks[block].get_variable_ref(variable)?.as_pointer_ref()?.into());
		}

		let ptr = curr.unwrap().as_ptr()?;

		return Ok(build_field_pointer(&mut ctx.mir_ctx, ptr, variable)?.into())
	}

	panic!("Invalid step!")
}

pub fn lower_hir_lru(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> DiagnosticResult<BaseMIRValue> {
	if let HIRNodeKind::StructLRU { steps, last: _ } = node.kind {
		let mut curr = lower_hir_lru_step(block, steps[0].clone(), ctx, None)?;

		for i in 1..steps.len() {
			curr = lower_hir_lru_step(block, steps[i].clone(), ctx, Some(curr))?
		}

		let val = MIRVariableReference::from(curr.as_ptr()?);

		return Ok(val.read(block, &mut ctx.mir_ctx)?);
	}

	panic!("Invalid node!")
}