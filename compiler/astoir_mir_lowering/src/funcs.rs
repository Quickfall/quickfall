use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{
    blocks::refer::MIRBlockReference,
    builder::{build_argument_grab, build_call},
    funcs::MIRFunction,
    vals::base::BaseMIRValue,
};
use compiler_typing::TypedGlobalScopeEntry;
use diagnostics::{DiagnosticResult, builders::make_expected_simple_error_originless};

use crate::{MIRLoweringContext, body::lower_hir_body, lower_hir_type, values::lower_hir_value};

pub fn lower_hir_function_decl(
    node: Box<HIRNode>,
    cctx: &mut MIRLoweringContext,
) -> DiagnosticResult<bool> {
    if let HIRNodeKind::FunctionDeclaration {
        func_name,
        arguments,
        return_type,
        body,
        ctx: _,
        requires_this,
    } = node.kind.clone()
    {
        let mut args = vec![];

        for argument in arguments {
            args.push(lower_hir_type(cctx, argument.1)?);
        }

        let ret_type;

        if return_type.is_some() {
            ret_type = Some(lower_hir_type(cctx, return_type.unwrap())?)
        } else {
            ret_type = None
        }

        let fns_ind = match &cctx.hir_ctx.global_scope.scope.entries[func_name].entry_type {
            TypedGlobalScopeEntry::Function {
                descriptor_ind,
                impl_ind: _,
            } => descriptor_ind,
            TypedGlobalScopeEntry::ImplLessFunction(ind) => ind,
            TypedGlobalScopeEntry::StructFunction {
                descriptor_ind,
                impl_ind: _,
                struct_type: _,
            } => descriptor_ind,

            _ => {
                return Err(make_expected_simple_error_originless(
                    &"function".to_string(),
                    &cctx.hir_ctx.global_scope.scope.entries[func_name].entry_type,
                )
                .into());
            }
        };

        let fns = cctx.hir_ctx.global_scope.descriptors[*fns_ind].clone();

        let name = fns.2.clone();

        let mut func = MIRFunction::new(name, args, ret_type, requires_this, func_name);
        let block = func.append_entry_block(&mut cctx.mir_ctx);

        cctx.mir_ctx.writer.move_end(block);

        let mut ind = 0;
        for arg in &func.arguments {
            build_argument_grab(&mut cctx.mir_ctx, ind, arg.clone())?;
            ind += 1;
        }

        cctx.mir_ctx.append_function(func);

        lower_hir_body(block, body, cctx)?;

        return Ok(true);
    }

    panic!("Invalid node")
}

pub fn lower_hir_shadow_decl(
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<bool> {
    if let HIRNodeKind::ShadowFunctionDeclaration {
        func_name,
        arguments,
        return_type,
    } = node.kind.clone()
    {
        let fns_ind = match &ctx.hir_ctx.global_scope.scope.entries[func_name].entry_type {
            TypedGlobalScopeEntry::Function {
                descriptor_ind,
                impl_ind: _,
            } => descriptor_ind,
            TypedGlobalScopeEntry::ImplLessFunction(ind) => ind,
            TypedGlobalScopeEntry::StructFunction {
                descriptor_ind,
                impl_ind: _,
                struct_type: _,
            } => descriptor_ind,

            _ => {
                return Err(make_expected_simple_error_originless(
                    &"function".to_string(),
                    &ctx.hir_ctx.global_scope.scope.entries[func_name].entry_type,
                )
                .into());
            }
        };

        let fns = ctx.hir_ctx.global_scope.descriptors[*fns_ind].clone();

        let name = fns.2.clone();

        let mut args = vec![];

        for argument in arguments {
            args.push(lower_hir_type(ctx, argument.1)?);
        }

        let ret_type;

        if return_type.is_some() {
            ret_type = Some(lower_hir_type(ctx, return_type.unwrap())?)
        } else {
            ret_type = None
        }

        let func = MIRFunction::new(name, args, ret_type, false, func_name);

        ctx.mir_ctx.append_function(func);
        return Ok(true);
    }

    panic!("Invalid node")
}

pub fn lower_hir_function_call(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<Option<BaseMIRValue>> {
    if let HIRNodeKind::FunctionCall {
        func_name,
        arguments,
    } = node.kind.clone()
    {
        let mut args = vec![];

        for arg in arguments {
            let mir_val = lower_hir_value(block, arg, ctx)?;

            args.push(mir_val);
        }

        let res = build_call(&mut ctx.mir_ctx, func_name, func_name, args)?;

        if res.is_some() {
            let res = res.unwrap();

            return Ok(Some(res));
        }

        return Ok(None);
    }

    panic!("Invalid node")
}
