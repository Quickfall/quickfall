//!
//! Error handling code for most of the Quickfall compiler!
//! 

use core::fmt;

use colored::Colorize;

use crate::Position;

/// An error that has a position
pub struct PositionedError {
	pub start: Position,
	pub end: Position,
	pub reason: String
}

impl fmt::Display for PositionedError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{} at {}", "ERR".bright_red().bold(), self.start);
		
		let line = match self.start.get_line_content() {
			Ok(v) => v,
			Err(e) => format!("{}","Couldn't read file contents!".red().bold())
		};

		let before = &line[self.start.col - 1..];
		let target = &line[self.start.col..self.end.col - 1].cyan().underline();
		let after = &line[self.end.col..];

		writeln!(f, "{}{}{}", before, target, after);
		writeln!(f, "");
		writeln!(f, "{}", self.reason.bright_red());

		Ok(())
	}
}