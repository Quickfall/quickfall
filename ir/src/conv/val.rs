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

pub fn parse_ir_value<'a>(lctx: &'a IRLocalContext<'a>, ctx: &'a IRContext<'a>, node: Box<ASTTreeNode>, left: Option<IRPointer<'a>>) -> PositionlessResult<IRValueRef<'a>> {
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
			if left.as_ref().is_some() {
				let struct_t = left.as_ref().unwrap().t.get_structured_type_descriptor()?;

				let ptr = struct_t.get_pointer_for_field_noref(ctx, left.unwrap(), e.hash)?;

				return Ok(IRValueRef::from_pointer(ptr));
			}

			let var = get_variable_ref(lctx, ctx, e.hash)?;

			return Ok(var);
		},

		ASTTreeNode::FunctionCall { func, args } => {
			let mut arguments = vec![];

			if left.as_ref().is_some() {
				arguments.push(IRValueRef::from_pointer(left.as_ref().unwrap().clone()));
			}			

			for arg in &args[0..args.len()] {
				arguments.push(parse_ir_value(lctx, ctx, arg.clone(), None)?);
			}

			let res: Option<IRPointer<'a>>;
	
			if left.is_some() {
				let descriptor = left.as_ref().unwrap().t.get_structured_type_descriptor()?;

				let f = descriptor.get_function(func.hash)?;

				res = f.call(ctx, arguments, true)?;
			} else {
				let f = ctx.get_funtion(func.hash)?;

				res = f.call(ctx, arguments, true)?;
			}

			if res.is_none() {
				return Err(PositionlessError::new(&format!("Cannot use the result of function {} as a value as it is void!", func.val)));
			}

			return Ok(IRValueRef::from_pointer(res.unwrap()));
		},

		ASTTreeNode::StructLRFunction { l, r } => {
			let l_val = parse_ir_value(lctx, ctx, l.clone(), None)?;
			let l_ptr = l_val.as_pointer()?;
			
			return parse_ir_value(lctx, ctx, r.clone(), Some(l_ptr));
		},

		ASTTreeNode::StructLRVariable { l, r } => {
			let l_val = parse_ir_value(lctx, ctx, l.clone(), None)?;
			let l_ptr = l_val.as_pointer()?;

			return parse_ir_value(lctx, ctx, r.clone(), Some(l_ptr));
		}

		_ => return Err(PositionlessError::new("The given node cannot be parsed as a value!"))
	}
}