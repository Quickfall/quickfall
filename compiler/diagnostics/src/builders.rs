use std::fmt::Display;

use crate::{DiagnosticSpanOrigin, diagnostic::{Diagnostic, Level, Span, SpanPosition}, errors::{EXPECTED_TOKEN, UNEXPECTED_TOKEN}};

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