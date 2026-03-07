use compiler_errors::errs::CompilerResult;
use compiler_utils::hash::HashedString;
use lexer::token::{LexerToken, LexerTokenType};

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};

use crate::structs::members::parse_types_field_member;

pub mod members;

pub fn parse_type_declaration(tokens: &Vec<LexerToken>, ind: &mut usize, layout: bool) -> CompilerResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	*ind += 1;

	let type_name = tokens[*ind].expects_keyword()?;

	*ind += 1;
	tokens[*ind].expects(LexerTokenType::BracketOpen)?;

	*ind += 1;

	let mut members: Vec<Box<ASTTreeNode>> = Vec::new();	

	while tokens[*ind].tok_type != LexerTokenType::BracketClose {
		members.push(parse_types_field_member(tokens, ind)?);
	}

	let end = tokens[*ind].get_end_pos().clone();

	*ind += 1;

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StructLayoutDeclaration { name: HashedString::new(type_name.0), layout, members }, start, end)));
}