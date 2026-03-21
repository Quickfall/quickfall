use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{ctx::HIRContext, nodes::HIRNode};
use compiler_errors::{IR_TYPE_WRONG_KIND, errs::{CompilerResult, ErrorKind, normal::CompilerError}};

pub fn lower_ast_literal(context: &HIRContext, node: Box<ASTTreeNode>) -> CompilerResult<Box<HIRNode>> {
	match node.kind {
		ASTTreeNodeKind::IntegerLit { val, hash } => {
			let lit_type = match context.type_storage.get_type(hash) {
				Ok(v) => v,
				Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
			};

			return Ok(Box::new(HIRNode::IntegerLiteral { value: val, int_type: lit_type.0 }))
		}, 

		ASTTreeNodeKind::StringLit(val) => {
			return Ok(Box::new(HIRNode::StringLiteral { value: val }))
		},

		_ => return Err(CompilerError::from_ast(ErrorKind::Error, IR_TYPE_WRONG_KIND!().to_string(), &node.start, &node.end))
	}
}