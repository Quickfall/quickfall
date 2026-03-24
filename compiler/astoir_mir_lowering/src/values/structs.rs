use astoir_hir::nodes::HIRNode;
use astoir_mir::{blocks::refer::MIRBlockReference, builder::build_static_struct_const, vals::structs::MIRStructValue};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, lower_hir_type, values::lower_hir_value};

pub fn lower_hir_struct_init(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<MIRStructValue> {
	if let HIRNode::StructVariableInitializerValue { t, fields } = *node {
		let mut values = vec![];
	
		for field in fields {
			values.push(lower_hir_value(block, field, ctx)?);
		}

		let lowered_type = lower_hir_type(ctx, t)?.get_generic(&ctx.hir_ctx.type_storage);

		println!("{:#?}", lowered_type);

		return build_static_struct_const(&mut ctx.mir_ctx, lowered_type, values);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}