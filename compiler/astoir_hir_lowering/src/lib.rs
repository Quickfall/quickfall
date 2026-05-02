use ast::{
    ctx::ParserCtx,
    tree::{ASTTreeNode, ASTTreeNodeKind},
};
use astoir_hir::{context::HIRContext, nodes::HIRNode};
use diagnostics::DiagnosticResult;

use crate::funcs::lower_ast_function_declaration;

pub mod arrays;
pub mod body;
pub mod booleans;
pub mod funcs;
pub mod math;
pub mod types;
pub mod values;
pub mod vars;

pub fn lower_ast_hir(ctx: ParserCtx) -> DiagnosticResult<HIRContext> {
    let mut hir = HIRContext::new();

    for key in &ctx.iter_order {
        let node = ctx.map[key].clone();

        let _ = lower_root_node(&mut hir, node)?;
    }

    Ok(hir)
}

pub fn lower_root_node(
    ctx: &mut HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    match &node.kind {
        ASTTreeNodeKind::FunctionDeclaration { .. } => {
            return lower_ast_function_declaration(ctx, node);
        }

        _ => panic!("Invalid node"),
    }
}
