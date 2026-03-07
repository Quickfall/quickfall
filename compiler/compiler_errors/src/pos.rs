use std::{fs};

use colored::Colorize;
use compiler_utils::Position;

#[derive(Clone, Debug)]
pub struct BoundPosition {
	pub start: Position,
	pub end: Position
}

impl BoundPosition {
	pub fn from_size(start: Position, size: usize) -> Self {
		return BoundPosition { start: start.clone(), end: start.increment_by(size) }
	}
	
	pub fn new(start: Position, end: Position) -> Self {
		return Self { start, end }
	}

	pub fn get_bound(&self) -> String {
		let contents = match fs::read_to_string(&self.start.file_path) {
			Ok(v) => v,
			Err(_) => return "ERR: Cannot read source file".red().bold().to_string()
		};

		let split: Vec<&str> = contents.split('\n').collect();

		let start_line = split[self.start.line - 1];

		let mut str = start_line[self.start.col..].to_string();

		for line in self.start.line + 1..self.end.line {
			str += "\n";
			str += split[line - 1];
		}

		let end_line = split[self.end.line - 1];

		str += &end_line[..self.end.col - 1];

		return str;
	}

}