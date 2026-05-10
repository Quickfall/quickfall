use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    ctx::{HIRBranchedContext, HIRContext},
    nodes::{HIRNode, HIRNodeKind},
};
use compiler_global_scope::key::EntryKey;
use compiler_typing::TypedGlobalScopeEntry;
use diagnostics::{DiagnosticResult, builders::make_already_in_scope};

use crate::{lower_ast_body, types::lower_ast_type, values::lower_ast_value};

pub fn lower_ast_function_call(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::FunctionCall { func, args } = node.kind.clone() {
        let name = EntryKey {
            name_hash: func.hash,
        };

        let func = context
            .global_scope
            .get_function_base(name.clone(), &*node)?;

        let func_ind = context.global_scope.get_ind(name, &*node)?;

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
                func_name: func_ind,
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

        let mut curr_ctx = HIRBranchedContext::new(ret_type.clone());

        let branch = curr_ctx.start_branch();

        for arg in &arguments {
            context
                .global_scope
                .enforce_not_here(EntryKey { name_hash: arg.0 }, &*node)?;

            match curr_ctx.introduce_variable(arg.0, arg.1.clone(), true) {
                Ok(_) => {}
                Err(_) => return Err(make_already_in_scope(&*node, &arg.0).into()),
            }
        }

        let key = EntryKey {
            name_hash: func_name.hash,
        };

        let entry =
            TypedGlobalScopeEntry::ImplLessFunction(context.global_scope.scope.descriptor_counter);

        let ind = context.global_scope.append_implless_function(
            key.clone(),
            (ret_type.clone(), arguments.clone(), func_name.val.clone()),
            &*node,
        )?;

        let body = lower_ast_body(context, &mut curr_ctx, body, false)?;

        curr_ctx.end_branch(branch);

        let implementation = Box::new(HIRNode::new(
            HIRNodeKind::FunctionDeclaration {
                func_name: ind,
                arguments: arguments.clone(),
                return_type: ret_type.clone(),
                body,
                ctx: curr_ctx.clone(),
                requires_this,
            },
            &node.start,
            &node.end,
        ));

        // Remove old impless version
        context.global_scope.scope.entries.pop();
        context.global_scope.scope.entry_to_ind.remove(&key);
        context.global_scope.scope.value_to_ind.remove(&entry);
        context.global_scope.descriptors.pop();
        //context.global_scope.contexts.pop();
        context.global_scope.scope.descriptor_counter -= 1;
        //context.global_scope.scope.ctx_counter -= 1;

        // Append the new verison as a impl-containing function

        context.global_scope.append_func(
            key,
            (ret_type.clone(), arguments.clone(), func_name.val.clone()),
            implementation.clone(),
            curr_ctx.clone(),
            &*node,
        )?;

        return Ok(implementation);
    }

    panic!("Invalid node passed!");
}

pub fn lower_ast_extern_function_declaration(
    context: &mut HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::ExternFunctionDeclaration {
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

        let ind = context.global_scope.append_implless_function(
            EntryKey {
                name_hash: func_name.hash,
            },
            (ret_type.clone(), arguments.clone(), func_name.val.clone()),
            &*node,
        )?;

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ExternFunctionDeclaration {
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
