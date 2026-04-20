use astoir_hir::{
    nodes::{HIRNode, HIRNodeKind},
    structs::StructLRUStep,
};
use astoir_mir::{
    blocks::refer::MIRBlockReference,
    builder::build_field_pointer,
    vals::{base::BaseMIRValue, refer::MIRVariableReference},
};
use compiler_typing::raw::RawType;
use diagnostics::{DiagnosticResult, unsure_panic};

use crate::MIRLoweringContext;

pub fn lower_hir_lru_step(
    block: MIRBlockReference,
    step: StructLRUStep,
    ctx: &mut MIRLoweringContext,
    curr: Option<BaseMIRValue>,
) -> DiagnosticResult<BaseMIRValue> {
    if let StructLRUStep::VariableStep { variable } = step {
        if curr.is_none() {
            return Ok(ctx.mir_ctx.blocks[block]
                .get_variable_ref(variable)?
                .as_pointer_ref()?
                .into());
        }

        let curr = curr.unwrap();
        let ptr = curr.as_ptr()?;

        let struct_type = match ctx
            .mir_ctx
            .ssa_hints
            .get_hint(curr.get_ssa_index())
            .get_type()
            .get_generic(&ctx.hir_ctx.type_storage)
        {
            RawType::LoweredStruct(_, container) => container,
            _ => unsure_panic!("lower_hir_lru_step curr was not an actual thing"),
        };

        return Ok(build_field_pointer(
            &mut ctx.mir_ctx,
            ptr,
            struct_type.resolve_hir_index(variable),
        )?
        .into());
    }

    panic!("Invalid step!")
}

pub fn lower_hir_lru(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<BaseMIRValue> {
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
