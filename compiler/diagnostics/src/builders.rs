use std::fmt::Display;

use crate::{DiagnosticSpanOrigin, diagnostic::{Diagnostic, Level, Span, SpanKind, SpanPosition}, errors::{ALREADY_IN_SCOPE, DIFF_SIZE_SPECIFIERS, EXPECTED_FREE, EXPECTED_TOKEN, EXPECTED_TYPE, FIND_TYPE, UNEXPECTED_TOKEN}, warnings::UNUSED_VAR};

pub fn make_expected_simple_error<K: DiagnosticSpanOrigin, E: Display, G: Display>(origin: &K, expected: &E, got: &G) -> Diagnostic {
	origin.make_simple_diagnostic(EXPECTED_TOKEN.0, Level::Error, format!("expected {} but got {}", expected, got), None, vec![], vec![])
}

pub fn make_unexpected_simple_error<K: DiagnosticSpanOrigin, E: Display>(origin: &K, got: &E) -> Diagnostic {
	origin.make_simple_diagnostic(UNEXPECTED_TOKEN.0, Level::Error, format!("unexpected {}", got), None, vec![], vec![])
}

pub fn make_unexpected_simple_error_outside<K: Display>(got: &K, pos: SpanPosition) -> Diagnostic {
	let span = Span::make_primary(pos, None);

	Diagnostic::new_base(Level::Error, EXPECTED_TOKEN.0, format!("unexpected {}", got), span, vec![], vec![], vec![])
}

pub fn make_expected_single_simple_error<K: DiagnosticSpanOrigin, E: Display>(origin: &K, got: &E) -> Diagnostic {
	origin.make_simple_diagnostic(EXPECTED_FREE.0, Level::Error, format!("expected {}", got), None, vec![], vec![])
}

pub fn make_unused_variable<K: DiagnosticSpanOrigin, E: Display>(origin: &K, var: &E) -> Diagnostic {
	origin.make_simple_diagnostic(UNUSED_VAR.0, Level::Warning, format!("unused variable {}", var), None, vec![], vec![])
}

pub fn make_already_in_scope<K: DiagnosticSpanOrigin, E: Display>(origin: &K, val: &E) -> Diagnostic {
	origin.make_simple_diagnostic(ALREADY_IN_SCOPE.0, Level::Error, format!("{} was already found in this scope", val), None, vec![], vec![])
}

pub fn make_cannot_find_type<K: DiagnosticSpanOrigin, E: Display>(origin: &K, t: &E) -> Diagnostic {
	origin.make_simple_diagnostic(FIND_TYPE.0, Level::Error, format!("cannot find type {} in the current scope", t), None, vec![], vec![])
}

pub fn make_diff_size_specifiers<K: DiagnosticSpanOrigin, A: Display, B: Display>(origin: &K, a: &A, b: &B) -> Diagnostic {
	origin.make_simple_diagnostic(DIFF_SIZE_SPECIFIERS.0, Level::Error, format!("expected {} size specifiers on this type, got {}", a, b), None, vec![], vec![])
}

pub fn make_diff_type<K: DiagnosticSpanOrigin, V: Display, T: Display, A: Display>(origin: &K, var_name: &V, req_type: &T, got: &A, var_origin: &K) -> Diagnostic {
	origin.make_simple_diagnostic(EXPECTED_TYPE.0, Level::Error, format!("expected {} but got {}", req_type, got), Some(format!("variable {} was declared with type {} here", var_name, req_type)), vec![], vec![])
}