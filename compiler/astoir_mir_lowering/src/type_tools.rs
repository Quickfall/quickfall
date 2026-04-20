use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{
    blocks::refer::MIRBlockReference,
    builder::{
        build_comp_eq, build_field_pointer, build_ir_cast, build_load, build_unsigned_int_const,
    },
    vals::{base::BaseMIRValue, int::MIRIntValue},
};
use compiler_typing::{SizedType, raw::RawType, tree::Type};
use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin,
    builders::{make_req_type_kind, make_type_not_partof},
};

use crate::{
    MIRLoweringContext, lower_hir_type, values::lower_hir_value, vars::lower_hir_variable_reference,
};

pub fn is_enum_value_of_kind<K: DiagnosticSpanOrigin>(
    _block: MIRBlockReference,
    val: BaseMIRValue,
    enum_entry: RawType,
    ctx: &mut MIRLoweringContext,
    origin: &K,
) -> DiagnosticResult<MIRIntValue> {
    let enum_type = match ctx
        .mir_ctx
        .ssa_hints
        .get_hint(val.get_ssa_index())
        .get_type()
        .as_generic_lowered_safe(origin)?
    {
        RawType::Enum(v) => v,
        RawType::LoweredStruct(_, container) => {
            if !container.is_lowered_enum_parent {
                return Err(make_req_type_kind(origin, &"enum parent".to_string()).into());
            }

            container.lowered_enum_parent.unwrap()
        }
        _ => return Err(make_req_type_kind(origin, &"enum parent".to_string()).into()),
    };

    let enum_entry = match enum_entry {
        RawType::EnumEntry(v) => v,
        RawType::LoweredStruct(_, container) => {
            if !container.is_lowered_enum_child {
                return Err(make_req_type_kind(origin, &"enum child".to_string()).into());
            }

            container.lowered_enum_child.unwrap()
        }
        _ => return Err(make_req_type_kind(origin, &"enum child".to_string()).into()),
    };

    if enum_entry.parent != enum_type.self_ref {
        return Err(make_type_not_partof(origin, &enum_entry.child, &enum_type.self_ref).into());
    }

    let hint_type = enum_type.get_hint_type();

    let field_ptr = build_field_pointer(&mut ctx.mir_ctx, val.as_ptr()?, 0)?; // 0 = hint type index
    let hint_val = build_load(&mut ctx.mir_ctx, field_ptr)?.as_int()?;

    let hint_true = build_unsigned_int_const(
        &mut ctx.mir_ctx,
        enum_entry.child as u128,
        hint_type.get_size(
            &Type::GenericLowered(hint_type.clone()),
            false,
            &ctx.hir_ctx.type_storage,
        ),
    )?;

    return build_comp_eq(&mut ctx.mir_ctx, hint_val, hint_true);
}

pub fn cast_to_enum_child<K: DiagnosticSpanOrigin>(
    _block: MIRBlockReference,
    val: BaseMIRValue,
    enum_entry: RawType,
    ctx: &mut MIRLoweringContext,
    origin: &K,
) -> DiagnosticResult<BaseMIRValue> {
    let enum_type = match ctx
        .mir_ctx
        .ssa_hints
        .get_hint(val.get_ssa_index())
        .get_type()
        .as_generic_lowered_safe(origin)?
    {
        RawType::Enum(v) => v,
        RawType::LoweredStruct(_, container) => {
            if !container.is_lowered_enum_parent {
                return Err(make_req_type_kind(origin, &"enum parent".to_string()).into());
            }

            container.lowered_enum_parent.unwrap()
        }

        _ => return Err(make_req_type_kind(origin, &"enum parent".to_string()).into()),
    };

    let enum_entry_container = match &enum_entry {
        RawType::EnumEntry(v) => v,
        RawType::LoweredStruct(_, container) => {
            if !container.is_lowered_enum_child {
                return Err(make_req_type_kind(origin, &"enum parent".to_string()).into());
            }

            container.lowered_enum_child.as_ref().unwrap()
        }
        _ => return Err(make_req_type_kind(origin, &"enum child".to_string()).into()),
    };

    if enum_entry_container.parent != enum_type.self_ref {
        return Err(
            make_type_not_partof(origin, &enum_entry_container.child, &enum_type.self_ref).into(),
        );
    }

    return build_ir_cast(&mut ctx.mir_ctx, val, Type::GenericLowered(enum_entry));
}

pub fn lower_hir_unwrap_cond(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<BaseMIRValue> {
    if let HIRNodeKind::UnwrapCondition {
        original,
        new_type,
        new_var,
        unsafe_unwrap,
    } = node.kind.clone()
    {
        let original = lower_hir_variable_reference(block, &original, ctx)?
            .as_pointer_ref()?
            .into();
        let new_type = lower_hir_type(ctx, new_type)?;

        let cond;

        if unsafe_unwrap {
            cond = build_unsigned_int_const(&mut ctx.mir_ctx, 1, 1)?;
        } else {
            cond = is_enum_value_of_kind(
                block,
                original,
                new_type.get_generic(&ctx.hir_ctx.type_storage),
                ctx,
                &*node,
            )?
        }

        if new_var.is_none() {
            return Ok(cond.into());
        }

        ctx.block_introduction_var_queue.push(node.clone());

        return Ok(cond.into());
    }

    panic!("Invalid node!")
}

pub fn lower_hir_unwrap_value(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<BaseMIRValue> {
    if let HIRNodeKind::UnwrapValue {
        original,
        new_type,
        unsafe_unwrap: _,
    } = node.kind.clone()
    {
        let original = lower_hir_value(block, original, ctx)?;
        let new_type = lower_hir_type(ctx, new_type)?;

        return cast_to_enum_child(
            block,
            original,
            new_type.get_generic(&ctx.hir_ctx.type_storage),
            ctx,
            &*node,
        );
    }

    panic!("Invalid node!")
}
