use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::{HIRContext, local::BranchedContext},
    func::HIRFunction,
    nodes::HIRNode,
};
use diagnostics::DiagnosticResult;

pub fn lower_ast_function_declaration(
    ctx: &mut HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::FunctionDeclaration {
        func_name,
        args,
        body,
        return_type,
        requires_this,
    } = node.kind.clone()
    {
        let branched = BranchedContext::new();
    }

    panic!("Invalid node")
}
