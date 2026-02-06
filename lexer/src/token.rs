//! 
//! Module containing lexer token-based utilities and classes
//! 

use std::any::Any;

use commons::{Position, err::{PositionedError, PositionedResult}};

use crate::{LexerParseResult, LexerParsingError};

/// The token type for the lexer
#[derive(PartialEq, Debug)]
pub enum LexerTokenType {
    /// Represent the func keyword
    FUNCTION,

	VAR,
	STRUCT,
	LAYOUT,
	LAY,
		
	MATH_ADD,
	MATH_SUBTRACT,
	MATH_DIVIDE,
	MATH_MULTIPLY,
	

    /// Represent the ret keyword
    RETURN,

	TRUE,
	FALSE,

	FOR,
	IF,
	ELSE,
	WHILE,

    EQUAL_SIGN,
	EXCLAMATION_MARK,

    COMMA,
    DOT,
	AMPERSAND,

    BRACKET_OPEN,
    BRACKET_CLOSE,

    PAREN_OPEN,
    PAREN_CLOSE,

    ARRAY_OPEN,
    ARRAY_CLOSE,

    INT_LIT(i64),
    STRING_LIT(String),

    ANGEL_BRACKET_OPEN,
    ANGEL_BRACKET_CLOSE,

    KEYWORD(String, u64),
    END_OF_FILE
}

pub struct LexerToken {
	pub tok_type: LexerTokenType,
	pub pos: Position, // Valid tokens require a position
	pub end_pos: Position
}

impl LexerToken {
	pub fn make_single_sized(pos: Position, t: LexerTokenType) -> Self {
		let end = pos.increment_by(1);
		return LexerToken { tok_type: t, pos, end_pos: end };
	}

	pub fn new(start: Position, end: Position, t: LexerTokenType) -> Self {
		return LexerToken { tok_type:t , pos: start, end_pos: end }
	}

	pub fn is(&self, t: LexerTokenType) -> bool {
		return self.tok_type == t;
	}

	pub fn expects(&self, t: LexerTokenType) -> PositionedResult<bool> {
		if self.tok_type != t {
			return Err(PositionedError::new(self.pos.clone(), self.end_pos.clone(), format!("Expected {:#?} token but instead got {:#?}!", t, self.tok_type)))
		}

		return Ok(true);
	}

	pub fn expects_int_lit(&self) -> PositionedResult<i64> {
		match &self.tok_type {
			LexerTokenType::INT_LIT(v) => return Ok(*v),
			_ => return Err(self.make_err("Expected int litteral here!"))
		};
	}

	pub fn expects_string_lit(&self) -> PositionedResult<String> {
		match &self.tok_type {
			LexerTokenType::STRING_LIT(v) => return Ok(v.to_string()),
			_ => return Err(self.make_err("Expected string litteral here!"))
		};
	}

	pub fn expects_keyword(&self) -> PositionedResult<(String, u64)> {
		match &self.tok_type {
			LexerTokenType::KEYWORD(s, h) => return Ok((s.to_string(), *h)),
			_ => return Err(self.make_err("Expected keyword here!"))
		};
	}

	pub fn make_err(&self, err: &str) -> PositionedError {
		return PositionedError::new(self.pos.clone(), self.end_pos.clone(), String::from(err));
	}

	pub fn as_keyword(&self) -> LexerParseResult<(String, u64)> {
		match &self.tok_type {
			LexerTokenType::KEYWORD(str, hash) => Ok((str.clone(), *hash)),
			_ => Err(LexerParsingError::new(String::from("Token is not a keyword!"), 0))
		}
	}

	pub fn is_keyword(&self) -> bool {
		match &self.tok_type {
			LexerTokenType::KEYWORD(_, _) => true,
			_ => false
		}
	}
}