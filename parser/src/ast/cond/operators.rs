use lexer::token::LexerToken;

use crate::{ParserError, ParserResult};

#[derive(Debug)]
pub enum ConditionOperator {
	EQUAL,
	NOT_EQUAL,

	HIGHER, // A > B
	LOWER, // A < B

	HIGHEREQ, // A >= B
	LOWEREQ // A <= B
}

pub fn parse_condition_operator(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<ConditionOperator> {
	match &tokens[*ind] {
		LexerToken::EQUAL_SIGN => {
			*ind += 1;

			if tokens[*ind] == LexerToken::EQUAL_SIGN {
				return Ok(ConditionOperator::EQUAL);
			}
		},

		LexerToken::EXCLAMATION_MARK => {
			*ind += 1;

			if tokens[*ind] == LexerToken::EQUAL_SIGN {
				return Ok(ConditionOperator::NOT_EQUAL)
			}
		},

		LexerToken::ANGEL_BRACKET_OPEN => {
			*ind += 1;

			if tokens[*ind] == LexerToken::EQUAL_SIGN {
				return Ok(ConditionOperator::LOWEREQ);
			}

			return Ok(ConditionOperator::LOWER);
		},

		LexerToken::ANGEL_BRACKET_CLOSE => {
			*ind += 1;

			if tokens[*ind] == LexerToken::EQUAL_SIGN {
				return Ok(ConditionOperator::HIGHEREQ);
			}

			return Ok(ConditionOperator::HIGHER);
		},

		_ => {}
	}

	Err(ParserError::new(String::from("Pattern doesn't represent a valid condition operator!"), 0))
}