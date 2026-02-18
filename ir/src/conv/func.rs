use commons::err::{PositionlessError, PositionlessResult};
use parser::ast::tree::ASTTreeNode;

use crate::{conv::val::parse_ir_value, ctx::IRContext, irstruct::{funcs::IRFunction, ptr::IRPointer}, refs::IRValueRef, types::typing::IRType};

pub fn parse_ir_function_decl<'a>(ctx: &'a IRContext<'a>, node: Box<ASTTreeNode>) -> PositionlessResult<IRFunction<'a>> {
	if let ASTTreeNode::FunctionDeclaration { func_name, args, body, returnType } = *node {
		let return_type = match returnType {
			Some(h) => ctx.type_storage.get(h),
			None => None
		};

		let mut arguments: Vec<&'a IRType<'a>> = vec![];

		for k in args {
			let t = match ctx.type_storage.get(k.argument_type) {
				Some(v) => v,
				None => return Err(PositionlessError::new(&format!("Cannot get type with hash {} for argument {}!", k.argument_type, k.name.val)))
			};

			arguments.push(t);
		}

		let func = IRFunction::create(ctx, func_name.val, &ctx.module, return_type, arguments)?;

		return Ok(func);
	}

	return Err(PositionlessError::new("Given node in parse_ir_function_decl wasn't a function decl!"));
}

pub fn parse_ir_function_body_member<'a>(ctx: &'a IRContext<'a>, func: &'a mut IRFunction<'a>, node: Box<ASTTreeNode>) -> PositionlessResult<bool> {

	let mut lctx = func.lctx.borrow_mut();

	match *node {
		ASTTreeNode::VarDeclaration { var_name, var_type, value } => {
			let var_t = match ctx.type_storage.get(var_type) {
				Some(v) => v,
				None => return Err(PositionlessError::new(&format!("Cannot find variable type {} in type storage!", var_name.val)))
			};

			{
				let initial = if let Some(v) = value {
					Some(parse_ir_value(&mut lctx, ctx, v, None)?)
				} else {
					None
				};
			}

			let ptr = IRPointer::create(ctx, var_name.val.clone(), var_t, None)?;

			let ptr = IRPointer::create(ctx, var_name.val, var_t, None)?;

			lctx.add_variable(var_name.hash, ptr);

			return Ok(true);
		},

		_ => return Err(PositionlessError::new("Cannot parse said ASTNode as a function body member!"))
	};
}