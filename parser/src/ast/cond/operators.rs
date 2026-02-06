use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::{ParserError, ParserResult};

#[derive(Debug, PartialEq, Clone)]
pub enum ConditionOperator {
	EQUAL,
	NOT_EQUAL,

	HIGHER, // A > B
	LOWER, // A < B

	HIGHEREQ, // A >= B
	LOWEREQ // A <= B
}

pub fn parse_condition_operator(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<ConditionOperator> {
	match &tokens[*ind].tok_type {
		LexerTokenType::EQUAL_SIGN => {
			*ind += 1;

			tokens[*ind].expects(LexerTokenType::EQUAL_SIGN);

			return Ok(ConditionOperator::EQUAL)
		},

		LexerTokenType::EXCLAMATION_MARK => {
			*ind += 1;

			if tokens[*ind].tok_type == LexerTokenType::EQUAL_SIGN {
				return Ok(ConditionOperator::NOT_EQUAL)
			}
		},

		LexerTokenType::ANGEL_BRACKET_OPEN => {
			*ind += 1;

			if tokens[*ind].tok_type == LexerTokenType::EQUAL_SIGN {
				return Ok(ConditionOperator::LOWEREQ);
			}

			return Ok(ConditionOperator::LOWER);
		},

		LexerTokenType::ANGEL_BRACKET_CLOSE => {
			*ind += 1;

			if tokens[*ind].tok_type == LexerTokenType::EQUAL_SIGN {
				return Ok(ConditionOperator::HIGHEREQ);
			}

			return Ok(ConditionOperator::HIGHER);
		},

		_ => {}
	}

	Err(tokens[*ind].make_err("Token doesn't make a valid condition operator"))
}