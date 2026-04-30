//! The prelude is the environment in where the defaults of the language are loaded

use astoir_hir::scope::ScopeStorage;
use diagnostics::{DiagnosticSpanOrigin, MaybeDiagnostic};

use crate::types::apply_prelude_types;

pub mod types;

pub fn apply_prelude<K: DiagnosticSpanOrigin>(
    scope: &mut ScopeStorage,
    origin: &K,
) -> MaybeDiagnostic {
    apply_prelude_types(scope, origin)
}
