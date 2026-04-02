use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::{HIRNode, HIRNodeKind}, structs::HIRStructContainer};
use compiler_errors::{IR_TYPE_WRONG_KIND, errs::{CompilerResult, ErrorKind, normal::CompilerError}};
use compiler_typing::{raw::RawType, structs::RawStructTypeContainer, tree::Type};
use compiler_utils::{hash::{HashedString, SelfHash}, utils::indexed::IndexStorage};
use diagnostics::DiagnosticResult;

use crate::{lower_ast_body, types::{lower_ast_type, lower_ast_type_struct}, values::lower_ast_value};

fn lower_ast_struct_member(context: &mut HIRContext, node: Box<ASTTreeNode>, container: &mut RawStructTypeContainer) -> DiagnosticResult<bool> {
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

fn lower_ast_struct_function_decl(context: &mut HIRContext, node: Box<ASTTreeNode>, container: &mut RawStructTypeContainer) -> DiagnosticResult<Box<HIRNode>> {
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

		return Ok(Box::new(HIRNode::new(HIRNodeKind::StructFunctionDeclaration { func_name: ind, arguments, return_type: ret_type, body, ctx: curr_ctx, requires_this }, &node.start, &node.end)))		
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_struct_declaration(context: &mut HIRContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
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

		return Ok(Box::new(HIRNode::new(HIRNodeKind::StructDeclaration { type_name: ind, container, layout }, &node.start, &node.end)));
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_struct_initializer(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::StructVariableInitializerValue { struct_type, map } = node.kind.clone() {
		let raw = match context.type_storage.get_type(HashedString::new(struct_type.get_generic_name()).hash) {
			Ok(v) => Type::GenericLowered(v),
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		let hir_type = match lower_ast_type(context, struct_type) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		let fields = match raw.get_fields(&context.type_storage) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		let mut vals = vec![];

		for field in fields {
			let id = SelfHash { hash: field };

			if !map.contains_key(&id) {
				return Err(CompilerError::from_ast(ErrorKind::Error, "Missing field!".to_string(), &node.start, &node.end));
			}

			let field = match raw.get_field(&context.type_storage, field) {
				Ok(v) => v,
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			};

			let tt = field.1.resolve(&hir_type);

			let val = lower_ast_value(context, curr_ctx, map[&id].clone())?;

			let val = match val.use_as(context, curr_ctx, tt.clone()) {
				Ok(v) => Box::new(v),
				Err(e) =>return Err(CompilerError::from_base(e, &node.start, &node.end))
			};

			vals.push(val)
		}

		return Ok(Box::new(HIRNode::new(HIRNodeKind::StructVariableInitializerValue { t: hir_type, fields: vals }, &node.start, &node.end)))
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
}