use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode};
use compiler_errors::{IR_FIND_ELEMENT, IR_FUNCTION_INVALID_ARGUMENTS, IR_INVALID_NODE_TYPE, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::{lower_ast_body, types::lower_ast_type, values::lower_ast_value};

pub fn lower_ast_function_call(context: &HIRContext, curr_ctx: &HIRBranchedContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::FunctionCall { func, args } = node.kind.clone() {
		let f_ind = match context.functions.get_index(func.hash) {
			Some(v) => v,
			None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_ELEMENT!().to_string(), &node.start, &node.end))
		};

		let func = &context.functions.vals[f_ind];
		let mut hir_args = vec![];
		let mut ind = 0;

		for ast in args {
			let hir = lower_ast_value(context, curr_ctx, ast)?;

			if !hir.get_node_type(context, curr_ctx).unwrap().can_transmute_into(&func.1[ind].1) {
				return Err(CompilerError::from_ast(ErrorKind::Error, IR_FUNCTION_INVALID_ARGUMENTS!().to_string(), &node.start, &node.end));
			}

			hir_args.push(hir);

			ind += 1;
		}

		return Ok(Box::new(HIRNode::FunctionCall { func_name: f_ind, arguments: hir_args }))
	}
	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_function_declaration(context: &mut HIRContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::FunctionDeclaration { func_name, args, body, return_type, requires_this } = node.kind {
		let ret_type;

		if return_type.is_some() {
			let lower = match lower_ast_type(context, return_type.unwrap()) {
				Ok(v) => v,
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			};
			
			ret_type = Some(lower)
		} else {
			ret_type = None;
		}

		let mut arguments = vec![];
		let mut types = vec![];

		for arg in args {
			types.push(arg.argument_type.clone());
			let t = match lower_ast_type(context, arg.argument_type) {
				Ok(v) => v,
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			};

			arguments.push((arg.name.hash, t));
		}

		let mut curr_ctx = HIRBranchedContext::new();

		let branch = curr_ctx.start_branch();

		for arg in &arguments {
			match curr_ctx.introduce_variable(arg.0, arg.1.clone(), true) {
				Ok(_) => {},
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			}
		}

		let body = lower_ast_body(context, &mut curr_ctx, body, false)?;

		curr_ctx.end_branch(branch);

		let ind = context.functions.append(func_name.hash, (ret_type.clone(), arguments.clone()));

		for var in 0..curr_ctx.variables.len() {
			if curr_ctx.is_eligible_for_ssa(var) {
				println!("* Function variable {} is eligible for SSA treatment!", var);
			}
		}

		return Ok(Box::new(HIRNode::FunctionDeclaration { func_name: ind, arguments, return_type: ret_type, body, ctx: curr_ctx, requires_this }))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_shadow_function_declaration(context: &mut HIRContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::ShadowFunctionDeclaration { func_name, args, return_type } = node.kind.clone() {
		let ret_type;

		if return_type.is_some() {
			let lower = match lower_ast_type(context, return_type.unwrap()) {
				Ok(v) => v,
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			};
			
			ret_type = Some(lower)
		} else {
			ret_type = None;
		}

		let mut arguments = vec![];
		let mut types = vec![];

		for arg in args {
			types.push(arg.argument_type.clone());
			let t = match lower_ast_type(context, arg.argument_type) {
				Ok(v) => v,
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			};

			arguments.push((arg.name.hash, t));
		}

		let ind = context.functions.append(func_name.hash, (ret_type.clone(), arguments.clone()));

		return Ok(Box::new(HIRNode::ShadowFunctionDeclaration { func_name: ind, arguments, return_type: ret_type }))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_INVALID_NODE_TYPE!().to_string(), &node.start, &node.end))
}