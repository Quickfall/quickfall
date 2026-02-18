//! AST value -> IR value conversion

use commons::err::{PositionedError, PositionedResult, PositionlessError, PositionlessResult};
use parser::ast::tree::ASTTreeNode;

use crate::{ctx::{IRContext, IRLocalContext}, irstruct::{ptr::IRPointer, staticvars::IRStaticVariable}, refs::IRValueRef, types::{POINTER_TYPE_HASH, SIGNED64_TYPE_HASH}, values::IRValue};

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

pub fn parse_ir_value<'a>(lctx: &'a IRLocalContext<'a>, ctx: &'a IRContext<'a>, node: Box<ASTTreeNode>) -> PositionlessResult<IRValueRef<'a>> {
	match node.as_ref() {
		ASTTreeNode::IntegerLit(v) => {
			let t = ctx.type_storage.get(SIGNED64_TYPE_HASH);

			if !t.is_some() {
				return Err(PositionlessError::new("Invalid type storage! si64 not found!"));
			}

			return Ok(IRValueRef::from_val(IRValue::from_signed(t.unwrap(), *v as i128)?));
		},

		ASTTreeNode::StringLit(v) => {
			let t = ctx.type_storage.get(POINTER_TYPE_HASH);

			if !t.is_some() {
				return Err(PositionlessError::new("Invalid type storage! pointer not found!"));
			}

			let global = IRStaticVariable::from_str(&ctx.builder, v, String::from("__string_literal"), t.unwrap())?;

			return Ok(IRValueRef::from_static(global));
		},

		ASTTreeNode::VariableReference(e) => {
			let var = get_variable_ref(lctx, ctx, e.hash)?;

			return Ok(var);
		},

		ASTTreeNode::FunctionCall { func, args } => {
			let mut arguments = vec![];

			// TODO: support struct functions here

			for arg in &args[0..args.len()] {
				arguments.push(parse_ir_value(lctx, ctx, arg.clone())?);
			}

			let f = ctx.get_funtion(func.hash)?;

			let res = f.call(ctx, arguments, true)?;

			if res.is_none() {
				return Err(PositionlessError::new(&format!("Cannot use the result of function {} as a value as it is void!", func.val)));
			}

			return Ok(IRValueRef::from_pointer(res.unwrap()));
		}

		_ => return Err(PositionlessError::new("The given node cannot be parsed as a value!"))
	}
}