use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
    scope::key::EntryKey,
};
use diagnostics::DiagnosticResult;

use crate::{
    arrays::lower_ast_array_modify,
    funcs::lower_ast_function_call,
    math::lower_ast_math_operation,
    values::lower_ast_value,
    vars::{lower_ast_variable_assign, lower_ast_variable_declaration},
};

pub fn lower_ast_body_node(
    context: &mut HIRContext,
    func_key: &EntryKey,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    match node.kind {
        ASTTreeNodeKind::VarDeclaration { .. } => {
            lower_ast_variable_declaration(context, func_key, node)
        }

        ASTTreeNodeKind::MathResult { .. } => {
            lower_ast_math_operation(context, Some(func_key), node, true)
        }

        ASTTreeNodeKind::FunctionCall { .. } => {
            lower_ast_function_call(context, Some(func_key), node)
        }

        ASTTreeNodeKind::VarValueChange { .. } => {
            lower_ast_variable_assign(context, func_key, node)
        }

        ASTTreeNodeKind::ArrayIndexModifiy { .. } => {
            lower_ast_array_modify(context, func_key, node)
        }

        ASTTreeNodeKind::ReturnStatement { val } => {
            let v;

            if val.is_some() {
                v = Some(lower_ast_value(context, Some(func_key), val.unwrap())?);
            } else {
                v = None;
            }

            return Ok(Box::new(HIRNode::new(
                HIRNodeKind::ReturnStatement { value: v },
                &node.start,
                &node.end,
            )));
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
