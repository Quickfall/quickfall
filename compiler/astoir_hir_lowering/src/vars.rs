use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
    scope::key::EntryKey,
};
use diagnostics::{DiagnosticResult, builders::make_cannot_find};

use crate::{types::lower_ast_type, values::lower_ast_value};

pub fn lower_ast_variable_declaration(
    context: &mut HIRContext,
    func_key: &EntryKey,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::VarDeclaration {
        var_name,
        var_type,
        value,
    } = node.kind.clone()
    {
        let mut variable_id = Ok(0);
        let var_type = lower_ast_type(context, var_type, None, &*node)?;

        context.scope.modify_function(func_key, &*node, |f| {
            if let Some(ctx) = f.ctx.as_mut() {
                variable_id =
                    ctx.introduce_variable(var_name.val.clone(), var_type.clone(), value.is_some());
            }
        })?;

        let variable_id = match variable_id {
            Ok(v) => v,
            Err(_) => return Err(()),
        };

        let var_value;

        if let Some(v) = value {
            var_value = Some(lower_ast_value(context, func_key, v)?.use_as(
                context,
                Some(func_key),
                var_type.clone(),
                &*node,
            )?);
        } else {
            var_value = None;
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::VariableDeclaration {
                variable: variable_id,
                name: var_name,
                var_type,
                default_val: var_value,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node")
}

pub fn lower_ast_variable_reference(
    context: &mut HIRContext,
    func_key: &EntryKey,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::VariableReference(e) = node.kind.clone() {
        let ctx = context
            .scope
            .get(func_key, &*node)?
            .as_function(&*node)?
            .ctx
            .as_ref()
            .clone()
            .unwrap();

        if ctx.hash_to_ind.contains_key(&e) {
            return Ok(Box::new(HIRNode::new(
                HIRNodeKind::VariableReference {
                    index: ctx.hash_to_ind[&e],
                    name: e.clone(),
                    static_key: None,
                },
                &node.start,
                &node.end,
            )));
        }

        return Err(make_cannot_find(&*node, &e.val).into());
    }

    panic!("Invalid node")
}
