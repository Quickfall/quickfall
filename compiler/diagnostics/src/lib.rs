use std::cell::RefCell;

use crate::diagnostic::Diagnostic;

pub mod diagnostic;
pub mod errors;
pub mod warnings;
pub mod infos;

pub type DiagnosticResult<K> = Result<K, Diagnostic>;

thread_local! {
	static DIAGNOSTIC_CONTAINER: RefCell<DiagnosticContainer> = RefCell::new(DiagnosticContainer::new())
}

pub struct DiagnosticPosition {
	pub path: String,
	pub line: usize,
	pub start_col: usize,
	pub end_col: usize
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
