use astoir_hir::{
    scope::{ScopeStorage, entry::ScopeEntry, key::EntryKey},
    types::ScopeStoredType,
};

use compiler_utils::hash::HashedString;
use diagnostics::{DiagnosticSpanOrigin, MaybeDiagnostic};
use typing::raw::RawType;

fn register_type<K: DiagnosticSpanOrigin>(
    hir: &mut ScopeStorage,
    name: &str,
    val: RawType,
    origin: &K,
) -> MaybeDiagnostic {
    let key = EntryKey::new(HashedString::new(name.to_string()));
    let entry = ScopeEntry::new_type(ScopeStoredType {
        t: val,
        function_implementations: vec![],
    });

    hir.append(key, entry, origin)?;
    Ok(())
}

pub fn apply_prelude_types<K: DiagnosticSpanOrigin>(
    hir: &mut ScopeStorage,
    origin: &K,
) -> MaybeDiagnostic {
    register_type(hir, "u", RawType::UnsizedInteger(true), origin)?;
    register_type(hir, "s8", RawType::Integer(true, 8), origin)?;
    register_type(hir, "s16", RawType::Integer(true, 16), origin)?;
    register_type(hir, "s32", RawType::Integer(true, 32), origin)?;
    register_type(hir, "s64", RawType::Integer(true, 64), origin)?;
    register_type(hir, "s128", RawType::Integer(true, 128), origin)?;

    register_type(hir, "u", RawType::UnsizedInteger(false), origin)?;
    register_type(hir, "u8", RawType::Integer(false, 8), origin)?;
    register_type(hir, "u16", RawType::Integer(false, 16), origin)?;
    register_type(hir, "u32", RawType::Integer(false, 32), origin)?;
    register_type(hir, "u64", RawType::Integer(false, 64), origin)?;
    register_type(hir, "u128", RawType::Integer(false, 128), origin)?;

    register_type(hir, "f", RawType::UnsizedFloating(true), origin)?;
    register_type(hir, "f8", RawType::Floating(true, 8), origin)?;
    register_type(hir, "f16", RawType::Floating(true, 16), origin)?;
    register_type(hir, "f32", RawType::Floating(true, 32), origin)?;
    register_type(hir, "f64", RawType::Floating(true, 64), origin)?;
    register_type(hir, "f128", RawType::Floating(true, 128), origin)?;

    register_type(hir, "uf", RawType::UnsizedFloating(false), origin)?;
    register_type(hir, "uf8", RawType::Floating(false, 8), origin)?;
    register_type(hir, "uf16", RawType::Floating(false, 16), origin)?;
    register_type(hir, "uf32", RawType::Floating(false, 32), origin)?;
    register_type(hir, "uf64", RawType::Floating(false, 64), origin)?;
    register_type(hir, "uf128", RawType::Floating(false, 128), origin)?;

    register_type(hir, "x", RawType::UnsizedFixedPoint(true), origin)?;
    register_type(hir, "x8", RawType::FixedPoint(true, 4, 4), origin)?;
    register_type(hir, "x16", RawType::FixedPoint(true, 8, 8), origin)?;
    register_type(hir, "x32", RawType::FixedPoint(true, 16, 16), origin)?;
    register_type(hir, "x64", RawType::FixedPoint(true, 32, 32), origin)?;
    register_type(hir, "x128", RawType::FixedPoint(true, 64, 64), origin)?;

    register_type(hir, "ux", RawType::UnsizedFixedPoint(false), origin)?;
    register_type(hir, "ux8", RawType::FixedPoint(false, 4, 4), origin)?;
    register_type(hir, "ux16", RawType::FixedPoint(false, 8, 8), origin)?;
    register_type(hir, "ux32", RawType::FixedPoint(false, 16, 16), origin)?;
    register_type(hir, "ux64", RawType::FixedPoint(false, 32, 32), origin)?;
    register_type(hir, "ux128", RawType::FixedPoint(false, 64, 64), origin)?;

    register_type(hir, "bool", RawType::Boolean, origin)?;
    register_type(hir, "staticstr", RawType::StaticString, origin)?;
    register_type(hir, "ptr", RawType::AnyPointer, origin)?;

    Ok(())
}
