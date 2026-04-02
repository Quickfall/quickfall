//! The core of diagnostics

use std::{fmt::Display, fs, io::Error};

use colored::{ColoredString, Colorize};
use compiler_utils::Position;

use crate::DIAGNOSTIC_CONTAINER;

#[derive(Clone)]
pub enum Level {
	Error,
	Warning,
	Note
}

#[derive(Clone)]
pub enum SpanKind {
	Primary,
	Secondary
}

impl SpanKind {
	pub fn get_marker_char(&self) -> char {
		match self {
			Self::Primary => '^',
			Self::Secondary => '-'
		}
	}
}

impl Level {
	pub fn apply_color(&self, str: ColoredString) -> ColoredString {
		match self {
			Self::Error => str.bright_red().bold(),
			Self::Warning => str.yellow().bold(),
			Self::Note => str.blue().bold()
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

#[derive(Clone)]
pub struct Diagnostic {
	pub level: Level,
	pub code: usize,
	pub message: String,

	pub primary_span: Span,
	pub spans: Vec<Span>,

	pub note: Vec<String>,
	pub help: Vec<String>
}

impl Diagnostic {
	pub fn new(level: Level, decl: (usize, &str), primary_span: Span, spans: Vec<Span>, note: Vec<String>, help: Vec<String>) -> Self {
		let d = Diagnostic { level, code: decl.0, message: decl.1.to_string(), primary_span, spans, note, help};
		
		d.push_to_storage();

		d
 	}

	pub fn new_base(level: Level, code: usize, message: String, primary_span: Span, spans: Vec<Span>, note: Vec<String>, help: Vec<String>) -> Self {
		let d = Diagnostic { level, code, message, primary_span, spans, note, help};

		d.push_to_storage();

		d
	}

	fn push_to_storage(&self) {		
		DIAGNOSTIC_CONTAINER.with_borrow_mut(|f| {
			f.append(self.clone());
		})
	}
}

impl Into<()> for Diagnostic {
	fn into(self) -> () {
		
	}
}

#[derive(Clone)]
pub struct SpanPosition {
	pub line: usize,
	pub col: usize,
	pub file_path: String,
	pub end_col: usize
}

impl SpanPosition {
	pub fn from_pos(pos: Position, end_col: usize) -> Self {
		SpanPosition { line: pos.line, col: pos.col, file_path: pos.file_path, end_col }
	}

	pub fn from_pos2(start: Position, end: Position) -> Self {
		SpanPosition { line: start.line, col: start.col, file_path: start.file_path, end_col: end.col }
	}

	pub fn get_line_content(&self) -> Result<String, Error> {
		let contents = fs::read_to_string(&self.file_path)?;

		let spl: Vec<&str> = contents.split('\n').collect();

		return Ok(String::from(spl[self.line - 1]));
	}
}

impl Display for SpanPosition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = write!(f, "{}:{}:{}", self.file_path, self.line, self.col);

		Ok(())
	}
}

#[derive(Clone)]
pub struct Span {
	pub start: SpanPosition,

	pub label: Option<String>,
	pub kind: SpanKind
}

impl Span {
	pub fn make_primary(pos: SpanPosition, label: Option<String>) -> Self {
		Span { start: pos, label, kind: SpanKind::Primary }
	}

	pub fn make_secondary(pos: SpanPosition, label: Option<String>) -> Self {
		Span { start: pos, label, kind: SpanKind::Secondary }
	}
}

fn print_underline(start: usize, end: usize, c: char) -> String {
	let mut str = "".to_string();
	for _ in 0..start {
		str += " ";
	}

	for _ in start..end {
		str += &c.to_string();
	}

	str
}

fn print_space(start: usize) -> String {
	let mut str = "".to_string();

	for _ in 0..start {
		str += " ";
	}

	str
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let line = match self.start.get_line_content() {
			Ok(v) => v,
			Err(_) => "".to_string()
		};

		writeln!(f, "   {}    {}", "|".bright_blue() , line)?;

		let underline = print_underline(self.start.col, self.start.end_col, self.kind.get_marker_char());

		writeln!(f, "   {}    {}", "|".bright_blue(), underline.bright_yellow())?;

		if let Some(v) = self.label.clone() {
			let space = print_space(self.start.col + 4);
			writeln!(f, "   {}{}{}", "|".bright_blue(), space, v)?;
		}

		Ok(())
	}
}

impl Display for Diagnostic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let header = ColoredString::from(format!("{}[{}{}]", self.level, self.level.get_code_char(), self.code));
		let header = self.level.apply_color(header);

		writeln!(f, "{}: {}", header, self.message)?;
		writeln!(f, "  {} {}", "-->".bright_blue(), self.primary_span.start)?;
		writeln!(f, "   {}", "|".bright_blue())?;

		write!(f, "{}", self.primary_span)?;

		for span in &self.spans {
			writeln!(f, "{}", span)?;
		}

		writeln!(f, "   {}", "|".bright_blue())?;
		
		let mut ind = 0;
		for note in &self.note {
			writeln!(f, "   {} {}: {}", "=".bright_blue(), "note".bold(), note)?;
			writeln!(f, "   {} {}: {}", "=".bright_blue(), "help".bold(), self.help[ind])?;
			write!(f, "   {}", "|".bright_blue())?;

			ind += 1;
		}
		
		Ok(())
	}
}