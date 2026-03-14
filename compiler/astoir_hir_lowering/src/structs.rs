use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode, structs::HIRStructContainer};
use astoir_typing::{base::BaseType, structs::StructTypeContainer};
use compiler_errors::{IR_TYPE_WRONG_KIND, errs::{CompilerResult, ErrorKind, base::BaseError, normal::CompilerError}};

use crate::{func, lower_ast_body, types::lower_ast_type};

fn lower_ast_struct_member(context: &mut HIRContext, node: Box<ASTTreeNode>, container: &mut StructTypeContainer) -> CompilerResult<bool> {
	if let ASTTreeNodeKind::StructFieldMember { name, member_type } = node.kind.clone() {
		let t = match lower_ast_type(context, member_type) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		container.fields.append(name.hash, t);
		return Ok(true);
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
}

fn lower_ast_struct_function_decl(context: &mut HIRContext, node: Box<ASTTreeNode>, container: &mut StructTypeContainer) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::FunctionDeclaration { func_name, args, body, return_type, requires_this } = node.kind.clone() {
		let mut arguments = vec![];

		for arg in args {
			let lowered = match lower_ast_type(context, arg.argument_type) {
				Ok(v) => v,
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			};

			arguments.push((arg.name.hash, lowered));
		}

		let ret_type;

		if return_type.is_some() {
			let lowered = match lower_ast_type(context, return_type.unwrap()) {
				Ok(v) => v,
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			};

			ret_type = Some(lowered)
		} else {
			ret_type = None;
		}

		let mut curr_ctx = HIRBranchedContext::new();
		let body = lower_ast_body(context, &mut curr_ctx, body, true)?;

		let ind = container.functions.append(func_name.hash, (ret_type.clone(), arguments.clone()));

		return Ok(Box::new(HIRNode::FunctionDeclaration { func_name: ind, arguments, return_type: ret_type, body, ctx: curr_ctx, requires_this }))		
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_struct_declaration(context: &mut HIRContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::StructLayoutDeclaration { name, layout, members } = node.kind.clone() {
		let mut container = StructTypeContainer::new(context.type_storage.types.len());

		let mut func_impls = vec![];

		let base = BaseType::Struct(layout, container.clone());

		let ind = match context.type_storage.register_type(name.hash, base) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		for member in members {
			match &member.kind {
				&ASTTreeNodeKind::StructFieldMember { .. } => {
					lower_ast_struct_member(context, member, &mut container)?;
				
					context.type_storage.types[ind] = BaseType::Struct(layout, container.clone());
				},
				&ASTTreeNodeKind::FunctionDeclaration { .. } => {
					let body = lower_ast_struct_function_decl(context, member, &mut container)?;

					context.type_storage.types[ind] = BaseType::Struct(layout, container.clone());

					func_impls.push(body);
				},

				_ => return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
			};
		}

		context.struct_func_impls.insert(ind, HIRStructContainer { function_impls: func_impls });

		return Ok(Box::new(HIRNode::StructDeclaration { type_name: ind, container, layout }));
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
}