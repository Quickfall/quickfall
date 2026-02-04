//! 
//! Module containing lexer token-based utilities and classes
//! 

use crate::{LexerParseResult, LexerParsingError};

/// The token type for the lexer
#[derive(PartialEq, Debug)]
pub enum LexerToken {
    /// Represent the func keyword
    FUNCTION,

	VAR,
	STRUCT,
	LAYOUT,
	LAY,
		
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

impl LexerToken {
	pub fn as_keyword(&self) -> LexerParseResult<(String, u64)> {
		match self {
			LexerToken::KEYWORD(str, hash) => Ok((str.clone(), *hash)),
			_ => Err(LexerParsingError::new(String::from("Token is not a keyword!"), 0))
		}
	}

	pub fn is_keyword(&self) -> bool {
		match self {
			LexerToken::KEYWORD(_, _) => true,
			_ => false
		}
	}
}

pub struct LexerTokenPosition {
	line: usize,
	col: usize
}