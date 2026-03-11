//! Variable related lowering

use astoir_hir::nodes::HIRNode;
use astoir_mir::{blocks::MIRBlock, builder::{build_stack_alloc, build_store}, lower_astoir_typing_type, vals::ptr::MIRPointerValue};
use compiler_errors::{IR_INVALID_NODE_TYPE, errs::{BaseResult, base::BaseError}};

pub fn lower_hir_variable_declaration(block: &mut MIRBlock, node: Box<HIRNode>) -> BaseResult<MIRPointerValue> {
	if let HIRNode::VarDeclaration { variable, var_type, default_val } = *node {
		let lowered = lower_astoir_typing_type(var_type.get_concrete().clone())?;
		let size = var_type.get_concrete().base.get_size()?; // TODO: normalize MIR's typing system to strictly use astoir_typing to avoid lowering issues

		let ptr = build_stack_alloc(block, size, lowered)?;
		
		if block.ctx.pointer_vals.len() != variable {
			return Err(BaseError::err("lower_hir_variable_declaration possibly skipped a variable!".to_string()));
		}
		
		block.ctx.pointer_vals[variable] = ptr.clone();

		// TODO: parse values

		//if default_val.is_some() {
		//	build_store(block, ptr.clone(), val)
		//}

		return Ok(ptr)
	}

	return Err(BaseError::err(IR_INVALID_NODE_TYPE!().to_string()))
}