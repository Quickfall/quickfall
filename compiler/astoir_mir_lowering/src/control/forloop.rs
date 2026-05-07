//! Lowering for for loops

use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{
    blocks::{MIRBlock, refer::MIRBlockReference},
    builder::{
        build_comp_lt, build_conditional_branch, build_int_add, build_unconditional_branch,
        build_unsigned_int_const,
    },
};
use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin, MaybeDiagnostic, move_current_diagnostic_pos,
};

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

        // Initial State

        move_current_diagnostic_pos(initial_state.get_pos());
        lower_hir_variable_declaration(block, initial_state, ctx, None)?;

        MIRBlock::propagate_variables(block, body_ref, &mut ctx.mir_ctx);
        MIRBlock::propagate_variables(block, header_ref, &mut ctx.mir_ctx);
        MIRBlock::propagate_variables(block, cond_ref, &mut ctx.mir_ctx);

        build_unconditional_branch(&mut ctx.mir_ctx, cond_ref)?;

        // Body reference

        ctx.mir_ctx.writer.move_end(body_ref);

        lower_hir_body(body_ref, body, ctx)?;

        // Incrementation

        move_current_diagnostic_pos(incrementation.get_pos());
        lower_hir_math_operation(body_ref, incrementation, ctx)?;

        build_unconditional_branch(&mut ctx.mir_ctx, header_ref)?;

        // Header

        ctx.mir_ctx.writer.move_end(header_ref);

        ctx.mir_ctx.resolve_ssa(header_ref)?;
        build_unconditional_branch(&mut ctx.mir_ctx, cond_ref)?;

        // Condition

        ctx.mir_ctx.writer.move_end(cond_ref);

        move_current_diagnostic_pos(condition.get_pos());
        let cond_val = lower_hir_value(block, condition, ctx)?;

        build_conditional_branch(&mut ctx.mir_ctx, cond_val.as_int()?, body_ref, exit_ref)?;

        ctx.mir_ctx.writer.move_end(exit_ref);
    }

    panic!("Invalid node")
}

pub fn lower_hir_ranged_for_loop(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> MaybeDiagnostic {
    if let HIRNodeKind::RangedForBlock {
        variable,
        range,
        body,
    } = node.kind.clone()
    {
        move_current_diagnostic_pos(node.get_pos());

        let header_ref = MIRBlock::new_merge(block, &mut ctx.mir_ctx, false);
        let cond_ref = MIRBlock::new_merge(header_ref, &mut ctx.mir_ctx, false);
        let exit_ref = MIRBlock::new_merge(block, &mut ctx.mir_ctx, false);
        let body_ref = MIRBlock::new_merge(header_ref, &mut ctx.mir_ctx, false);

        ctx.mir_ctx.blocks[header_ref].merge_blocks.push(block);
        ctx.mir_ctx.blocks[header_ref].merge_blocks.push(body_ref);

        // Initial state

        let min = lower_hir_value(block, range.min, ctx)?;
        let max = lower_hir_value(block, range.max, ctx)?;

        let v = lower_hir_variable_declaration(block, variable, ctx, Some(min.clone()))?;

        MIRBlock::propagate_variables(block, body_ref, &mut ctx.mir_ctx);
        MIRBlock::propagate_variables(block, header_ref, &mut ctx.mir_ctx);
        MIRBlock::propagate_variables(block, cond_ref, &mut ctx.mir_ctx);

        build_unconditional_branch(&mut ctx.mir_ctx, cond_ref)?;

        // Body reference

        ctx.mir_ctx.writer.move_end(body_ref);
        lower_hir_body(body_ref, body, ctx)?;

        // Incrementation

        let val = v.read(body_ref, &mut ctx.mir_ctx)?;
        let increment = build_unsigned_int_const(&mut ctx.mir_ctx, 1, min.as_int()?.size)?;
        let incremented = build_int_add(&mut ctx.mir_ctx, val.as_int()?, increment, false, false)?;

        v.write(
            body_ref,
            &mut ctx.mir_ctx,
            incremented.base,
            &ctx.hir_ctx.global_scope.scope,
        )?;

        build_unconditional_branch(&mut ctx.mir_ctx, header_ref)?;

        // Header

        ctx.mir_ctx.writer.move_end(header_ref);
        ctx.mir_ctx.resolve_ssa(header_ref)?;

        build_unconditional_branch(&mut ctx.mir_ctx, cond_ref)?;

        // Condition

        ctx.mir_ctx.writer.move_end(cond_ref);

        let val = v.read(cond_ref, &mut ctx.mir_ctx)?.as_int()?;
        let cond_val = build_comp_lt(&mut ctx.mir_ctx, val, max.as_int()?)?;

        build_conditional_branch(&mut ctx.mir_ctx, cond_val, body_ref, exit_ref)?;

        ctx.mir_ctx.writer.move_end(exit_ref);

        return Ok(());
    }

    panic!("Invalid node")
}
