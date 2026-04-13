use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{blocks::{hints::MIRValueHint, refer::MIRBlockReference}, vals::base::BaseMIRValue};
use diagnostics::DiagnosticResult;

use crate::{MIRLoweringContext, lower_hir_type, values::lower_hir_value};

pub fn lower_cast(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> DiagnosticResult<BaseMIRValue> {
	if let HIRNodeKind::CastValue { intentional: _, value, old_type, new_type } = node.kind.clone() {
		let value = lower_hir_value(block, value, ctx)?;
		let old_type = lower_hir_type(ctx, old_type)?;

		let new_type = lower_hir_type(ctx, new_type)?;

		if old_type.get_generic(&ctx.hir_ctx.type_storage).is_enum_child() && new_type.get_generic(&ctx.hir_ctx.type_storage).is_enum_parent() {
			match ctx.mir_ctx.ssa_hints.vec[value.get_ssa_index()] {
				MIRValueHint::Pointer(_) => ctx.mir_ctx.ssa_hints.vec[value.get_ssa_index()] = MIRValueHint::Pointer(new_type),
				MIRValueHint::Value(_) => ctx.mir_ctx.ssa_hints.vec[value.get_ssa_index()] = MIRValueHint::Value(new_type),
				_ => panic!("constant enum cast")
			}

			return Ok(value);
		}

		panic!("Bad castt {:#?} -> {:#?}", old_type, new_type);
	}

	panic!("Invalid node or cast!")
}