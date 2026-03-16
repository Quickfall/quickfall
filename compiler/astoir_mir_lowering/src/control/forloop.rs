//! Lowering for for loops

use astoir_hir::nodes::HIRNode;
use astoir_mir::{blocks::MIRBlock, builder::build_unconditional_branch};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, body::lower_hir_body};

pub fn lower_hir_for_loop(block: &mut MIRBlock, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	if let HIRNode::ForBlock { initial_state, condition, incrementation, body } = *node {
		let header_ref = MIRBlock::new_merge(block.self_ref, &mut ctx.mir_ctx, false);
		let header_block = &mut ctx.mir_ctx.blocks[header_ref];

		let body_ref = MIRBlock::new_merge(header_ref, &mut ctx.mir_ctx, false);
		let body_block = &mut ctx.mir_ctx.blocks[body_ref];

		lower_hir_body(body_block, body, ctx)?;
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}