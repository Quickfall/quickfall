use core::fmt;

pub mod token;
pub mod lexer;

type LexerParseResult<T> = std::result::Result<T, LexerParsingError>;

#[derive(Debug, Clone)]
pub struct LexerParsingError {
    reason: String,

    position: usize
}

impl LexerParsingError {
    pub fn new(reason: String, pos: usize) -> Self {
        LexerParsingError { reason, position: pos }
    }
}

impl fmt::Display for LexerParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer Parsing Error: {} at position {}", self.reason, self.position)
    }
}