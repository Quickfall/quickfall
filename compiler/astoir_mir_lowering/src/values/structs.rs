use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{blocks::refer::MIRBlockReference, builder::build_static_struct_const, vals::structs::MIRStructValue};
use diagnostics::DiagnosticResult;

use crate::{MIRLoweringContext, lower_hir_type, values::lower_hir_value};

pub fn lower_hir_struct_init(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> DiagnosticResult<MIRStructValue> {
	if let HIRNodeKind::StructVariableInitializerValue { t, fields } = node.kind {
		let mut values = vec![];
	
		for field in fields {
			values.push(lower_hir_value(block, field, ctx)?);
		}

		let lowered_type = lower_hir_type(ctx, t)?.get_generic(&ctx.hir_ctx.type_storage);

		return build_static_struct_const(&mut ctx.mir_ctx, lowered_type, values);
	}

	panic!("Invalid node")
}