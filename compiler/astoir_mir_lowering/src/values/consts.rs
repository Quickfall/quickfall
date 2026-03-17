use astoir_hir::{nodes::HIRNode};
use astoir_mir::{builder::{build_signed_int_const, build_static_string_const, build_unsigned_int_const}, vals::base::BaseMIRValue};
use astoir_typing::compacted::CompactedType;
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::MIRLoweringContext;

pub fn lower_hir_literal(node: Box<HIRNode>, ctx: &mut MIRLoweringContext, t: Option<CompactedType>) -> BaseResult<BaseMIRValue> {
	match *node {
		HIRNode::IntegerLiteral { value, int_type } => {	
			let base;

			if t.is_some() {
				base = t.unwrap().base;
			} else {
				base = ctx.hir_ctx.type_storage.types[int_type].clone();
			}

			if base.is_signed() {
				let val = build_signed_int_const(&mut ctx.mir_ctx, value, base.get_size()?)?;

				return Ok(val.into());
			}

			let val = build_unsigned_int_const(&mut ctx.mir_ctx, value as u128, base.get_size()?)?;

			return Ok(val.into());
		},

		HIRNode::StringLiteral { value } => {
			let val = build_static_string_const(&mut ctx.mir_ctx, value)?;

			return Ok(val.into());
		},

		_ => return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
	}
}