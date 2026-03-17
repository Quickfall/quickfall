use astoir_hir::{nodes::HIRNode};
use astoir_mir::{blocks::refer::MIRBlockReference, builder::build_call, funcs::MIRFunction, transmutation::transmute_value, vals::base::BaseMIRValue};
use astoir_typing::compacted::CompactedType;
use compiler_errors::{IR_FUNCTION_INVALID_ARGUMENTS, IR_INVALID_NODE_TYPE, IR_TRANSMUTATION, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, body::lower_hir_body, values::lower_hir_value};

pub fn lower_hir_function_decl(node: Box<HIRNode>, cctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	if let HIRNode::FunctionDeclaration { func_name, arguments, return_type, body, ctx, requires_this } = *node {
		let mut args = vec![];

		for argument in arguments {
			args.push(CompactedType::from(argument.1));
		}

		let ret_type;

		if return_type.is_some() {
			ret_type = Some(CompactedType::from(return_type.unwrap()))
		} else {
			ret_type = None
		}

		let mut func = MIRFunction::new(format!("func_{}", func_name), args, ret_type, requires_this);
		let block = func.append_entry_block(&mut cctx.mir_ctx)?;

		cctx.mir_ctx.writer.move_end(block);

		lower_hir_body(block, body, cctx)?;

		cctx.mir_ctx.append_function(func);
		return Ok(true)
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}


pub fn lower_hir_shadow_decl(node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	if let HIRNode::ShadowFunctionDeclaration { func_name, arguments, return_type } = *node {
		let mut args = vec![];

		for argument in arguments {
			args.push(CompactedType::from(argument.1));
		}

		let ret_type;

		if return_type.is_some() {
			ret_type = Some(CompactedType::from(return_type.unwrap()))
		} else {
			ret_type = None
		}

		let func = MIRFunction::new(format!("func_{}", func_name), args, ret_type, false);

		ctx.mir_ctx.append_function(func);
		return Ok(true);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}

pub fn lower_hir_function_call(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext, expected: Option<CompactedType>) -> BaseResult<Option<BaseMIRValue>> {
	if let HIRNode::FunctionCall { func_name, arguments } = *node {
		let mut args = vec![];

		let mut i = 0;
		for arg in arguments {
			let t = &ctx.mir_ctx.functions[func_name].arguments[i].clone();
			let mir_val = lower_hir_value(block, arg, ctx, None)?;

			if !mir_val.vtype.can_transmute(t) {
				return Err(BaseError::err(IR_FUNCTION_INVALID_ARGUMENTS!().to_string()))
			}

			args.push(mir_val);

			i += 1;
		}

		let res = build_call(&mut ctx.mir_ctx, func_name, func_name, args)?;

		if res.is_some() {
			let res = res.unwrap();

			if expected.is_some() {
				let expected = expected.unwrap();

				if !res.vtype.can_transmute(&expected) {
					return Err(BaseError::err(IR_TRANSMUTATION!().to_string()))
				}

				return Ok(Some(transmute_value(res, expected.base, &mut ctx.mir_ctx)?));
			}
		}

		return Ok(None);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}