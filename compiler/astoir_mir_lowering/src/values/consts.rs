use astoir_hir::{nodes::HIRNode};
use astoir_mir::{blocks::MIRBlock, builder::{build_signed_int_const, build_static_string_const, build_unsigned_int_const}, vals::base::BaseMIRValue};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::MIRLoweringContext;

pub fn lower_hir_literal(block: &mut MIRBlock, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<BaseMIRValue> {
	match *node {
		HIRNode::IntegerLiteral { value, int_type } => {
			let t = &ctx.hir_ctx.type_storage.types[int_type];
			
			if t.is_signed() {
				let val = build_signed_int_const(&mut ctx.mir_ctx, block, value, t.get_size()?)?;

				return Ok(val.into());
			}

			let val = build_unsigned_int_const(&mut ctx.mir_ctx, block, value as u128, t.get_size()?)?;

			return Ok(val.into());
		},

		HIRNode::StringLiteral { value } => {
			let val = build_static_string_const(&mut ctx.mir_ctx, block, value)?;

			return Ok(val.into());
		},

		_ => return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
	}
}