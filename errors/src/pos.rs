use std::{fs, io};

#[derive(Clone)]
pub struct ErrorPosition {
	pub line: usize,

	pub col: usize,
	pub size: usize,

	pub file_path: String
}

impl ErrorPosition {
	pub fn new(path: String, line: usize, col: usize, end: usize) -> Self {
		return ErrorPosition { line, col, size: end - col, file_path: path }
	}

	pub fn get_line_str(&self) -> Result<String, io::Error> {
		let contents = fs::read_to_string(&self.file_path)?;

		let split: Vec<&str> = contents.split("\n").collect();

		return Ok(split[self.line].to_string())
	}

	pub fn get_line_slice(&self) -> Result<String, io::Error> {
		return Ok(self.get_line_str()?[self.col - 1..self.col + self.size - 1].to_string());
	}
}