//! AST value -> IR value conversion

use commons::err::{PositionedError, PositionedResult, PositionlessError, PositionlessResult};
use parser::ast::tree::ASTTreeNode;

use crate::{ctx::{IRContext, IRLocalContext}, irstruct::{ptr::IRPointer, staticvars::IRStaticVariable}, refs::IRValueRef};

pub fn get_variable_ref<'a>(lctx: &'a IRLocalContext<'a>, ctx: &'a IRContext<'a>, hash: u64) -> PositionlessResult<IRValueRef<'a>> {
	match ctx.get_variable(hash) {
		Ok(v) => return Ok(IRValueRef::from_static(IRStaticVariable::clone(v))),
		Err(_) => {}
	};

	match lctx.get_variable(hash) {
		Ok(v) => return Ok(IRValueRef::from_pointer(IRPointer::clone(v))),
		Err(_) => return Err(PositionlessError::new(&format!("Cannot find variable with hash {} in the current context", hash)))
	};
}

pub fn parse_ir_chain_access<'a>(node: Box<ASTTreeNode>, lctx: &'a IRLocalContext<'a>, ctx: &'a IRContext<'a>) -> PositionlessResult<bool> {
	 
}