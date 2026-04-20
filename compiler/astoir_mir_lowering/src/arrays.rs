use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{
    blocks::refer::MIRBlockReference,
    builder::{build_index_pointer, build_load, build_store},
    vals::base::BaseMIRValue,
};
use diagnostics::{DiagnosticResult, unsure_panic};

use crate::{MIRLoweringContext, values::lower_hir_value};

pub fn lower_hir_aray_index_access(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<BaseMIRValue> {
    if let HIRNodeKind::ArrayIndexAccess { val, index } = node.kind {
        let array = lower_hir_value(block, val, ctx)?;

        if ctx
            .mir_ctx
            .ssa_hints
            .get_hint(array.get_ssa_index())
            .is_pointer()
        {
            let index = lower_hir_value(block, index, ctx)?.as_int()?;

            let res = build_index_pointer(&mut ctx.mir_ctx, array.as_ptr()?, index)?;

            return build_load(&mut ctx.mir_ctx, res);
        } else {
            unsure_panic!("tried lowering a non SSA array!")
        }
    }

    panic!("Invalid node type")
}

pub fn lower_hir_array_modify(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<bool> {
    if let HIRNodeKind::ArrayIndexModify {
        array,
        index,
        new_val,
    } = node.kind.clone()
    {
        let array = lower_hir_value(block, array, ctx)?.as_ptr()?;
        let index = lower_hir_value(block, index, ctx)?.as_int()?;
        let val = lower_hir_value(block, new_val, ctx)?;

        let index_pointer = build_index_pointer(&mut ctx.mir_ctx, array, index)?;

        build_store(
            &mut ctx.mir_ctx,
            &ctx.hir_ctx.type_storage,
            index_pointer,
            val,
        )?;

        return Ok(true);
    }

    panic!("Invalid node type")
}
