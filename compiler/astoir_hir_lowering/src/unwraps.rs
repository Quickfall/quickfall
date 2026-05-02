use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
    scope::key::EntryKey,
};
use diagnostics::DiagnosticResult;

use crate::{types::lower_ast_type, values::lower_ast_value};

pub fn lower_ast_unwrap_condition(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::UnwrapCondition {
        original,
        target_type,
        unsafe_unwrap,
        target_var,
    } = node.kind.clone()
    {
        let target_type = lower_ast_type(context, target_type, None, &*node)?;
        let original = lower_ast_value(context, func_key, original)?;
        let mut id: Option<Result<usize, ()>> = None;

        if target_var.is_some() {
            let target_var = target_var.clone().unwrap();

            context
                .scope
                .modify_function(func_key.unwrap(), &*node, |f| {
                    id = Some(f.ctx.as_mut().unwrap().introduce_variable(
                        target_var.val.clone(),
                        target_type.clone(),
                        true,
                    ));
                })?;
        }

        let id = match id {
            Some(v) => Some(v.unwrap()),
            None => None,
        };

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::UnwrapCondition {
                original,
                new_type: target_type,
                new_var: id,
                new_var_name: target_var,
                unsafe_unwrap,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node!")
}

pub fn lower_ast_unwrap_value(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::UnwrapValue {
        original,
        target_type,
        unsafe_unwrap,
    } = node.kind.clone()
    {
        let target_type = lower_ast_type(context, target_type, None, &*node)?;
        let original = lower_ast_value(context, func_key, original)?;

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::UnwrapValue {
                original,
                new_type: target_type,
                unsafe_unwrap,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node!")
}
