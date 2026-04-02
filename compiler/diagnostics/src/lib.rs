use std::{cell::RefCell};

use crate::diagnostic::{Diagnostic, Level, Span, SpanKind, SpanPosition};

pub mod diagnostic;
pub mod errors;
pub mod warnings;
pub mod infos;
pub mod builders;

pub type DiagnosticResult<K> = Result<K, ()>;
pub type MaybeDiagnostic = DiagnosticResult<()>;

thread_local! {
	static DIAGNOSTIC_CONTAINER: RefCell<DiagnosticContainer> = RefCell::new(DiagnosticContainer::new());
}

thread_local! {
	/// Used whenever the position cannot be passed directly (eg: interacting with the typing system or MIR.)
	static CURR_DIAGNOSTIC_POS: RefCell<Option<SpanPosition>> = RefCell::new(None)
}

/// Used whenever a panic might be unsafe to trigger. Every panic triggered by this will be also logged in the error system
#[macro_export]
macro_rules! unsure_panic {
	($msg: expr) => { {
		diagnostics::builders::make_unsure_panic(&$msg.to_string());
		diagnostics::dump_diagnostics();
		panic!($msg);
	}
	};
}

/// Origin that can generate diagnostics easily
pub trait DiagnosticSpanOrigin {
	fn make_span(&self, kind: SpanKind, msg: Option<String>) -> Span;
	fn make_simple_diagnostic(&self, code: usize, level: Level, message: String, primary_span_msg: Option<String>, spans: Vec<Span>, notes: Vec<String>, help: Vec<String>) -> Diagnostic;
}

pub struct DiagnosticContainer {
	pub diagnostics: Vec<Diagnostic>
}

impl DiagnosticContainer {
	pub fn new() -> Self {
		DiagnosticContainer { diagnostics: vec![] }
	}

	pub fn append(&mut self, diagnostic: Diagnostic) {
		self.diagnostics.push(diagnostic);
	}
}

pub fn dump_diagnostics() {
	DIAGNOSTIC_CONTAINER.with_borrow(|f| {
		for diagnostic in &f.diagnostics {
			println!("{}", diagnostic);
		}
	})
}

pub fn move_current_diagnostic_pos(pos: SpanPosition) {
	CURR_DIAGNOSTIC_POS.with_borrow_mut(|f| {
		*f = Some(pos);
	})
}

pub fn get_current_diagnostic_pos() -> SpanPosition {
	CURR_DIAGNOSTIC_POS.with_borrow(|f| {
		return f.clone().unwrap();
	})
}