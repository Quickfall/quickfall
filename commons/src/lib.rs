use core::fmt;
use std::{fs, io::Error};

pub mod err;

#[derive(Debug)]
pub struct Position {
	pub line: usize,
	pub col: usize,
	pub file_path: String
}

impl Position {
	fn get_line_content(&self) -> Result<String, Error> {
		let contents = fs::read_to_string(&self.file_path)?;

		let spl: Vec<&str> = contents.split('\n').collect();

		return Ok(String::from(spl[self.line - 1]));
	}
}

impl Position {
	pub fn new(path: String, line: usize, col: usize) -> Self {
		return Position { line, col, file_path: path };
	}

	pub fn increment_by(&self, count: usize) -> Self {
		return Position::new(self.file_path.clone(), self.line, self.col + count);
	}
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{} in {}", self.line, self.col, self.file_path);

		Ok(())
	}
}