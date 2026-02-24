use std::rc::Rc;

use errors::{INKWELL_FUNC_FAILED, IR_FIND_TYPE, IR_INVALID_NODE_TYPE, MATH_OP_NO_ASSIGN, errs::{CompilerResult, ErrorKind, normal::CompilerError}};
use parser::{ast::{func, tree::{ASTTreeNode, ASTTreeNodeKind}}, parse_ast_ctx};

use crate::{conv::{control::{parse_for_statement_ir, parse_if_statement_ir}, val::parse_ir_value}, ctx::{IRContext, IRLocalContext}, irstruct::{funcs::IRFunction, ptr::IRPointer}, refs::IRValueRef, types::typing::IRType, values::IRValue};

pub fn parse_ir_shadow_function_decl(ctx: &mut IRContext, node: Box<ASTTreeNode>) -> CompilerResult<Rc<IRFunction>> {
	if let ASTTreeNodeKind::ShadowFunctionDeclaration { func_name, args, returnType } = node.kind {
		let return_type = match returnType {
			Some(h) => ctx.type_storage.get(h),
			None => None
		};

		let mut arguments: Vec<(Rc<IRType>, u64)> = vec![];

		for k in args {
			let t = match ctx.type_storage.get(k.argument_type) {
				Some(v) => v,
				None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_TYPE!().to_string(), &node.start, &node.end))
			};

			arguments.push((t, k.name.hash));
		}

		let func = match IRFunction::create_shadow(ctx, func_name.val.clone(), func_name.hash, &ctx.module, return_type, arguments) {
			Ok(v) => v,
			Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
		};

		match ctx.add_function(func_name.hash, func) {
			Ok(_) => {},
			Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
		};

		match ctx.get_function(func_name.hash) {
			Ok(v) => return Ok(v),
			Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
		};
	}	

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end));
}

pub fn parse_ir_function_decl(ctx: &mut IRContext, node: Box<ASTTreeNode>) -> CompilerResult<Rc<IRFunction>> {
	if let ASTTreeNodeKind::FunctionDeclaration { func_name, args, body, returnType } = node.kind {
		let return_type = match returnType {
			Some(h) => ctx.type_storage.get(h),
			None => None
		};

		let mut arguments: Vec<(Rc<IRType>, u64)> = vec![];

		for k in args {
			let t = match ctx.type_storage.get(k.argument_type) {
				Some(v) => v,
				None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_TYPE!().to_string(), &node.start, &node.end))
			};

			arguments.push((t, k.name.hash));
		}

		let mut func = match IRFunction::create(ctx, func_name.val,func_name.hash, &ctx.module, return_type, arguments) {
			Ok(v) => v,
			Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
		};


		let mut ind = 0;
		for argument in &func.args {
			let val = match func.get_nth_arg(ind) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};
		

			match func.lctx.add_argument(argument.1, IRValue::new(val, argument.0.clone())) {
				Ok(_) => {},
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};
			
			ind += 1;
		}

		func.prepare_body_filling(ctx);
		parse_ir_body(ctx, &mut func, body, true)?;

		if func.ret_type.is_none() {
			match ctx.builder.build_return(None) {
				Ok(_) => {},
				Err(e) => return Err(CompilerError::from_ast(ErrorKind::Critical, format!(INKWELL_FUNC_FAILED!(), "build_return", e), &node.start, &node.end))
			};
		}

		match ctx.add_function(func_name.hash, func) {
			Ok(_) => {},
			Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
		};
		
		
		return match ctx.get_function(func_name.hash) {
			Ok(v) => Ok(v),
			Err(b) => Err(CompilerError::from_base(b, &node.start, &node.end))
		};
	
	}

	return Err(CompilerError::from_ast(ErrorKind::Critical, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end));
}

pub fn parse_ir_body(ctx: &IRContext, func: &mut IRFunction, nodes: Vec<Box<ASTTreeNode>>, drop_body: bool) -> CompilerResult<bool> {
	for node in nodes {
		parse_ir_function_body_member(ctx, func, node)?;
	}

	if drop_body {
		func.lctx.end_nested_body_depth();	
	}

	return Ok(true);
}

pub fn parse_ir_function_call(ctx: &IRContext, f: &IRFunction, node: Box<ASTTreeNode>, owner: Option<IRPointer>, grab_result: bool) -> CompilerResult<Option<IRValueRef>> {
	if let ASTTreeNodeKind::FunctionCall { func: ff, args } = node.kind {
		let mut arguments = vec![];

		if owner.as_ref().is_some() {
			arguments.push(IRValueRef::from_pointer(owner.as_ref().unwrap().clone()));
		}	

		for v in args {
			arguments.push(parse_ir_value(Some(&f), ctx, v, None, false)?);
		}


		let ret;

		if ff.hash == f.hash {
			ret = match f.call(ctx, arguments, grab_result) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};
			
		} else {
			let func = match ctx.get_function(ff.hash) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};

			ret = match func.call(ctx, arguments, grab_result) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};
			
		}

		if !grab_result || ret.is_none() {
			return Ok(None);
		}

		return Ok(Some(IRValueRef::from_val(ret.unwrap())));
	}

	return Err(CompilerError::from_ast(ErrorKind::Critical, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}

pub fn parse_ir_function_body_member(ctx: &IRContext, func: &mut IRFunction, node: Box<ASTTreeNode>) -> CompilerResult<bool> {
	match node.kind {
		ASTTreeNodeKind::VarDeclaration { var_name, var_type, value } => {
			let var_t = match ctx.type_storage.get(var_type) {
				Some(v) => v,
				None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_TYPE!().to_string(), &node.start, &node.end))
			};

			let initial = if let Some(v) = value {
				Some(parse_ir_value(Some(&func), ctx, v, None, true)?)
			} else {
				None
			};

			let ptr = match IRPointer::create(ctx, var_name.val.clone(), var_t, initial) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};

			match func.lctx.add_variable(var_name.hash, ptr) {
				Ok(v) => {},
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};

			return Ok(true);
		},

		ASTTreeNodeKind::StructLRFunction { .. } =>  {
			parse_ir_value(Some(&func), ctx, node, None, false)?;

			return Ok(true)
		},

		ASTTreeNodeKind::StructLRVariable { .. } => { 
			parse_ir_value(Some(&func), ctx, node, None, false)?;

			return Ok(true)
		},

		ASTTreeNodeKind::FunctionCall { .. } => {
			parse_ir_function_call(ctx, &func, node, None, false)?;
			
			return Ok(true)
		},

		ASTTreeNodeKind::ReturnStatement { val } => {
			if val.clone().is_none() || func.ret_type.is_none() {
				ctx.builder.build_return(None);

				return Ok(true);
			}	

			let val = parse_ir_value(Some(&func), ctx, val.unwrap(), None, true)?;

			let ob = match val.obtain(ctx) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};

			ctx.builder.build_return(Some(&ob.obtain().inner));

			return Ok(true);
		}

		ASTTreeNodeKind::IfStatement { .. } => {
			return parse_if_statement_ir(func, ctx, node);
		},

		ASTTreeNodeKind::ForBlock {  .. } => {
			return parse_for_statement_ir(func, ctx, node);
		}

		ASTTreeNodeKind::MathResult { lval: _, rval: _ , operator: _, assigns } => {
			if !assigns {
				return Err(CompilerError::from_ast(ErrorKind::Error, MATH_OP_NO_ASSIGN!().to_string(), &node.start, &node.start))
			}

			parse_ir_value(Some(&func), ctx, node, None, false)?;
			return Ok(true);
		}

		_ => return Err(CompilerError::from_ast(ErrorKind::Critical, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
	};
}