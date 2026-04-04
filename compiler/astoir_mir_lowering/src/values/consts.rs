use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{builder::{build_signed_int_const, build_static_string_const, build_unsigned_int_const}, vals::base::BaseMIRValue};
use compiler_typing::SizedType;
use diagnostics::DiagnosticResult;

use crate::MIRLoweringContext;

pub fn lower_hir_literal(node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> DiagnosticResult<BaseMIRValue> {
	match node.kind {
		HIRNodeKind::IntegerLiteral { value, int_type } => {	
			if int_type.get_generic(&ctx.hir_ctx.type_storage).is_signed() {
				let val = build_signed_int_const(&mut ctx.mir_ctx, value, int_type.get_size(&int_type, true, &ctx.hir_ctx.type_storage))?;

				return Ok(val.into());
			}

			let val = build_unsigned_int_const(&mut ctx.mir_ctx, value as u128, int_type.get_size(&int_type, true, &ctx.hir_ctx.type_storage))?;

			return Ok(val.into());
		},

		HIRNodeKind::StringLiteral { value } => {
			let val = build_static_string_const(&mut ctx.mir_ctx, value)?;

			return Ok(val.into());
		},

		_ => panic!("Invalid node")
	}
}