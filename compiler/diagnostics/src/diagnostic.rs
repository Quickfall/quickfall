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
		Diagnostic { level, code: decl.0, message: decl.1.to_string(), primary_span, spans, note, help}
 	}
}

pub struct Span {
	pub start: Position,
	pub end_col: usize,

	pub label: String,
	pub kind: SpanKind
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

		let underline = print_underline(self.start.col, self.end_col, self.kind.get_marker_char());

		writeln!(f, "   {}    {}", "|".bright_blue(), underline.bright_yellow())?;

		let space = print_space(self.start.col + 4);
		writeln!(f, "   {}{}{}", "|".bright_blue(), space, self.label)?;

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