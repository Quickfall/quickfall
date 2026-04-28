//! Lowering for for loops

use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{
    blocks::{MIRBlock, refer::MIRBlockReference},
    builder::{build_conditional_branch, build_unconditional_branch},
};
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, move_current_diagnostic_pos};

use crate::{
    MIRLoweringContext, body::lower_hir_body, math::lower_hir_math_operation,
    values::lower_hir_value, vars::lower_hir_variable_declaration,
};

pub fn lower_hir_for_loop(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<bool> {
    if let HIRNodeKind::ForBlock {
        initial_state,
        condition,
        incrementation,
        body,
    } = node.kind.clone()
    {
        let header_ref = MIRBlock::new_merge(block, &mut ctx.mir_ctx, false);
        let cond_ref = MIRBlock::new_merge(header_ref, &mut ctx.mir_ctx, false);
        let body_ref = MIRBlock::new_merge(header_ref, &mut ctx.mir_ctx, false);
        let exit_ref = MIRBlock::new_merge(block, &mut ctx.mir_ctx, false);

        ctx.mir_ctx.blocks[header_ref].merge_blocks.push(block);
        ctx.mir_ctx.blocks[header_ref].merge_blocks.push(body_ref);

        move_current_diagnostic_pos(initial_state.get_pos());
        lower_hir_variable_declaration(block, initial_state, ctx)?;

        ctx.mir_ctx.writer.move_end(body_ref);

        lower_hir_body(body_ref, body, ctx)?;

        move_current_diagnostic_pos(incrementation.get_pos());
        lower_hir_math_operation(body_ref, incrementation, ctx)?;

        build_unconditional_branch(&mut ctx.mir_ctx, header_ref)?;

        ctx.mir_ctx.writer.move_end(header_ref);

        ctx.mir_ctx.resolve_ssa(header_ref)?;
        build_unconditional_branch(&mut ctx.mir_ctx, cond_ref)?;

        ctx.mir_ctx.writer.move_end(cond_ref);

        move_current_diagnostic_pos(condition.get_pos());
        let cond_val = lower_hir_value(block, condition, ctx)?;

        build_conditional_branch(&mut ctx.mir_ctx, cond_val.as_int()?, body_ref, exit_ref)?;

        ctx.mir_ctx.writer.move_end(exit_ref);
    }

    panic!("Invalid node")
}
