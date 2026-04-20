use astoir_hir::nodes::{HIRNode, HIRNodeKind};
use astoir_mir::{
    blocks::refer::MIRBlockReference,
    builder::{build_static_struct_const, build_unsigned_int_const},
    vals::structs::MIRStructValue,
};
use compiler_typing::{SizedType, raw::RawType, tree::Type};
use diagnostics::DiagnosticResult;

use crate::{MIRLoweringContext, lower_hir_type, values::lower_hir_value};

pub fn lower_hir_struct_init(
    block: MIRBlockReference,
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<MIRStructValue> {
    if let HIRNodeKind::StructInitializerTyped { t, fields } = node.kind {
        let mut values = vec![];

        match t.get_generic(&ctx.hir_ctx.type_storage) {
            RawType::Struct(_, _) => {
                for field in fields {
                    values.push(lower_hir_value(block, field, ctx)?);
                }
            }

            RawType::EnumEntry(container) => {
                let parent = match &ctx.hir_ctx.type_storage.types.vals[container.parent] {
                    RawType::Enum(container) => container.clone(),
                    _ => panic!("Enum parent not enum"),
                };

                let hint = build_unsigned_int_const(
                    &mut ctx.mir_ctx,
                    container.child as u128,
                    parent.get_hint_type().get_size(
                        &Type::GenericLowered(parent.get_hint_type()),
                        false,
                        &ctx.hir_ctx.type_storage,
                    ),
                )?;

                values.push(hint.into());

                for field in fields {
                    values.push(lower_hir_value(block, field, ctx)?);
                }
            }

            _ => panic!("Invalid type for a StructInitializedTyped"),
        }

        let lowered_type = lower_hir_type(ctx, t)?.get_generic(&ctx.hir_ctx.type_storage);

        return build_static_struct_const(&mut ctx.mir_ctx, lowered_type, values);
    }

    panic!("Invalid node")
}
