//! Parsing for type related features

use ast::{tree::{ASTTreeNode, ASTTreeNodeKind}, types::CompleteType};
use compiler_errors::errs::CompilerResult;
use lexer::token::{LexerToken, LexerTokenType};

pub fn parse_type(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<CompleteType> {
	let base_type = tokens[*ind].expects_keyword()?;

	let mut sizes: Vec<usize> = vec![];
	let mut types: Vec<u64> = vec![];

	while tokens[*ind].tok_type == LexerTokenType::Dot {
		*ind += 1;

		let size_def = tokens[*ind].expects_int_lit()?.0 as usize;
		sizes.push(size_def);
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

	return Ok(CompleteType { base_type: base_type.1, sizes, types })
}