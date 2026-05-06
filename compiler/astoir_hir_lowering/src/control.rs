use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    ctx::{HIRBranchedContext, HIRContext},
    nodes::{HIRNode, HIRNodeKind},
    structs::HIRIfBranch,
};
use compiler_typing::{raw::RawType, tree::Type};
use diagnostics::DiagnosticResult;

use crate::{
    bools::lower_ast_condition, lower_ast_body, math::lower_ast_math_operation,
    values::lower_ast_range, var::lower_ast_variable_declaration,
};

pub fn lower_ast_for_block(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::ForBlock {
        initial_state,
        cond,
        increment,
        body,
    } = node.kind.clone()
    {
        let branch = curr_ctx.start_branch();

        let initial =
            lower_ast_variable_declaration(context, curr_ctx, initial_state, false, None)?;
        let condition = lower_ast_condition(context, curr_ctx, cond)?;

        let incrementation = lower_ast_math_operation(context, curr_ctx, increment, true)?;

        let body = lower_ast_body(context, curr_ctx, body, false)?;

        curr_ctx.end_branch(branch);

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ForBlock {
                initial_state: initial,
                condition,
                incrementation,
                body,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node passed!");
}

pub fn lower_ast_for_ranged_block(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::RangedForBlock { var, range, body } = node.kind.clone() {
        let branch = curr_ctx.start_branch();

        let ty = Type::Generic(RawType::Integer(64, false), vec![], vec![]);

        let initial =
            lower_ast_variable_declaration(context, curr_ctx, var, true, Some(ty.clone()))?;

        let range = lower_ast_range(context, curr_ctx, range, ty, &*node)?;
        let body = lower_ast_body(context, curr_ctx, body, false)?;

        curr_ctx.end_branch(branch);

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::RangedForBlock {
                variable: initial,
                range,
                body,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node!")
}

pub fn lower_ast_while_block(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::WhileBlock { cond, body } = node.kind.clone() {
        let condition = lower_ast_condition(context, curr_ctx, cond)?;

        let branch = curr_ctx.start_branch();

        let body = lower_ast_body(context, curr_ctx, body, true)?;

        curr_ctx.end_branch(branch);

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::WhileBlock { condition, body },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node passed!");
}

pub fn lower_ast_if_statement_branch(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<HIRIfBranch> {
    match node.kind {
        ASTTreeNodeKind::IfElseStatement { cond, body } => {
            let condition = lower_ast_condition(context, curr_ctx, cond.unwrap())?;
            let body = lower_ast_body(context, curr_ctx, body, true)?;

            return Ok(HIRIfBranch::ElseIfBranch {
                cond: condition,
                body,
            });
        }

        ASTTreeNodeKind::ElseStatement { body } => {
            let body = lower_ast_body(context, curr_ctx, body, true)?;

            return Ok(HIRIfBranch::ElseBranch { body });
        }

        ASTTreeNodeKind::IfStatement {
            cond,
            body,
            branches: _,
            depth: _,
        } => {
            let condition = lower_ast_condition(context, curr_ctx, cond)?;
            let body = lower_ast_body(context, curr_ctx, body, true)?;

            return Ok(HIRIfBranch::IfBranch {
                cond: condition,
                body,
            });
        }

        _ => panic!("Invalid node passed!"),
    }
}

pub fn lower_ast_if_statement(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::IfStatement {
        cond: _,
        body: _,
        branches,
        depth: _,
    } = node.kind.clone()
    {
        let mut hir_branches = vec![];

        hir_branches.push(lower_ast_if_statement_branch(
            context,
            curr_ctx,
            node.clone(),
        )?);

        for b in branches {
            hir_branches.push(lower_ast_if_statement_branch(context, curr_ctx, b)?);
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::IfStatement {
                branches: hir_branches,
            },
            &node.end,
            &node.start,
        )));
    }

    panic!("Invalid node passed!");
}
