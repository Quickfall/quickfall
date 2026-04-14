//! Operator related utils

use compiler_utils::operators::{ComparingOperator, MathOperator, MathOperatorType};
use diagnostics::{DiagnosticResult, builders::make_unexpected_simple_error};
use lexer::token::{LexerToken, LexerTokenType};

pub fn parse_math_operator(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<MathOperator> {
	let op = match tokens[*ind].tok_type {
		LexerTokenType::Plus => MathOperatorType::Add,
		LexerTokenType::PercentSign => MathOperatorType::Modulo,
		LexerTokenType::Minus => MathOperatorType::Subtract,
		LexerTokenType::Asterisk => {
			if tokens[*ind + 1].tok_type == LexerTokenType::Asterisk {
				*ind += 1;

				MathOperatorType::ShiftLeft
			} else {
				MathOperatorType::Multiply
			}
		}
		LexerTokenType::Divide => {
			if tokens[*ind + 1].tok_type == LexerTokenType::Divide {
				*ind += 1;

				MathOperatorType::ShiftRight
			} else {
				MathOperatorType::Divide
			}
		}

		_ => return Err(make_unexpected_simple_error(&tokens[*ind], &tokens[*ind].tok_type).into())
	};

	*ind += 1;

	let assigns = match tokens[*ind].tok_type {
		LexerTokenType::EqualSign => true,
		_ => false
	};

	*ind += 1;

	let fast = match tokens[*ind].tok_type {
		LexerTokenType::Tidle => true,
		_ => false
	};

	*ind += 1;

	return Ok(MathOperator { operator: op, assigns, fast });
}

pub fn parse_compare_operator(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<ComparingOperator> {
	let eq = match tokens[*ind + 1].tok_type {
		LexerTokenType::EqualSign => true,
		_ => false
	};

	let op = match tokens[*ind].tok_type {
		LexerTokenType::EqualSign => {
			tokens[*ind + 1].expects(LexerTokenType::EqualSign)?;

			ComparingOperator::Equal
		},

		LexerTokenType::ExclamationMark => {
			tokens[*ind + 1].expects(LexerTokenType::EqualSign)?;

			ComparingOperator::NotEqual
		},

		LexerTokenType::AngelBracketOpen => {
			if eq {
				ComparingOperator::LowerEqual
			} else {
				ComparingOperator::Lower
			}
		},

		LexerTokenType::AngelBracketClose => {
			if eq {
				ComparingOperator::HigherEqual
			} else {
				ComparingOperator::Higher
			}
		},

		_ => {
			return Err(make_unexpected_simple_error(&tokens[*ind], &tokens[*ind].tok_type).into())
		}
	};

	return Ok(op);
}