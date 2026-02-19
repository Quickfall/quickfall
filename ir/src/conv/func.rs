use std::rc::Rc;

use commons::err::{PositionlessError, PositionlessResult};
use parser::{ast::{func, tree::ASTTreeNode}, parse_ast_ctx};

use crate::{conv::{control::{parse_for_statement_ir, parse_if_statement_ir}, val::parse_ir_value}, ctx::{IRContext, IRLocalContext}, irstruct::{funcs::IRFunction, ptr::IRPointer}, refs::IRValueRef, types::typing::IRType};

pub fn parse_ir_shadow_function_decl(ctx: &mut IRContext, node: Box<ASTTreeNode>) -> PositionlessResult<Rc<IRFunction>> {
	if let ASTTreeNode::ShadowFunctionDeclaration { func_name, args, returnType } = *node {
		let return_type = match returnType {
			Some(h) => ctx.type_storage.get(h),
			None => None
		};

		let mut arguments: Vec<Rc<IRType>> = vec![];

		for k in args {
			let t = match ctx.type_storage.get(k.argument_type) {
				Some(v) => v,
				None => return Err(PositionlessError::new(&format!("Cannot get type with hash {} for argument {}!", k.argument_type, k.name.val)))
			};

			arguments.push(t);
		}

		let func = IRFunction::create_shadow(ctx, func_name.val.clone(), &ctx.module, return_type, arguments)?;

		ctx.add_function(func_name.hash, func)?;

		return Ok(ctx.get_funtion(func_name.hash)?);		
	}	

	return Err(PositionlessError::new("Cannot parse ir shadow funtion decl as the node is incompatible!"));
}

pub fn parse_ir_function_decl(ctx: &mut IRContext, node: Box<ASTTreeNode>) -> PositionlessResult<Rc<IRFunction>> {
	if let ASTTreeNode::FunctionDeclaration { func_name, args, body, returnType } = *node {
		let return_type = match returnType {
			Some(h) => ctx.type_storage.get(h),
			None => None
		};

		let mut arguments: Vec<Rc<IRType>> = vec![];

		for k in args {
			let t = match ctx.type_storage.get(k.argument_type) {
				Some(v) => v,
				None => return Err(PositionlessError::new(&format!("Cannot get type with hash {} for argument {}!", k.argument_type, k.name.val)))
			};

			arguments.push(t);
		}

		let mut func = IRFunction::create(ctx, func_name.val, &ctx.module, return_type, arguments)?;

		func.prepare_body_filling(ctx);
		parse_ir_body(ctx, &mut func, body, true)?;

		if func.ret_type.is_none() {
			match ctx.builder.build_return(None) {
				Ok(_) => {},
				Err(_) => return Err(PositionlessError::new("build_return on void failed!"))
			};
		}

		ctx.add_function(func_name.hash, func);

		return ctx.get_funtion(func_name.hash);
	}

	return Err(PositionlessError::new("Given node in parse_ir_function_decl wasn't a function decl!"));
}

pub fn parse_ir_body(ctx: &IRContext, func: &mut IRFunction, nodes: Vec<Box<ASTTreeNode>>, drop_body: bool) -> PositionlessResult<bool> {
	for node in nodes {
		parse_ir_function_body_member(ctx, func, node)?;
	}

	if drop_body {
		func.lctx.end_nested_body_depth();	
	}

	return Ok(true);
}

pub fn parse_ir_function_call(ctx: &IRContext, lctx: &IRLocalContext, node: Box<ASTTreeNode>, owner: Option<IRPointer>, grab_result: bool) -> PositionlessResult<Option<IRValueRef>> {
	if let ASTTreeNode::FunctionCall { func, args } = *node {
		let mut arguments = vec![];

		if owner.as_ref().is_some() {
			arguments.push(IRValueRef::from_pointer(owner.as_ref().unwrap().clone()));
		}	

		for v in args {
			arguments.push(parse_ir_value(Some(lctx), ctx, v, None, false)?);
		}

		let func = ctx.get_funtion(func.hash)?;

		let ret =func.call(ctx, arguments, grab_result)?;

		if !grab_result || ret.is_none() {
			return Ok(None);
		}

		return Ok(Some(IRValueRef::from_pointer(ret.unwrap())));
	}

	return Err(PositionlessError::new("Cannot parse ir function call as the node is not a function call"))
}

pub fn parse_ir_function_body_member(ctx: &IRContext, func: &mut IRFunction, node: Box<ASTTreeNode>) -> PositionlessResult<bool> {
	match *node {
		ASTTreeNode::VarDeclaration { var_name, var_type, value } => {
			let var_t = match ctx.type_storage.get(var_type) {
				Some(v) => v,
				None => return Err(PositionlessError::new(&format!("Cannot find variable type {} in type storage!", var_name.val)))
			};

			println!("Var name: {}", var_name.val.clone());

			let initial = if let Some(v) = value {
				Some(parse_ir_value(Some(&func.lctx), ctx, v, None, true)?)
			} else {
				None
			};

			let ptr = IRPointer::create(ctx, var_name.val.clone(), var_t, initial)?;

			func.lctx.add_variable(var_name.hash, ptr)?;

			println!("Added lctx value: {} -> {}", var_name.val.clone(), var_name.hash);

			return Ok(true);
		},

		ASTTreeNode::StructLRFunction { .. } =>  {
			parse_ir_value(Some(&func.lctx), ctx, node, None, false)?;

			return Ok(true)
		},

		ASTTreeNode::StructLRVariable { .. } => { 
			parse_ir_value(Some(&func.lctx), ctx, node, None, false)?;

			return Ok(true)
		},

		ASTTreeNode::FunctionCall { .. } => {
			parse_ir_function_call(ctx, &func.lctx, node, None, false)?;
			
			return Ok(true)

		}

		ASTTreeNode::IfStatement { .. } => {
			return parse_if_statement_ir(func, ctx, node);
		},

		ASTTreeNode::ForBlock {  .. } => {
			return parse_for_statement_ir(func, ctx, node);
		}

		ASTTreeNode::MathResult { lval: _, rval: _ , operator: _, assigns } => {
			if !assigns {
				return Err(PositionlessError::new("Cannot use a math expression in IR body if it is not assignments!"))
			}

			parse_ir_value(Some(&func.lctx), ctx, node, None, false)?;
			return Ok(true);
		}

		_ => return Err(PositionlessError::new("Cannot parse said ASTNode as a function body member!"))
	};
}