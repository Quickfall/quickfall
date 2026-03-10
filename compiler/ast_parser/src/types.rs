//! Parsing for type related features

use ast::{types::CompleteType};
use compiler_errors::errs::CompilerResult;
use lexer::token::{LexerToken, LexerTokenType};

pub fn parse_type(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<CompleteType> {
	let pointer;
	let pointer_array;

	if tokens[*ind].tok_type == LexerTokenType::Asterisk {
		*ind += 1;
		pointer = true;
	} else {
		pointer = false;
	}

	if tokens[*ind].tok_type == LexerTokenType::ArrayOpen {
		*ind += 1;

		tokens[*ind].expects(LexerTokenType::ArrayClose)?;

		pointer_array = true;
		*ind += 1;
	} else {
		pointer_array = false;
	}

	let base_type = tokens[*ind].expects_keyword()?;
	*ind += 1;

	let mut sizes: Vec<usize> = vec![];
	let mut types: Vec<u64> = vec![];

	while tokens[*ind].tok_type == LexerTokenType::Dot {
		*ind += 1;

		let size_def = tokens[*ind].expects_int_lit()?.0 as usize;
		sizes.push(size_def);

		*ind += 1;
	}

	if tokens[*ind].tok_type == LexerTokenType::AngelBracketOpen {
		*ind += 1;

		loop {
			let type_spec = tokens[*ind].expects_keyword()?;

			types.push(type_spec.1);

			*ind += 1;
			
			if tokens[*ind].tok_type == LexerTokenType::AngelBracketClose {
				break;
			}

			tokens[*ind].expects(LexerTokenType::Comma)?;
		}
	}

	let array_sz: usize;

	if tokens[*ind].tok_type == LexerTokenType::ArrayOpen {
		*ind += 1;


		array_sz = tokens[*ind].expects_int_lit()?.0 as usize;
		*ind += 1;

		tokens[*ind].expects(LexerTokenType::ArrayClose)?;

		*ind += 1;
	} else {
		array_sz = 0;
	}

	return Ok(CompleteType { base_type: base_type.1, sizes, types, pointer, pointer_array, array_sz })
}