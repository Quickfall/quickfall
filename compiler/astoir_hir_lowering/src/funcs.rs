use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::{HIRContext, local::BranchedContext},
    func::HIRFunction,
    nodes::{HIRNode, HIRNodeKind},
    scope::{entry::ScopeEntry, key::EntryKey},
};
use diagnostics::DiagnosticResult;
use typing::raw::RawType;

use crate::{body::lower_ast_body, types::lower_ast_type, values::lower_ast_value};

pub fn lower_ast_function_declaration(
    ctx: &mut HIRContext,
    ty: Option<RawType>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::FunctionDeclaration {
        func_name,
        args,
        body,
        return_type,
        requires_this,
    } = node.kind.clone()
    {
        let mut branched = BranchedContext::new();
        let ret_type;

        if let Some(return_type) = return_type {
            ret_type = Some(lower_ast_type(ctx, return_type, None, &*node)?);
        } else {
            ret_type = None;
        }

        let mut arguments = vec![];

        for arg in args {
            arguments.push((
                arg.name.clone(),
                lower_ast_type(ctx, arg.argument_type, None, &*node)?,
            ))
        }

        for arg in &arguments {
            branched.introduce_variable(arg.0.val.clone(), arg.1.clone(), true)?;
        }

        let hir_function = HIRFunction::new_pre_full(
            func_name.val.clone(),
            ret_type.clone(),
            arguments.clone(),
            branched,
        );

        let key;

        if ty.is_some() {
            key = EntryKey::new_linked(func_name.clone(), ty.unwrap());
        } else {
            key = EntryKey::new(func_name.clone());
        }

        // Register pre full function

        let res = ctx
            .scope
            .append(key.clone(), ScopeEntry::new_function(hir_function), &*node)?;

        let hir_function = ctx.scope.get_function(&key, &*node)?;

        let body = lower_ast_body(ctx, &key, body)?;

        let implementation = Box::new(HIRNode::new(
            HIRNodeKind::FunctionDeclaration {
                func_name: res,
                raw_name: func_name.clone(),
                arguments,
                return_type: ret_type,
                body,
                ctx: &hir_function.ctx.as_ref().unwrap(),
                requires_this,
            },
            &node.start,
            &node.end,
        ));

        ctx.scope.modify_function(&key, &*node, |func| {
            func.implementation = Some(implementation.clone());
        })?;

        return Ok(implementation);
    }

    panic!("Invalid node")
}

pub fn lower_ast_function_call(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::FunctionCall { func, args } = node.kind.clone() {
        let function_key = EntryKey::new(func);
        let func = context.scope.get_function(&function_key, &*node)?;

        let mut arguments = vec![];
        let mut i = 0;

        for argument in args {
            let expected_type = func.arguments[i].1.clone();
            let arg = lower_ast_value(context, func_key, argument.clone())?.use_as(
                context,
                func_key,
                expected_type,
                &*argument,
            )?;

            arguments.push(arg);

            i += 1;
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::FunctionCall {
                func_name: func.self_id,
                arguments,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node")
}
