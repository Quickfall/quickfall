//! Parsing for type related features

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use compiler_errors::errs::CompilerResult;
use lexer::token::{LexerToken, LexerTokenType};

pub fn parse_type(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<ASTTreeNode> {
	let base_type = tokens[*ind].expects_keyword()?;
	let start = tokens[*ind].pos.clone();

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

	let end = tokens[*ind].get_end_pos();

	return Ok(ASTTreeNode::new(ASTTreeNodeKind::ComplexType { t_hash: base_type.1, size_definitions: sizes, extended_types: types }, start, end))
}