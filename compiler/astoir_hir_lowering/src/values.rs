use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
    scope::key::EntryKey,
};
use diagnostics::DiagnosticResult;

pub fn lower_ast_generic(
    context: &mut HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::IntegerLit { val, hash } = node.kind.clone() {
        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::IntegerLiteral(val),
            &node.start,
            &node.end,
        )));
    };

    if let ASTTreeNodeKind::StringLit(val) = node.kind.clone() {
        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::StringLiteral(val),
            &node.start,
            &node.end,
        )));
    };

    panic!("Invalid node")
}

pub fn lower_ast_value(
    context: &mut HIRContext,
    func_key: &EntryKey,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    match node.kind {
        ASTTreeNodeKind::StringLit(_) | ASTTreeNodeKind::IntegerLit { .. } => {
            return lower_ast_generic(context, node);
        }

        _ => panic!("Invalid node"),
    }
}
