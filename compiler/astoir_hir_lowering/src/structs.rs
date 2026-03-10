use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::HIRContext, nodes::HIRNode};
use astoir_typing::{base::BaseType, structs::StructTypeContainer};
use compiler_errors::{IR_TYPE_WRONG_KIND, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

use crate::types::lower_ast_type;

fn lower_ast_struct_member(context: &mut HIRContext, node: Box<ASTTreeNode>, container: &mut StructTypeContainer) -> CompilerResult<bool> {
	if let ASTTreeNodeKind::StructFieldMember { name, member_type } = node.kind.clone() {
		let t = match lower_ast_type(context, member_type) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		container.fields.append(name.hash, t);
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
}

pub fn lower_ast_struct_declaration(context: &mut HIRContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::StructLayoutDeclaration { name, layout, members } = node.kind.clone() {
		let mut container = StructTypeContainer::new(context.type_storage.types.len());

		for member in members {
			lower_ast_struct_member(context, member, &mut container)?;
		}

		let base = BaseType::Struct(layout, container.clone());

		let ind = match context.type_storage.register_type(name.hash, base) {
			Ok(v) => v,
			Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
		};

		return Ok(Box::new(HIRNode::StructDeclaration { type_name: ind, container, layout }));
	}

	return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
}