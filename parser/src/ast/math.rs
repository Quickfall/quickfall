use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};

/// The different operators
#[derive(Debug, PartialEq, Clone)]
pub enum MathOperator {
	ADD,
	SUBSTRACT,
	MULTIPLY,
	DIVIDE
}

/// Obtains the operator from the token type
pub fn operator_from_token_type(token: LexerToken) -> PositionedResult<MathOperator> {
	match &token.tok_type {
		LexerTokenType::MATH_ADD => return Ok(MathOperator::ADD),
		LexerTokenType::MATH_SUBTRACT => return Ok(MathOperator::SUBSTRACT),
		LexerTokenType::MATH_MULTIPLY => return Ok(MathOperator::MULTIPLY),
		LexerTokenType::MATH_DIVIDE => return Ok(MathOperator::DIVIDE),

		_ => return Err(token.make_err("Expected math operator!"))
	}
}