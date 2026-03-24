use astoir_hir::nodes::HIRNode;
use astoir_mir::{blocks::refer::MIRBlockReference, builder::build_static_struct_const, vals::structs::MIRStructValue};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, values::lower_hir_value};

pub fn lower_hir_struct_init(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<MIRStructValue> {
	if let HIRNode::StructVariableInitializerValue { t, fields } = *node {
		let mut values = vec![];
	
		for field in fields {
			values.push(lower_hir_value(block, field, ctx)?);
		}

		return build_static_struct_const(&mut ctx.mir_ctx, t, values);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}