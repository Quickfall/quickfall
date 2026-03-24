use std::collections::HashMap;

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use compiler_errors::errs::CompilerResult;
use compiler_utils::hash::HashedString;
use lexer::token::{LexerToken, LexerTokenType};

use crate::value::parse_ast_value;

pub fn parse_struct_initialize(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	tokens[*ind].expects(LexerTokenType::BracketOpen)?;

	let start = tokens[*ind].pos.clone();

	*ind += 1;

	let mut map: HashMap<HashedString, Box<ASTTreeNode>> = HashMap::new();

	while tokens[*ind].is_keyword() {
		let field_name = tokens[*ind].as_keyword()?;
		*ind += 1;

		let value = parse_ast_value(tokens, ind)?;

		map.insert(HashedString::new(field_name.0), value);

		*ind += 1;

		if tokens[*ind].tok_type == LexerTokenType::BracketClose {
			break;
		}

		tokens[*ind].expects(LexerTokenType::Comma)?;
	}

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StructVariableInitializerValue { map }, start, tokens[*ind].get_end_pos())))
}