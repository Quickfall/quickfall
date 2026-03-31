//! The core of diagnostics

use colored::{ColoredString, Colorize};
use compiler_utils::Position;

pub enum Level {
	Error,
	Warning,
	Note
}

impl Level {
	pub fn apply_color(&self, str: ColoredString) -> ColoredString {
		match self {
			Self::Error => str.red(),
			Self::Warning => str.yellow(),
			Self::Note => str.blue()
		}
	}
}

pub struct Diagnostic {
	pub level: Level,
	pub code: usize,
	pub message: String,

	pub primary_span: Span,
	pub secondary_spans: Vec<Span>,

	pub note: Vec<String>,
	pub help: Vec<String>
}

pub struct Span {
	pub start: Position,
	pub end: Position,

	pub label: String
}