use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    ctx::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
};
use compiler_global_scope::key::EntryKey;
use compiler_typing::tree::Type;
use diagnostics::{DiagnosticResult, builders::make_cannot_find_type};

pub fn lower_ast_literal(
    context: &HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    match node.kind {
        ASTTreeNodeKind::IntegerLit { val, hash } => {
            let lit_type = match context
                .global_scope
                .get_type(EntryKey { name_hash: hash }, &*node)
            {
                Ok(v) => v,
                Err(_) => return Err(make_cannot_find_type(&*node, &hash).into()),
            };

            return Ok(Box::new(HIRNode::new(
                HIRNodeKind::IntegerLiteral {
                    value: val,
                    int_type: Type::Generic(lit_type, vec![], vec![]),
                },
                &node.start,
                &node.end,
            )));
        }

        ASTTreeNodeKind::StringLit(val) => {
            return Ok(Box::new(HIRNode::new(
                HIRNodeKind::StringLiteral { value: val },
                &node.start,
                &node.end,
            )));
        }

        _ => panic!("Invalid note type"),
    }
}
