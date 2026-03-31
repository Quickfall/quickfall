//! The core of diagnostics

pub enum Level {
	Error,
	Warning,
	Note
}

pub struct Diagnostic {
	pub level: Level,
	pub code: usize,
	pub message: String,

	pub note: Vec<String>,
	pub help: Vec<String>,
}