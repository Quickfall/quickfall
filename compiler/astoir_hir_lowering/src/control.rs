use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::{self, HIRContext},
    func,
    nodes::{HIRNode, HIRNodeKind},
    scope::key::EntryKey,
};
use diagnostics::{DiagnosticResult, diagnostic::Diagnostic};

use crate::{
    body::lower_ast_body, booleans::lower_ast_boolean_condition, math::lower_ast_math_operation,
    vars::lower_ast_variable_declaration,
};

pub fn lower_ast_for_block(
    context: &mut HIRContext,
    func_key: &EntryKey,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::ForBlock {
        initial_state,
        cond,
        increment,
        body,
    } = node.kind.clone()
    {
        let mut branch = 0;

        context.scope.modify_function(func_key, &*node, |f| {
            branch = f.ctx.as_mut().unwrap().start_branch()
        })?;

        let initial_state = lower_ast_variable_declaration(context, func_key, initial_state)?;
        let condition = lower_ast_boolean_condition(context, Some(func_key), cond)?;
        let increment = lower_ast_math_operation(context, Some(func_key), increment, true)?;

        let body = lower_ast_body(context, func_key, body)?;

        context.scope.modify_function(func_key, &*node, |f| {
            f.ctx.as_mut().unwrap().end_branch(branch);
        })?;

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ForBlock {
                initial_value: initial_state,
                condition,
                incrementation: increment,
                body,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!()
}

pub fn lower_ast_while_block(
    context: &mut HIRContext,
    func_key: &EntryKey,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::WhileBlock { cond, body } = node.kind.clone() {
        let condition = lower_ast_boolean_condition(context, Some(func_key), cond)?;

        let mut branch = 0;

        context.scope.modify_function(func_key, &*node, |f| {
            branch = f.ctx.as_mut().unwrap().start_branch()
        })?;

        let body = lower_ast_body(context, func_key, body)?;

        context.scope.modify_function(func_key, &*node, |f| {
            f.ctx.as_mut().unwrap().end_branch(branch);
        })?;

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::WhileBlock { condition, body },
            &node.start,
            &node.end,
        )));
    }

    panic!()
}
