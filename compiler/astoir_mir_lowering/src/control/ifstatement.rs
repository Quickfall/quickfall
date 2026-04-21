use astoir_hir::{
    nodes::{HIRNode, HIRNodeKind},
    structs::HIRIfBranch,
};
use astoir_mir::{
    blocks::{MIRBlock, refer::MIRBlockReference},
    builder::{build_conditional_branch, build_unconditional_branch},
};
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, move_current_diagnostic_pos};

use crate::{MIRLoweringContext, body::lower_hir_body, values::lower_hir_value};

pub fn lower_hir_if_statement(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<bool> {
    if let HIRNodeKind::IfStatement { branches } = node.kind.clone() {
        let merge_ref = MIRBlock::new_merge(block, &mut ctx.mir_ctx, false);
        let mut branch_blocks = vec![];

        ctx.mir_ctx.blocks[merge_ref].merge_blocks.push(block);

        for branch in &branches {
            match branch {
                &HIRIfBranch::IfBranch { .. } => {
                    let ifbranch_body = MIRBlock::new_merge(block, &mut ctx.mir_ctx, false);

                    branch_blocks.push(ifbranch_body);

                    ctx.mir_ctx.blocks[merge_ref]
                        .merge_blocks
                        .push(ifbranch_body);
                }

                &HIRIfBranch::ElseIfBranch { .. } => {
                    let ifelsebranch_cond = MIRBlock::new_merge(block, &mut ctx.mir_ctx, false);
                    let ifelsebranch_body = MIRBlock::new_merge(block, &mut ctx.mir_ctx, false);

                    branch_blocks.push(ifelsebranch_cond);
                    branch_blocks.push(ifelsebranch_body);

                    ctx.mir_ctx.blocks[merge_ref]
                        .merge_blocks
                        .push(ifelsebranch_body);
                }

                &HIRIfBranch::ElseBranch { .. } => {
                    let else_body = MIRBlock::new_merge(block, &mut ctx.mir_ctx, false);

                    branch_blocks.push(else_body);

                    ctx.mir_ctx.blocks[merge_ref].merge_blocks.push(else_body);
                }
            }
        }

        branch_blocks.push(merge_ref); // Allows for array usage for branch descending

        let mut branch_ind = 0;

        for branch in branches {
            match branch {
                HIRIfBranch::IfBranch { cond, body } => {
                    ctx.mir_ctx.writer.move_end(block);

                    move_current_diagnostic_pos(cond.get_pos());
                    let val = lower_hir_value(block, cond, ctx)?.as_int()?;

                    build_conditional_branch(
                        &mut ctx.mir_ctx,
                        val,
                        branch_blocks[branch_ind],
                        branch_blocks[branch_ind + 1],
                    )?;

                    ctx.mir_ctx.writer.move_end(branch_blocks[branch_ind]);

                    lower_hir_body(branch_blocks[branch_ind], body, ctx)?;

                    build_unconditional_branch(&mut ctx.mir_ctx, merge_ref)?;

                    branch_ind += 1;
                }

                HIRIfBranch::ElseIfBranch { cond, body } => {
                    ctx.mir_ctx.writer.move_end(branch_blocks[branch_ind]);

                    move_current_diagnostic_pos(cond.get_pos());
                    let val = lower_hir_value(branch_blocks[branch_ind], cond, ctx)?.as_int()?;

                    build_conditional_branch(
                        &mut ctx.mir_ctx,
                        val,
                        branch_blocks[branch_ind + 1],
                        branch_blocks[branch_ind + 2],
                    )?;

                    branch_ind += 1;
                    ctx.mir_ctx.writer.move_end(branch_blocks[branch_ind]);

                    lower_hir_body(branch_blocks[branch_ind], body, ctx)?;

                    build_unconditional_branch(&mut ctx.mir_ctx, merge_ref)?;

                    branch_ind += 1
                }

                HIRIfBranch::ElseBranch { body } => {
                    ctx.mir_ctx.writer.move_end(branch_blocks[branch_ind]);

                    lower_hir_body(branch_blocks[branch_ind], body, ctx)?;

                    build_unconditional_branch(&mut ctx.mir_ctx, merge_ref)?;

                    branch_ind += 1;
                }
            }
        }

        ctx.mir_ctx.writer.move_end(merge_ref);
        ctx.mir_ctx.resolve_ssa(merge_ref)?;

        return Ok(true);
    }

    panic!("Invalid node");
}
