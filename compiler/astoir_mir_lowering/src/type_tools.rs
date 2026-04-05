use astoir_hir::nodes::HIRNode;
use astoir_mir::{blocks::refer::MIRBlockReference, builder::{build_comp_eq, build_field_pointer, build_load, build_unsigned_int_const}, vals::{base::BaseMIRValue, int::MIRIntValue}};
use compiler_typing::{SizedType, raw::RawType, tree::Type};
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::make_req_type_kind};

use crate::MIRLoweringContext;

pub fn is_enum_value_of_kind<K: DiagnosticSpanOrigin>(block: MIRBlockReference, val: BaseMIRValue, enum_entry: RawType, ctx: &mut MIRLoweringContext, origin: &K) -> DiagnosticResult<MIRIntValue> {
	let enum_type = match ctx.mir_ctx.ssa_hints.get_hint(val.get_ssa_index()).get_type().as_generic_lowered() {
		RawType::Enum(v) => v,
		_ => return Err(make_req_type_kind(origin, &"enum parent".to_string()).into())
	};

	let enum_entry = match ctx.mir_ctx.ssa_hints.get_hint(val.get_ssa_index()).get_type().as_generic_lowered() {
		RawType::EnumEntry(v) => v,
		_ => return Err(make_req_type_kind(origin, &"enum child".to_string()).into())
	};

	let hint_type = enum_type.get_hint_type();

	let field_ptr = build_field_pointer(&mut ctx.mir_ctx, val.as_ptr()?, 0)?; // 0 = hint type index
	let hint_val = build_load(&mut ctx.mir_ctx, field_ptr)?.as_int()?;

	let hint_true = build_unsigned_int_const(&mut ctx.mir_ctx, enum_entry.child as u128, hint_type.get_size(&Type::GenericLowered(hint_type.clone()), false, &ctx.hir_ctx.type_storage))?;

	return build_comp_eq(&mut ctx.mir_ctx, hint_val, hint_true);
}