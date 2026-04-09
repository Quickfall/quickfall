//! AST parsing for struct & layout parsing

use errors::errs::CompilerResult;
use lexer::token::{LexerToken, LexerTokenType};
use utils::hash::WithHash;

use crate::ast::tree::{ASTTreeNode, ASTTreeNodeKind};

/// Parses a struct/layout member (field)
pub fn parse_types_field_member(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();
	let type_name = tokens[*ind].expects_keyword()?;

	*ind += 1;

	let field_name = tokens[*ind].expects_keyword()?;

	let end = tokens[*ind].get_end_pos().clone();

	*ind += 1;

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StructFieldMember { name: WithHash::new(field_name.0), member_type: type_name.1 }, start, end)))
}

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

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StructLayoutDeclaration { name: WithHash::new(type_name.0), layout, members }, start, end)));
}