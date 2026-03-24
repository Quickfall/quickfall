use astoir_hir::{nodes::HIRNode};
use astoir_mir::{blocks::refer::MIRBlockReference, builder::{build_argument_grab, build_call}, funcs::MIRFunction, vals::base::BaseMIRValue};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

use crate::{MIRLoweringContext, body::lower_hir_body, lower_hir_type, values::lower_hir_value};

pub fn lower_hir_function_decl(node: Box<HIRNode>, cctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	if let HIRNode::FunctionDeclaration { func_name, arguments, return_type, body, ctx: _, requires_this } = *node {
		let mut args = vec![];

		for argument in arguments {
			args.push(lower_hir_type(cctx, argument.1)?);
		}

		let ret_type;

		if return_type.is_some() {
			ret_type = Some(lower_hir_type(cctx, return_type.unwrap())?)
		} else {
			ret_type = None
		}

		let name = cctx.hir_ctx.functions.vals[func_name].2.clone();

		let mut func = MIRFunction::new(name, args, ret_type, requires_this, &cctx.mir_ctx);
		let block = func.append_entry_block(&mut cctx.mir_ctx)?;

		cctx.mir_ctx.writer.move_end(block);

		let mut ind = 0;
		for arg in &func.arguments {
			build_argument_grab(&mut cctx.mir_ctx, ind, arg.clone())?;
			ind += 1;
		}

		cctx.mir_ctx.append_function(func);

		lower_hir_body(block, body, cctx)?;

		return Ok(true)
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}


pub fn lower_hir_shadow_decl(node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	if let HIRNode::ShadowFunctionDeclaration { func_name, arguments, return_type } = *node {
		let name = ctx.hir_ctx.functions.vals[func_name].2.clone();

		let mut args = vec![];

		for argument in arguments {
			args.push(lower_hir_type(ctx, argument.1)?);
		}

		let ret_type;

		if return_type.is_some() {
			ret_type = Some(lower_hir_type(ctx, return_type.unwrap())?)
		} else {
			ret_type = None
		}

		let func = MIRFunction::new(name, args, ret_type, false, &ctx.mir_ctx);

		ctx.mir_ctx.append_function(func);
		return Ok(true);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}

pub fn lower_hir_function_call(block: MIRBlockReference, node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<Option<BaseMIRValue>> {
	if let HIRNode::FunctionCall { func_name, arguments } = *node {
		let mut args = vec![];

		for arg in arguments {
			let mir_val = lower_hir_value(block, arg, ctx)?;
			
			args.push(mir_val);
		}

		let res = build_call(&mut ctx.mir_ctx, func_name, func_name, args)?;

		if res.is_some() {
			let res = res.unwrap();

			return Ok(Some(res));
		}

		return Ok(None);
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}