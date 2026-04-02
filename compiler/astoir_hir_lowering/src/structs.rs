use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::{HIRBranchedContext, HIRContext}, nodes::{HIRNode, HIRNodeKind}, structs::HIRStructContainer};
use compiler_errors::{IR_TYPE_WRONG_KIND, errs::{CompilerResult, ErrorKind, normal::CompilerError}};
use compiler_typing::{raw::RawType, structs::RawStructTypeContainer, tree::Type};
use compiler_utils::{hash::{HashedString, SelfHash}, utils::indexed::IndexStorage};
use diagnostics::{DiagnosticResult, builders::{make_already_in_scope, make_cannot_find_type, make_struct_init_missing_field, make_struct_missing_field}};

use crate::{lower_ast_body, types::{lower_ast_type, lower_ast_type_struct}, values::lower_ast_value};

fn lower_ast_struct_member(context: &mut HIRContext, node: Box<ASTTreeNode>, container: &mut RawStructTypeContainer) -> DiagnosticResult<bool> {
	if let ASTTreeNodeKind::StructFieldMember { name, member_type } = node.kind.clone() {
		let t = lower_ast_type_struct(context, member_type, container, &node)?;

		container.fields.append(name.hash, t);
		return Ok(true);
	}

	panic!("Invalid node type")
}

fn lower_ast_struct_function_decl(context: &mut HIRContext, node: Box<ASTTreeNode>, container: &mut RawStructTypeContainer) -> DiagnosticResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::FunctionDeclaration { func_name, args, body, return_type, requires_this } = node.kind.clone() {
		let mut arguments = vec![];

		for arg in args {
			let lowered = lower_ast_type_struct(context, arg.argument_type, container, &node)?;

			arguments.push((arg.name.hash, lowered));
		}

		let ret_type;

		if return_type.is_some() {
			let lowered = lower_ast_type_struct(context, return_type.unwrap(), container, &node)?;

			ret_type = Some(lowered)
		} else {
			ret_type = None;
		}

		let mut curr_ctx = HIRBranchedContext::new();
		let body = lower_ast_body(context, &mut curr_ctx, body, true)?;

		let ind = container.functions.append(func_name.hash, (arguments.clone(), ret_type.clone()));

		return Ok(Box::new(HIRNode::new(HIRNodeKind::StructFunctionDeclaration { func_name: ind, arguments, return_type: ret_type, body, ctx: curr_ctx, requires_this }, &node.start, &node.end)))		
	}

	panic!("Invalid node type")
}

pub fn lower_ast_struct_declaration(context: &mut HIRContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::StructLayoutDeclaration { name, layout, members, type_params } = node.kind.clone() {
		let mut container = RawStructTypeContainer { fields: IndexStorage::new(), functions: IndexStorage::new(), type_params };

		let mut func_impls = vec![];

		let base = RawType::Struct(layout, container.clone());

		let ind = match context.type_storage.append(name.hash, base) {
			Ok(v) => v,
			Err(e) => return Err(make_already_in_scope(&node, &name.val).into())
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

				_ => panic!("Invalid node type")
			};
		}

		context.struct_func_impls.insert(ind, HIRStructContainer { function_impls: func_impls });

		return Ok(Box::new(HIRNode::new(HIRNodeKind::StructDeclaration { type_name: ind, container, layout }, &node.start, &node.end)));
	}

	panic!("Invalid node type")
}

pub fn lower_ast_struct_initializer(context: &mut HIRContext, curr_ctx: &mut HIRBranchedContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::StructVariableInitializerValue { struct_type, map } = node.kind.clone() {
		let raw = match context.type_storage.get_type(HashedString::new(struct_type.get_generic_name()).hash) {
			Ok(v) => Type::GenericLowered(v),
			Err(e) => return Err(make_cannot_find_type(&node, &struct_type.get_generic_name()).into())
		};

		let hir_type = lower_ast_type(context, struct_type, &node)?;

		let fields = raw.get_fields(&context.type_storage);

		let mut vals = vec![];

		for field in fields {
			let id = SelfHash { hash: field };

			if !map.contains_key(&id) {
				return Err(make_struct_init_missing_field(&node, &hir_type, &field).into())
			}

			let field = match raw.get_field(&context.type_storage, field) {
				Ok(v) => v,
				Err(_) => return Err(make_struct_missing_field(&hir_type, ty, &id.hash).into())
			};

			let tt = field.1.resolve(&hir_type);

			let val = lower_ast_value(context, curr_ctx, map[&id].clone())?;

			let val = Box::new(val.use_as(context, curr_ctx, tt.clone(), &node, None)?);

			vals.push(val)
		}

		return Ok(Box::new(HIRNode::new(HIRNodeKind::StructVariableInitializerValue { t: hir_type, fields: vals }, &node.start, &node.end)))
	}

	panic!("Invalid node type")
}