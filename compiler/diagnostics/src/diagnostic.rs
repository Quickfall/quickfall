//! The core of diagnostics

use std::fmt::Display;

use colored::{ColoredString, Colorize};
use compiler_utils::Position;

pub enum Level {
	Error,
	Warning,
	Note
}

pub enum SpanKind {
	Primary,
	Secondary
}

impl SpanKind {
	pub fn get_marker_char(&self) -> char {
		match self {
			Self::Primary => '-',
			Self::Secondary => '^'
		}
	}
}

impl Level {
	pub fn apply_color(&self, str: ColoredString) -> ColoredString {
		match self {
			Self::Error => str.red(),
			Self::Warning => str.yellow(),
			Self::Note => str.blue()
		}
	}

	pub fn get_code_char(&self) -> char {
		match self {
			Self::Error => 'E',
			Self::Warning => 'W',
			Self::Note => 'I'
		}
	}
}

impl Display for Level {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Error => write!(f, "error"),
			Self::Warning => write!(f, "warning"),
			Self::Note => writeln!(f, "info")
		}
	}
}

pub struct Diagnostic {
	pub level: Level,
	pub code: usize,
	pub message: String,

	pub primary_span: Span,
	pub spans: Vec<Span>,

	pub note: Vec<String>,
	pub help: Vec<String>
}

pub struct Span {
	pub start: Position,
	pub end_col: usize,

	pub label: String,
	pub kind: SpanKind
}

fn print_underline(f: &mut std::fmt::Formatter<'_>, start: usize, end: usize, c: char) -> std::fmt::Result {
	for _ in 0..start {
		write!(f, " ")?;
	}

	for _ in start..end {
		write!(f, "{}", c)?;
	}

	Ok(())
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let line = match self.start.get_line_content() {
			Ok(v) => v,
			Err(_) => "".to_string()
		};

		writeln!(f, "{}", line)?;

		print_underline(f, self.start.col, self.end_col, self.kind.get_marker_char())?;
		writeln!(f, " {}\n", self.label)?;

		Ok(())
	}
}

impl Display for Diagnostic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let header = ColoredString::from(format!("{}[{}{}], ", self.level, self.level.get_code_char(), self.code));
		let header = self.level.apply_color(header);

		writeln!(f, "{}: {}", header, self.message)?;
		writeln!(f, "  --> {}", self.primary_span.start)?;
		writeln!(f, "   |\n")?;

		writeln!(f, "{}", self.primary_span)?;

		for span in &self.spans {
			writeln!(f, "{}", span)?;
		}
		
		Ok(())
	}
}