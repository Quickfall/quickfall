use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    ctx::{HIRBranchedContext, HIRContext},
    nodes::{HIRNode, HIRNodeKind},
};
use diagnostics::{
    DiagnosticResult,
    builders::{make_already_in_scope, make_cannot_find_func},
};

use crate::{lower_ast_body, types::lower_ast_type, values::lower_ast_value};

pub fn lower_ast_function_call(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::FunctionCall { func, args } = node.kind.clone() {
        let f_ind = match context.functions.get_index(func.hash) {
            Some(v) => v,
            None => return Err(make_cannot_find_func(&*node, &func.hash).into()),
        };

        let func = &context.functions.vals[f_ind].clone();
        let mut hir_args = vec![];
        let mut ind = 0;

        for ast in args {
            let hir = lower_ast_value(context, curr_ctx, ast)?;

            let val = hir.use_as(context, curr_ctx, func.1[ind].1.clone(), &*node, None)?;

            hir_args.push(Box::new(val));

            ind += 1;
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::FunctionCall {
                func_name: f_ind,
                arguments: hir_args,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node passed!");
}

pub fn lower_ast_function_declaration(
    context: &mut HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::FunctionDeclaration {
        func_name,
        args,
        body,
        return_type,
        requires_this,
    } = node.clone().kind
    {
        let ret_type;

        if return_type.is_some() {
            let lower = lower_ast_type(context, return_type.unwrap(), &*node)?;

            ret_type = Some(lower)
        } else {
            ret_type = None;
        }

        let mut arguments = vec![];
        let mut types = vec![];

        for arg in args {
            types.push(arg.argument_type.clone());
            let t = lower_ast_type(context, arg.argument_type, &*node)?;

            arguments.push((arg.name.hash, t));
        }

        let mut curr_ctx = HIRBranchedContext::new();

        let branch = curr_ctx.start_branch();

        for arg in &arguments {
            match curr_ctx.introduce_variable(arg.0, arg.1.clone(), true) {
                Ok(_) => {}
                Err(_) => return Err(make_already_in_scope(&*node, &arg.0).into()),
            }
        }

        let ind = context.functions.append(
            func_name.hash,
            (ret_type.clone(), arguments.clone(), func_name.val.clone()),
        );

        let body = lower_ast_body(context, &mut curr_ctx, body, false)?;

        context.function_contexts.push(Some(curr_ctx.clone()));

        curr_ctx.end_branch(branch);

        for var in 0..curr_ctx.variables.len() {
            if curr_ctx.is_eligible_for_ssa(var) {
                println!("* Function variable {} is eligible for SSA treatment!", var);
            }
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::FunctionDeclaration {
                func_name: ind,
                arguments,
                return_type: ret_type,
                body,
                ctx: curr_ctx,
                requires_this,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node passed!");
}

pub fn lower_ast_shadow_function_declaration(
    context: &mut HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::ShadowFunctionDeclaration {
        func_name,
        args,
        return_type,
    } = node.kind.clone()
    {
        let ret_type;

        if return_type.is_some() {
            let lower = lower_ast_type(context, return_type.unwrap(), &*node)?;

            ret_type = Some(lower)
        } else {
            ret_type = None;
        }

        let mut arguments = vec![];
        let mut types = vec![];

        for arg in args {
            types.push(arg.argument_type.clone());
            let t = lower_ast_type(context, arg.argument_type, &*node)?;

            arguments.push((arg.name.hash, t));
        }

        let ind = context.functions.append(
            func_name.hash,
            (ret_type.clone(), arguments.clone(), func_name.val.clone()),
        );

        context.function_contexts.push(None);

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ShadowFunctionDeclaration {
                func_name: ind,
                arguments,
                return_type: ret_type,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node passed!");
}
