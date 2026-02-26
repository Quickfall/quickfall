//! 
//! Module containing lexer token-based utilities and classes
//! 

use errors::EXPECTED_TOKEN;
use commons::{Position};
use errors::{errs::{CompilerResult, ErrorKind, normal::CompilerError}, pos::BoundPosition};

use crate::{toks::{comp::ComparingOperator, math::MathOperator}};

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

    IntLit(i128, u64),
    StringLit(String),

    AngelBracketOpen,
    AngelBracketClose,

    KEYWORD(String, u64),
    EndOfFile
}

#[derive(Debug)]
pub struct LexerToken {
	pub tok_type: LexerTokenType,
	pub pos: Position,
	pub pos_size: usize
}

impl LexerToken {
	pub fn make_single_sized(pos: Position, t: LexerTokenType) -> Self {
		
		return LexerToken { tok_type: t, pos, pos_size: 1 };
	}

	pub fn new(start: Position, size: usize, t: LexerTokenType) -> Self {
		return LexerToken { tok_type:t , pos: start, pos_size: size }
	}

	pub fn is(&self, t: LexerTokenType) -> bool {
		return self.tok_type == t;
	}

	pub fn expects(&self, t: LexerTokenType) -> CompilerResult<bool> {
		if self.tok_type != t {
			return Err(self.make_err(format!(EXPECTED_TOKEN!(), t, self.tok_type), ErrorKind::Error));
		}

		return Ok(true);
	}

	pub fn expects_int_lit(&self) -> CompilerResult<(i128, u64)> {
		match &self.tok_type {
			LexerTokenType::IntLit(v, h) => return Ok((*v, *h)),
			_ => return Err(self.make_err(format!(EXPECTED_TOKEN!(), "int literal", self.tok_type), ErrorKind::Error))
		};
	}

	pub fn expects_comp_operator(&self) -> CompilerResult<ComparingOperator> {
		match &self.tok_type {
			LexerTokenType::ComparingOperator(op) => return Ok(op.clone()),
			_ => return Err(self.make_err(format!(EXPECTED_TOKEN!(), "comparing operator", self.tok_type), ErrorKind::Error))
		};
	}

	pub fn expects_math_operator(&self) -> CompilerResult<(MathOperator, bool)> {
		match &self.tok_type {
			LexerTokenType::MathOperator(a, b) => return Ok((a.clone(), *b)),
			_ => return Err(self.make_err(format!(EXPECTED_TOKEN!(), "math operator", self.tok_type), ErrorKind::Error))
		};
	}

	pub fn expects_string_lit(&self) -> CompilerResult<String> {
		match &self.tok_type {
			LexerTokenType::StringLit(v) => return Ok(v.to_string()),
			_ => return Err(self.make_err(format!(EXPECTED_TOKEN!(), "string literal", self.tok_type), ErrorKind::Error))
		};
	}

	pub fn expects_keyword(&self) -> CompilerResult<(String, u64)> {
		match &self.tok_type {
			LexerTokenType::KEYWORD(s, h) => return Ok((s.to_string(), *h)),
			_ => return Err(self.make_err(format!(EXPECTED_TOKEN!(), "keyword", self.tok_type), ErrorKind::Error))
		};
	}

	pub fn make_err(&self, err: String, kind: ErrorKind) -> CompilerError {
		return CompilerError::new(kind, err, BoundPosition::from_size(self.pos.clone(), self.pos_size));
	}

	pub fn as_keyword(&self) -> CompilerResult<(String, u64)> {
		match &self.tok_type {
			LexerTokenType::KEYWORD(str, hash) => Ok((str.clone(), *hash)),
			_ => Err(self.make_err(format!(EXPECTED_TOKEN!(), "keyword", self.tok_type), ErrorKind::Error))
		}
	}

	pub fn is_keyword(&self) -> bool {
		match &self.tok_type {
			LexerTokenType::KEYWORD(_, _) => true,
			_ => false
		}
	}

	pub fn get_end_pos(&self) -> Position {
		return self.pos.increment_by(self.pos_size);
	}

}