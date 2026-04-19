//! The prelude is the environment in where the defaults of the language are loaded

use astoir_hir::ctx::HIRContext;
use diagnostics::{DiagnosticSpanOrigin, MaybeDiagnostic};

use crate::types::apply_prelude_types;

pub mod types;

pub fn apply_prelude<K: DiagnosticSpanOrigin>(hir: &mut HIRContext, origin: &K) -> MaybeDiagnostic {
	apply_prelude_types(hir, origin)
}