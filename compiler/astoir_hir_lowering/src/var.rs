use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    ctx::{HIRBranchedContext, HIRContext, VariableKind, get_variable},
    nodes::{HIRNode, HIRNodeKind},
};
use compiler_global_scope::key::EntryKey;
use compiler_typing::tree::Type;
use diagnostics::{
    DiagnosticResult,
    builders::{make_expected_simple_error, make_variable_uninit},
};

use crate::{arrays::lower_ast_array_index_access, types::lower_ast_type, values::lower_ast_value};

pub fn lower_ast_variable_declaration(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
    force_default: bool,
    enforce_type: Option<Type>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::VarDeclaration {
        var_name,
        var_type,
        value,
    } = node.kind.clone()
    {
        context.global_scope.enforce_not_here(
            EntryKey {
                name_hash: var_name.hash,
            },
            &*node,
        )?;

        let lowered = lower_ast_type(context, var_type, &*node)?;

        if let Some(enforce_type) = enforce_type {
            if lowered != enforce_type {
                return Err(make_expected_simple_error(&*node, &enforce_type, &lowered).into());
            }
        }

        println!("Variable introduce: {} -> {}", var_name.hash, var_name.val);

        let name_ind = curr_ctx.introduce_variable(
            var_name.hash,
            lowered.clone(),
            value.is_some() || force_default,
        )?;

        let default_val;

        if value.is_some() {
            let hir_val = Box::new(lower_ast_value(context, curr_ctx, value.unwrap())?.use_as(
                context,
                curr_ctx,
                lowered.clone(),
                &*node,
                None,
            )?);

            default_val = Some(hir_val);
        } else {
            default_val = None;
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::VarDeclaration {
                variable: name_ind,
                var_type: lowered,
                default_val,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node passed!");
}

pub fn lower_ast_variable_reference(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
    requires_value: bool,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::VariableReference(str) = node.kind.clone() {
        let var = get_variable(context, curr_ctx, str.hash, &*node)?;

        if var.0 == VariableKind::STATIC {
            return Ok(Box::new(HIRNode::new(
                HIRNodeKind::VariableReference {
                    index: var.2,
                    is_static: true,
                },
                &node.start,
                &node.end,
            )));
        }

        if requires_value {
            if !curr_ctx.has_variable_value(var.2) {
                return Err(make_variable_uninit(&*node, &str.val).into());
            }
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::VariableReference {
                index: var.2,
                is_static: false,
            },
            &node.start,
            &node.end,
        )));
    }

    if let ASTTreeNodeKind::ArrayIndexAccess { .. } = node.kind.clone() {
        return lower_ast_array_index_access(context, curr_ctx, node);
    }

    panic!("Invalid node passed!");
}

pub fn lower_ast_variable_assign(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::VarValueChange { var, value } = node.kind.clone() {
        let value = lower_ast_value(context, curr_ctx, value)?;

        let variable_reference =
            lower_ast_variable_reference(context, curr_ctx, var.clone(), false)?;

        let value = Box::new(
            value.use_as(
                context,
                curr_ctx,
                variable_reference
                    .get_node_type(context, curr_ctx)
                    .unwrap()
                    .get_maybe_containing_type(),
                &*var,
                Some(&*node),
            )?,
        );

        let var = variable_reference.as_variable_reference();

        if !var.1 {
            curr_ctx.introduce_variable_assign(var.0);
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::VarAssigment {
                variable: var.0,
                val: value,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node passed!");
}
