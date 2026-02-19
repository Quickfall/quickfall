//! 
//! Module containing lexer token-based utilities and classes
//! 

use commons::{Position, err::{PositionedError, PositionedResult}};

use crate::{LexerParseResult, LexerParsingError, toks::{comp::ComparingOperator, math::MathOperator}};

/// The token type for the lexer
#[derive(PartialEq, Debug)]
pub enum LexerTokenType {
    /// Represent the func keyword
    Function,
	ShadowFunction,

	Var,
	Struct,
	Layout,
	Lay,

	Static,

	/// 0: the operator
	/// 1: does the operator affect the original variable!
	MathOperator(MathOperator, bool),

	ComparingOperator(ComparingOperator),
	

    /// Represent the ret keyword
    Return,

	True,
	False,

	For,
	If,
	Else,
	While,

    EqualSign,
	ExclamationMark,

    Comma,
    Dot,
	Ampersand,

    BracketOpen,
    BracketClose,

    ParenOpen,
    ParenClose,

    ArrayOpen,
    ArrayClose,

    IntLit(i64),
    StringLit(String),

    AngelBracketOpen,
    AngelBracketClose,

    KEYWORD(String, u64),
    EndOfFile
}

#[derive(Debug)]
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
			LexerTokenType::IntLit(v) => return Ok(*v),
			_ => return Err(self.make_err("Expected int litteral here!"))
		};
	}

	pub fn expects_comp_operator(&self) -> PositionedResult<ComparingOperator> {
		match &self.tok_type {
			LexerTokenType::ComparingOperator(op) => return Ok(op.clone()),
			_ => return Err(self.make_err("Expected comparing operator here!"))
		};
	}

	pub fn expects_math_operator(&self) -> PositionedResult<(MathOperator, bool)> {
		match &self.tok_type {
			LexerTokenType::MathOperator(a, b) => return Ok((a.clone(), *b)),
			_ => return Err(self.make_err("Expected math operator here!"))
		};
	}

	pub fn expects_string_lit(&self) -> PositionedResult<String> {
		match &self.tok_type {
			LexerTokenType::StringLit(v) => return Ok(v.to_string()),
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