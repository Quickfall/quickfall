use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::HIRContext, nodes::{HIRNode, HIRNodeKind}};
use compiler_typing::{enums::{RawEnumTypeContainer}, raw::RawType};
use diagnostics::{DiagnosticResult, MaybeDiagnostic, builders::make_already_in_scope};

use crate::types::{lower_ast_type_struct};

pub fn lower_ast_enum_entry(context: &mut HIRContext, node: Box<ASTTreeNode>, container: &mut RawEnumTypeContainer) -> MaybeDiagnostic {
	if let ASTTreeNodeKind::EnumEntryDeclaration { name, fields } = node.kind.clone() {
		let mut hir_fields = vec![];

		for f in fields {
			if let ASTTreeNodeKind::StructFieldMember { name, member_type } = f.kind {
				let t = lower_ast_type_struct(context, member_type, container, &*node)?;

				hir_fields.push((name.hash, t));
				continue;
			}	

			panic!("Invalid field node type!");
		}

		container.append_entry(name, hir_fields);
		return Ok(())
	}


	panic!("Invalid node")
}

pub fn lower_ast_enum(context: &mut HIRContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
	if let ASTTreeNodeKind::EnumDeclaration { name, entries, functions: _, type_params } = node.kind.clone() {
		let mut container = RawEnumTypeContainer::new(context.type_storage.types.vals.len(), type_params);

		for entry in entries {
			lower_ast_enum_entry(context, entry, &mut container)?;
		}

		let ind = match context.type_storage.append_with_hash(name.hash, RawType::Enum(container.clone())) {
			Ok(v) => v,
			Err(_) => return Err(make_already_in_scope(&*node, &name.val).into())
		};

		return Ok(Box::new(HIRNode::new(HIRNodeKind::EnumDeclaration { type_name: ind, container }, &node.start, &node.end)))
	}	

	panic!("Invalid node")
}