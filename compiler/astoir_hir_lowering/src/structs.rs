use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::HIRNode, structs::HIRStructContainer};
use compiler_errors::{IR_TYPE_WRONG_KIND, errs::{CompilerResult, ErrorKind, normal::CompilerError}};
use compiler_typing::{raw::RawType, structs::RawStructTypeContainer};
use compiler_utils::utils::indexed::IndexStorage;

use crate::{lower_ast_body, types::{lower_ast_type_struct}};

fn lower_ast_struct_member(context: &mut HIRContext, node: Box<ASTTreeNode>, container: &mut RawStructTypeContainer) -> CompilerResult<bool> {
	if let ASTTreeNodeKind::StructFieldMember { name, member_type } = node.kind.clone() {
		let t = match lower_ast_type_struct(context, member_type, container) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		container.fields.append(name.hash, t);
		return Ok(true);
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
}

fn lower_ast_struct_function_decl(context: &mut HIRContext, node: Box<ASTTreeNode>, container: &mut RawStructTypeContainer) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::FunctionDeclaration { func_name, args, body, return_type, requires_this } = node.kind.clone() {
		let mut arguments = vec![];

		for arg in args {
			let lowered = match lower_ast_type_struct(context, arg.argument_type, container) {
				Ok(v) => v,
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			};

			arguments.push((arg.name.hash, lowered));
		}

		let ret_type;

		if return_type.is_some() {
			let lowered = match lower_ast_type_struct(context, return_type.unwrap(), container) {
				Ok(v) => v,
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			};

			ret_type = Some(lowered)
		} else {
			ret_type = None;
		}

		let mut curr_ctx = HIRBranchedContext::new();
		let body = lower_ast_body(context, &mut curr_ctx, body, true)?;

		let ind = container.functions.append(func_name.hash, (arguments.clone(), ret_type.clone()));

		return Ok(Box::new(HIRNode::StructFunctionDeclaration { func_name: ind, arguments, return_type: ret_type, body, ctx: curr_ctx, requires_this }))		
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_struct_declaration(context: &mut HIRContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::StructLayoutDeclaration { name, layout, members, type_params } = node.kind.clone() {
		let mut container = RawStructTypeContainer { fields: IndexStorage::new(), functions: IndexStorage::new(), type_params };

		let mut func_impls = vec![];

		let base = RawType::Struct(layout, container.clone());

		let ind = match context.type_storage.append(name.hash, base) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		for member in members {
			match &member.kind {
				&ASTTreeNodeKind::StructFieldMember { .. } => {
					lower_ast_struct_member(context, member, &mut container)?;
				
					context.type_storage.types.vals[ind] = RawType::Struct(layout, container.clone());
				},
				&ASTTreeNodeKind::FunctionDeclaration { .. } => {
					let body = lower_ast_struct_function_decl(context, member, &mut container)?;

					context.type_storage.types.vals[ind] = RawType::Struct(layout, container.clone());

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

pub fn lower_ast_struct_initializer(context: &mut HIRContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::StructVariableInitializerValue { struct_type, map } = node.kind.clone() {
		let struct_container = context.type_storage.get_type(struct_type.hash)?;
	}
}