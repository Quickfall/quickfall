use astoir_hir::nodes::HIRNode;
use astoir_mir::{blocks::refer::MIRBlockReference, builder::{build_index_pointer}, vals::base::BaseMIRValue};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, values::lower_hir_value};

pub fn lower_hir_aray_index_access(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<BaseMIRValue> {
	if let HIRNode::ArrayIndexAccess { val, index } = *node {
		let array = lower_hir_value(block, val, ctx)?;

		if ctx.mir_ctx.ssa_hints.get_hint(array.get_ssa_index())?.is_pointer() {
			let index = lower_hir_value(block, index, ctx)?.as_int()?;

			let res = build_index_pointer(&mut ctx.mir_ctx, array.as_ptr()?, index)?;
	
			return Ok(res.into())
		} else {
			return Err(BaseError::err("Tried lowering a non SSA array!".to_string()))
		}
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}