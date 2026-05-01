use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::{HIRContext, local::BranchedContext},
    func::HIRFunction,
    nodes::{HIRNode, HIRNodeKind},
    scope::{entry::ScopeEntry, key::EntryKey},
};
use diagnostics::DiagnosticResult;

use crate::{lower_ast_body, types::lower_ast_type};

pub fn lower_ast_function_declaration(
    ctx: &mut HIRContext,
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
        let key = EntryKey::new(func_name.clone());

        // Register pre full function

        let res = ctx
            .scope
            .append(key.clone(), ScopeEntry::new_function(hir_function), &*node)?;

        let hir_function = ctx.scope.get_function(&key, &*node)?;

        let body = lower_ast_body(ctx, body)?;

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
            None,
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
