use astoir_hir::{ctx::HIRContext, storage::{EntryKey, GlobalStorageEntryType}};
use compiler_typing::{TypeParameterContainer, enums::RawEnumTypeContainer, raw::RawType, references::TypeReference};
use compiler_utils::hash;
use compiler_utils::hash::{HashedString};
use diagnostics::{DiagnosticSpanOrigin, MaybeDiagnostic};

pub const SIGNED_INTEGER_8: u64 = hash!("s8");
pub const SIGNED_INTEGER_16: u64 = hash!("s16");
pub const SIGNED_INTEGER_32: u64 = hash!("s32");
pub const SIGNED_INTEGER_64: u64 = hash!("s64");
pub const SIGNED_INTEGER_128: u64 = hash!("s128");
pub const SIGNED_INTEGER: u64 = hash!("s");

pub const UNSIGNED_INTEGER_8: u64 = hash!("u8");
pub const UNSIGNED_INTEGER_16: u64 = hash!("u16");
pub const UNSIGNED_INTEGER_32: u64 = hash!("u32");
pub const UNSIGNED_INTEGER_64: u64 = hash!("u64");
pub const UNSIGNED_INTEGER_128: u64 = hash!("u128");
pub const UNSIGNED_INTEGER: u64 = hash!("u");

pub const SIGNED_FLOATING_POINT_8: u64 = hash!("f8");
pub const SIGNED_FLOATING_POINT_16: u64 = hash!("f16");
pub const SIGNED_FLOATING_POINT_32: u64 = hash!("f32");
pub const SIGNED_FLOATING_POINT_64: u64 = hash!("f64");
pub const SIGNED_FLOATING_POINT_80: u64 = hash!("f80");
pub const SIGNED_FLOATING_POINT_128: u64 = hash!("f128");
pub const SIGNED_FLOATING_POINT: u64 = hash!("f");

pub const SIGNED_FIXED_POINT_8: u64 = hash!("x8");
pub const SIGNED_FIXED_POINT_16: u64 = hash!("x16");
pub const SIGNED_FIXED_POINT_32: u64 = hash!("x32");
pub const SIGNED_FIXED_POINT_64: u64 = hash!("x64");
pub const SIGNED_FIXED_POINT_128: u64 = hash!("x128");
pub const SIGNED_FIXED_POINT: u64 = hash!("x");

pub const UNSIGNED_FLOATING_POINT_8: u64 = hash!("uf8");
pub const UNSIGNED_FLOATING_POINT_16: u64 = hash!("uf16");
pub const UNSIGNED_FLOATING_POINT_32: u64 = hash!("uf32");
pub const UNSIGNED_FLOATING_POINT_64: u64 = hash!("uf64");
pub const UNSIGNED_FLOATING_POINT_80: u64 = hash!("uf80");
pub const UNSIGNED_FLOATING_POINT_128: u64 = hash!("uf128");
pub const UNSIGNED_FLOATING_POINT: u64 = hash!("uf");

pub const UNSIGNED_FIXED_POINT_8: u64 = hash!("ux8");
pub const UNSIGNED_FIXED_POINT_16: u64 = hash!("ux16");
pub const UNSIGNED_FIXED_POINT_32: u64 = hash!("ux32");
pub const UNSIGNED_FIXED_POINT_64: u64 = hash!("ux64");
pub const UNSIGNED_FIXED_POINT_128: u64 = hash!("ux128");
pub const UNSIGNED_FIXED_POINT: u64 = hash!("ux");

pub const STATIC_STR: u64 = hash!("staticstr");

pub const POINTER_TYPE: u64 = hash!("ptr");
pub const BOOLEAN_TYPE: u64 = hash!("bool");

/// Experimental
pub const RESULT_TYPE: u64 = hash!("result");

pub fn register_prelude_type<K: DiagnosticSpanOrigin>(hir: &mut HIRContext, hash: u64, t: RawType, origin: &K) -> MaybeDiagnostic {
	hir.global_scope.append(EntryKey { name_hash: hash }, GlobalStorageEntryType::Type(t), origin)
}

