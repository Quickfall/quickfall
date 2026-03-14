use astoir_hir::{nodes::HIRNode};
use astoir_mir::{blocks::MIRBlock, builder::build_call, vals::base::BaseMIRValue};
use compiler_errors::{IR_FUNCTION_INVALID_ARGUMENTS, IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, values::lower_hir_value};

pub fn lower_hir_function_call(block: &mut MIRBlock, node: Box<HIRNode>, ctx: &MIRLoweringContext) -> BaseResult<BaseMIRValue> {
	if let HIRNode::FunctionCall { func_name, arguments } = *node {
		let mut args = vec![];

		let func = &ctx.mir_ctx.functions[func_name];

		let mut i = 0;
		for arg in arguments {
			let t = &func.arguments[i];
			let mir_val = lower_hir_value(block, arg, ctx)?;

			if !mir_val.vtype.can_transmute(&t) {
				return Err(BaseError::err(IR_FUNCTION_INVALID_ARGUMENTS!().to_string()))
			}

			args.push(mir_val);

			i += 1;
		}

		return build_call(&ctx.mir_ctx, block, func, func_name, args);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}