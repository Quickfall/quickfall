//! Variable related lowering

use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{
    blocks::{MIRBlockVariableSSAHint, MIRBlockVariableType, refer::MIRBlockReference},
    builder::{build_stack_alloc, build_store},
    vals::{base::BaseMIRValue, refer::MIRVariableReference},
};
use compiler_typing::{SizedType, TypedGlobalScopeEntry};
use diagnostics::{
    DiagnosticResult, MaybeDiagnostic, builders::make_expected_simple_error_originless,
};

use crate::{MIRLoweringContext, lower_hir_type, values::lower_hir_value};

pub fn lower_hir_variable_declaration(
    block_id: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
    override_val: Option<BaseMIRValue>,
) -> DiagnosticResult<MIRVariableReference> {
    if let HIRNodeKind::VarDeclaration {
        variable,
        var_type,
        default_val,
    } = node.clone().kind
    {
        let func = ctx.mir_ctx.block_to_func[&block_id];

        //let local_ctx = ctx.hir_ctx.function_contexts[func].as_ref().unwrap();

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
                println!("Curr entry: {}", func);
                println!("Entry dump: ");

                for entry in ctx.hir_ctx.global_scope.scope.entries.clone() {
                    println!("- {:#?}", entry);
                }

                return Err(make_expected_simple_error_originless(
                    &"function".to_string(),
                    &ctx.hir_ctx.global_scope.scope.entries[func].entry_type,
                )
                .into());
            }
        };

        let local_ctx = ctx.hir_ctx.global_scope.contexts[*fns_ind].clone();

        if local_ctx.is_eligible_for_ssa(variable) {
            if default_val.is_some() {
                let val = lower_hir_value(block_id, default_val.unwrap(), ctx)?;

                ctx.mir_ctx.blocks[block_id].variables.insert(
                    variable,
                    MIRBlockVariableSSAHint {
                        kind: MIRBlockVariableType::SSA,
                        hint: Some(val),
                    },
                );
            } else {
                ctx.mir_ctx.blocks[block_id].variables.insert(
                    variable,
                    MIRBlockVariableSSAHint {
                        kind: MIRBlockVariableType::SSA,
                        hint: None,
                    },
                );
            }

            return Ok(MIRVariableReference::SSAReference(variable));
        } else {
            let lowered = lower_hir_type(ctx, var_type)?;

            let ptr = build_stack_alloc(
                &mut ctx.mir_ctx,
                lowered.get_size(&lowered, false, &ctx.hir_ctx.global_scope.scope),
                lowered,
            )?;

            ctx.mir_ctx.blocks[block_id].variables.insert(
                variable,
                MIRBlockVariableSSAHint {
                    kind: MIRBlockVariableType::Pointer,
                    hint: Some(ptr.clone().into()),
                },
            );

            if !default_val.is_some() && override_val.is_some() {
                let val = override_val.unwrap();

                build_store(
                    &mut ctx.mir_ctx,
                    &ctx.hir_ctx.global_scope.scope,
                    ptr.clone(),
                    val,
                )?;
            }

            if default_val.is_some() {
                let val = lower_hir_value(block_id, default_val.unwrap(), ctx)?;

                build_store(
                    &mut ctx.mir_ctx,
                    &ctx.hir_ctx.global_scope.scope,
                    ptr.clone(),
                    val,
                )?;
            }

            return Ok(MIRVariableReference::PointerReference(ptr.clone()));
        }
    }

    panic!("Invalid node")
}

pub fn lower_hir_variable_reference(
    block: MIRBlockReference,
    node: &Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<MIRVariableReference> {
    if let HIRNodeKind::VariableReference {
        index,
        is_static: _,
    } = &node.kind
    {
        // TODO: add support for static variables
        return ctx.mir_ctx.blocks[block].get_variable_ref(*index);
    }

    panic!("Invalid node")
}

pub fn lower_hir_deref_modify(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> MaybeDiagnostic {
    if let HIRNodeKind::DereferenceModify { pointer, val } = node.kind.clone() {
        let ptr = lower_hir_value(block, pointer, ctx)?.as_ptr()?;
        let val = lower_hir_value(block, val, ctx)?;

        println!(
            "ptr: {}, val: {}",
            BaseMIRValue::from(ptr.clone().into()).vtype,
            val.vtype
        );

        let var = MIRVariableReference::PointerReference(ptr);
        var.write(
            block,
            &mut ctx.mir_ctx,
            val,
            &ctx.hir_ctx.global_scope.scope,
        )?;

        return Ok(());
    }

    panic!("Invalid node")
}

/// Lowers the HIR variable reference as if to obtain it's value. Requires a load
pub fn lower_hir_variable_reference_value(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<BaseMIRValue> {
    let ptr = lower_hir_variable_reference(block, &node, ctx)?;

    let read = ptr.read(block, &mut ctx.mir_ctx)?;

    return Ok(read);
}

pub fn lower_hir_variable_assignment(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<bool> {
    if let HIRNodeKind::VarAssigment { variable, val } = node.clone().kind {
        let variable_ref = ctx.mir_ctx.blocks[block].get_variable_ref(variable)?;

        let val = lower_hir_value(block, val, ctx)?;

        variable_ref.write(
            block,
            &mut ctx.mir_ctx,
            val,
            &ctx.hir_ctx.global_scope.scope,
        )?;
        return Ok(true);
    }

    panic!("Invalid node")
}
