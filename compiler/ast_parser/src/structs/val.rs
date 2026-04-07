use std::collections::HashMap;

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use compiler_utils::hash::{HashedString, SelfHash};
use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::{value::parse_ast_value};

pub fn parse_struct_initialize(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	*ind += 1;

	let mut map: HashMap<SelfHash, Box<ASTTreeNode>> = HashMap::new();

	while tokens[*ind].is_keyword() {
		let field_name = tokens[*ind].expects_keyword()?;
		*ind += 1;

		let value = parse_ast_value(tokens, ind)?;

		map.insert(SelfHash { hash: HashedString::new(field_name.0).hash }, value);

		//*ind += 1;

		if tokens[*ind].tok_type == LexerTokenType::BracketClose {
			break;
		}

		tokens[*ind].expects(LexerTokenType::Comma)?;
		*ind += 1;
	}

	*ind += 1;

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StructInitializer { map }, start, tokens[*ind].get_end_pos())))
}