pub fn apply_prelude_types<K: DiagnosticSpanOrigin>(hir: &mut HIRContext, origin: &K) -> MaybeDiagnostic {
	register_prelude_type(hir, SIGNED_INTEGER, RawType::SizedInteger(true), origin)?;
	register_prelude_type(hir, SIGNED_INTEGER_8, RawType::Integer(8, true), origin)?;
	register_prelude_type(hir, SIGNED_INTEGER_16, RawType::Integer(16, true), origin)?;
	register_prelude_type(hir, SIGNED_INTEGER_32, RawType::Integer(32, true), origin)?;
	register_prelude_type(hir, SIGNED_INTEGER_64, RawType::Integer(64, true), origin)?;
	register_prelude_type(hir, SIGNED_INTEGER_128, RawType::Integer(128, true), origin)?;

	register_prelude_type(hir, UNSIGNED_INTEGER, RawType::SizedInteger(false), origin)?;
	register_prelude_type(hir, UNSIGNED_INTEGER_8, RawType::Integer(8, false), origin)?;
	register_prelude_type(hir, UNSIGNED_INTEGER_16, RawType::Integer(16, false), origin)?;
	register_prelude_type(hir, UNSIGNED_INTEGER_32, RawType::Integer(32, false), origin)?;
	register_prelude_type(hir, UNSIGNED_INTEGER_64, RawType::Integer(64, false), origin)?;
	register_prelude_type(hir, UNSIGNED_INTEGER_128, RawType::Integer(128, false), origin)?;

	register_prelude_type(hir, SIGNED_FLOATING_POINT, RawType::SizedFloating(true), origin)?;
	register_prelude_type(hir, SIGNED_FLOATING_POINT_8, RawType::Floating(8, true), origin)?;
	register_prelude_type(hir, SIGNED_FLOATING_POINT_16, RawType::Floating(16, true), origin)?;
	register_prelude_type(hir, SIGNED_FLOATING_POINT_32, RawType::Floating(32, true), origin)?;
	register_prelude_type(hir, SIGNED_FLOATING_POINT_64, RawType::Floating(64, true), origin)?;
	register_prelude_type(hir, SIGNED_FLOATING_POINT_128, RawType::Floating(128, true), origin)?;

	register_prelude_type(hir, UNSIGNED_FLOATING_POINT, RawType::SizedFloating(false), origin)?;
	register_prelude_type(hir, UNSIGNED_FLOATING_POINT_8, RawType::Floating(8, false), origin)?;
	register_prelude_type(hir, UNSIGNED_FLOATING_POINT_16, RawType::Floating(16, false), origin)?;
	register_prelude_type(hir, UNSIGNED_FLOATING_POINT_32, RawType::Floating(32, false), origin)?;
	register_prelude_type(hir, UNSIGNED_FLOATING_POINT_64, RawType::Floating(64, false), origin)?;
	register_prelude_type(hir, UNSIGNED_FLOATING_POINT_128, RawType::Floating(128, false), origin)?;


	register_prelude_type(hir, SIGNED_FIXED_POINT, RawType::SizedFixedPoint(true), origin)?;
	register_prelude_type(hir, SIGNED_FIXED_POINT_8, RawType::FixedPoint(4, 4, true), origin)?;
	register_prelude_type(hir, SIGNED_FIXED_POINT_16, RawType::FixedPoint(8, 8, true), origin)?;
	register_prelude_type(hir, SIGNED_FIXED_POINT_32, RawType::FixedPoint(16, 16, true), origin)?;
	register_prelude_type(hir, SIGNED_FIXED_POINT_64, RawType::FixedPoint(64, 64, true), origin)?;
	register_prelude_type(hir, SIGNED_FIXED_POINT_128, RawType::FixedPoint(128, 128, true), origin)?;

	register_prelude_type(hir, UNSIGNED_FIXED_POINT, RawType::SizedFixedPoint(true), origin)?;
	register_prelude_type(hir, UNSIGNED_FIXED_POINT_8, RawType::FixedPoint(4, 4, true), origin)?;
	register_prelude_type(hir, UNSIGNED_FIXED_POINT_16, RawType::FixedPoint(8, 8, true), origin)?;
	register_prelude_type(hir, UNSIGNED_FIXED_POINT_32, RawType::FixedPoint(16, 16, true), origin)?;
	register_prelude_type(hir, UNSIGNED_FIXED_POINT_64, RawType::FixedPoint(64, 64, true), origin)?;
	register_prelude_type(hir, UNSIGNED_FIXED_POINT_128, RawType::FixedPoint(128, 128, true), origin)?;

	register_prelude_type(hir, BOOLEAN_TYPE, RawType::Boolean, origin)?;
	register_prelude_type(hir, STATIC_STR, RawType::StaticString, origin)?;
	register_prelude_type(hir, POINTER_TYPE, RawType::Pointer, origin)?;

	// result<V, E>
	{
		let mut type_params = TypeParameterContainer::new();

		type_params.insert(HashedString::new("V".to_string()), 0);
		type_params.insert(HashedString::new("E".to_string()), 1);

		let mut result_enum = RawEnumTypeContainer::new(hir.global_scope.entries.len(), type_params);

		result_enum.append_entry(HashedString::new("value".to_string()), vec![(hash!("val"), TypeReference::make_unresolved(0))]);
		result_enum.append_entry(HashedString::new("error".to_string()), vec![(hash!("err"), TypeReference::make_unresolved(1))]);

		register_prelude_type(hir, RESULT_TYPE, RawType::Enum(result_enum), origin)?;
	}

	Ok(())
}