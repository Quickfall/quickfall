use ast::{ctx::ParserCtx, tree::ASTTreeNode};
use astoir_hir::{context::HIRContext, nodes::HIRNode};
use diagnostics::DiagnosticResult;

pub mod funcs;
pub mod types;

pub fn lower_ast_hir(ctx: ParserCtx) -> DiagnosticResult<HIRContext> {
    let hir = HIRContext::new();

    Ok(hir)
}

pub fn lower_ast_body_node(
    ctx: &mut HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    todo!()
}

pub fn lower_ast_body(
    ctx: &mut HIRContext,
    body: Vec<Box<ASTTreeNode>>,
) -> DiagnosticResult<Vec<Box<HIRNode>>> {
    let mut vec = vec![];

    for node in body {
        vec.push(lower_ast_body_node(ctx, node)?);
    }

    return Ok(vec);
}
