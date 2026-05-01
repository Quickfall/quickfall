use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{context::HIRContext, nodes::HIRNode, scope::key::EntryKey};
use diagnostics::DiagnosticResult;

use crate::vars::lower_ast_variable_declaration;

pub fn lower_ast_body_node(
    context: &mut HIRContext,
    func_key: &EntryKey,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    match node.kind {
        ASTTreeNodeKind::VarDeclaration { .. } => {
            lower_ast_variable_declaration(context, func_key, node)
        }

        _ => panic!("Invalid node!"),
    }
}

pub fn lower_ast_body(
    ctx: &mut HIRContext,
    func_key: &EntryKey,
    body: Vec<Box<ASTTreeNode>>,
) -> DiagnosticResult<Vec<Box<HIRNode>>> {
    let mut vec = vec![];

    for node in body {
        vec.push(lower_ast_body_node(ctx, func_key, node)?);
    }

    return Ok(vec);
}
