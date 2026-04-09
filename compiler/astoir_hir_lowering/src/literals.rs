use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::HIRContext, nodes::{HIRNode, HIRNodeKind}};
use compiler_typing::tree::Type;
use diagnostics::{DiagnosticResult, builders::make_cannot_find_type};

pub fn lower_ast_literal(context: &HIRContext, node: Box<ASTTreeNode>) -> DiagnosticResult<Box<HIRNode>> {
	match node.kind {
		ASTTreeNodeKind::IntegerLit { val, hash } => {
			let lit_type = match context.type_storage.get_type(hash) {
				Ok(v) => v,
				Err(_) => return Err(make_cannot_find_type(&*node, &hash).into())
			};

			return Ok(Box::new(HIRNode::new(HIRNodeKind::IntegerLiteral { value: val, int_type: Type::Generic(lit_type, vec![], vec![]) }, &node.start, &node.end)))
		}, 

		ASTTreeNodeKind::StringLit(val) => {
			return Ok(Box::new(HIRNode::new(HIRNodeKind::StringLiteral { value: val }, &node.start, &node.end)))
		},

		_ => panic!("Invalid note type")
	}
}