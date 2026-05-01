use ast::tree::ASTTreeNode;
use astoir_hir::{context::HIRContext, nodes::HIRNode, scope::key::EntryKey};
use diagnostics::DiagnosticResult;

pub fn lower_ast_value(
    context: &mut HIRContext,
    func_key: &EntryKey,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    todo!()
}
