//! 
//! The core of the lexer crate. 
//! The lexer is the first step of parsing within Quickfall.
//! 

use commons::Position;

pub mod token;
pub mod lexer;
pub mod toks;

pub struct SizedPosition {
	pub pos: Position,
	pub size: usize
}

impl SizedPosition {
	pub fn new(pos: Position, size: usize) -> Self {
		return SizedPosition { pos, size }
	}

	pub fn get_line_str(&self) -> Result<String, std::io::Error> {
		return self.pos.get_line_content();
	}

	pub fn get_str_slice(&self) -> Result<String, std::io::Error> {
		return self.pos.get_line_content()[self.pos.col - 1..self.pos.col + self.size - 1];
	}


}
