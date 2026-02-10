//!
//! Error handling code for most of the Quickfall compiler!
//! 

use core::fmt;

use colored::Colorize;

use crate::Position;

pub type PositionedResult<K> = Result<K, PositionedError>;

/// An error that has a position
#[derive(Debug)]
pub struct PositionedError {
	pub start: Position,
	pub end: Position,
	pub reason: String
}

impl PositionedError {
	pub fn new(start: Position, end: Position, reason: String) -> Self {

		let err = PositionedError { start, end, reason };

		println!("{}", err);

		return err;
	}
}

impl fmt::Display for PositionedError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{} at {}", "ERR".bright_red().bold(), self.start)?;
		
		let line = match self.start.get_line_content() {
			Ok(v) => v,
			Err(e) => format!("{}: {}","Couldn't read file contents!".red().bold(), e)
		};

		let before = &line[0..self.start.col - 1];
		let target = &line[self.start.col - 1..self.end.col].cyan().underline();
		let after = &line[self.end.col..];

		writeln!(f, "{}{}{}", before, target, after)?;
		writeln!(f, "")?;
		writeln!(f, "{}", self.reason.bright_red())?;

		Ok(())
	}
}