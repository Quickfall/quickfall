use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{
    blocks::{MIRBlockVariableSSAHint, MIRBlockVariableType, refer::MIRBlockReference},
    builder::{build_stack_alloc, build_store},
};
use compiler_typing::{SizedType, TypedGlobalScopeEntry};
use diagnostics::{MaybeDiagnostic, builders::make_expected_simple_error_originless};

use crate::{
    MIRLoweringContext, lower_hir_type, type_tools::cast_to_enum_child, values::lower_hir_value,
};

pub fn handle_var_introduction_queue(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> MaybeDiagnostic {
    if let HIRNodeKind::UnwrapCondition {
        original,
        new_type,
        new_var,
        unsafe_unwrap: _,
    } = node.kind.clone()
    {
        let original = lower_hir_value(block, original, ctx)?;
        let new_type = lower_hir_type(ctx, new_type)?;
        let func = ctx.mir_ctx.block_to_func[&block];
        let new_var = new_var.unwrap();

        let fns_ind = match &ctx.hir_ctx.global_scope.scope.entries[func].entry_type {
            TypedGlobalScopeEntry::Function {
                descriptor_ind: _,
                impl_ind,
            } => impl_ind,
            TypedGlobalScopeEntry::StructFunction {
                descriptor_ind: _,
                impl_ind,
                struct_type: _,
            } => impl_ind,

            _ => {
                return Err(make_expected_simple_error_originless(
                    &"function".to_string(),
                    &ctx.hir_ctx.global_scope.scope.entries[func].entry_type,
                )
                .into());
            }
        };

        let fns = &ctx.hir_ctx.global_scope.contexts[*fns_ind];

        let eligible = fns.is_eligible_for_ssa(new_var);

        let casted = cast_to_enum_child(block, original, new_type.as_generic(), ctx, &*node)?;

        if eligible {
            ctx.mir_ctx.blocks[block].variables.insert(
                new_var,
                MIRBlockVariableSSAHint {
                    kind: MIRBlockVariableType::SSA,
                    hint: Some(casted),
                },
            );
        } else {
            let ptr = build_stack_alloc(
                &mut ctx.mir_ctx,
                new_type.get_size(&new_type, false, &ctx.hir_ctx.global_scope.scope),
                new_type,
            )?;

            ctx.mir_ctx.blocks[block].variables.insert(
                new_var,
                MIRBlockVariableSSAHint {
                    kind: MIRBlockVariableType::Pointer,
                    hint: Some(ptr.clone().into()),
                },
            );
            build_store(
                &mut ctx.mir_ctx,
                &ctx.hir_ctx.global_scope.scope,
                ptr.clone(),
                casted,
            )?;
        }

        return Ok(());
    }

    panic!("Invalid node")
}
