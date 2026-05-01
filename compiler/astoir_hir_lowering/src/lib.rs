use ast::{ctx::ParserCtx, tree::ASTTreeNode};
use astoir_hir::{context::HIRContext, nodes::HIRNode};
use diagnostics::DiagnosticResult;

pub mod body;
pub mod funcs;
pub mod types;
pub mod values;
pub mod vars;

pub fn lower_ast_hir(ctx: ParserCtx) -> DiagnosticResult<HIRContext> {
    let hir = HIRContext::new();

    Ok(hir)
}
