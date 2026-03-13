use astoir_hir::{ctx::HIRContext, nodes::HIRNode};
use astoir_mir::{blocks::MIRBlock, ctx::MIRContext, vals::base::BaseMIRValue};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::values::lower_hir_value;

pub fn lower_hir_function_call(block: &mut MIRBlock, mirctx: &MIRContext, node: Box<HIRNode>, ctx: &HIRContext) -> BaseResult<BaseMIRValue> {
	if let HIRNode::FunctionCall { func_name, arguments } = *node {
		let mut args = vec![];

		let func = &mirctx.functions[func_name];

		for (arg, t) in (arguments, func.arguments) {
			let mir_val = lower_hir_value(block, arg, ctx)?;

			if mir_val.vtype.
		}

		

	
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}