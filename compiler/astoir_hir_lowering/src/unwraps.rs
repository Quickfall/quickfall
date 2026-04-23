use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    ctx::{HIRContext, branched::HIRBranchedContext},
    nodes::{HIRNode, HIRNodeKind},
};
use diagnostics::DiagnosticResult;

use crate::{types::lower_ast_type, values::lower_ast_value, var::lower_ast_variable_reference};

pub fn lower_ast_condition_unwrap(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::UnwrapCondition {
        original,
        target_type,
        unsafe_unwrap,
        target_var,
    } = node.clone().kind
    {
        let original = lower_ast_variable_reference(context, curr_ctx, original, true)?;
        let target_type = lower_ast_type(context, target_type, &*node)?;

        if target_var.is_none() {
            return Ok(Box::new(HIRNode::new(
                HIRNodeKind::UnwrapCondition {
                    original,
                    new_type: target_type,
                    new_var: None,
                    unsafe_unwrap,
                },
                &node.start,
                &node.end,
            )));
        }

        let ind = curr_ctx.introduce_variable_next_era(
            target_var.unwrap().hash,
            target_type.clone(),
            true,
        )?;

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::UnwrapCondition {
                original,
                new_type: target_type,
                new_var: Some(ind),
                unsafe_unwrap,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node")
}

pub fn lower_ast_unwrap_value(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::UnwrapValue {
        original,
        target_type,
        unsafe_unwrap,
    } = node.kind.clone()
    {
        let original = lower_ast_value(context, curr_ctx, original)?;
        let target_type = lower_ast_type(context, target_type, &*node)?;

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
