use core::fmt;

pub mod ast;

type ParserResult<T> = std::result::Result<T, ParserError>;

#[derive(Debug, Clone)]
pub struct ParserError {
	reason: String,
	position: usize
}

impl ParserError {
	pub fn new(reason: String, position: usize) -> Self {
		ParserError { reason, position }
	}
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing Error: {} at position {}", self.reason, self.position)
    }
}