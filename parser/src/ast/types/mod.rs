//! AST parsing for struct & layout parsing

use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};
use utils::hash::WithHash;

use crate::ast::tree::ASTTreeNode;

/// Parses a struct/layout member (field)
pub fn parse_types_field_member(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	let type_name = tokens[*ind].expects_keyword()?;

	*ind += 1;

	let field_name = tokens[*ind].expects_keyword()?;

	*ind += 1;

	return Ok(Box::new(ASTTreeNode::StructFieldMember { name: WithHash::new(field_name.0), member_type: type_name.1 }))
}

pub fn parse_type_declaration(tokens: &Vec<LexerToken>, ind: &mut usize, layout: bool) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;

	let type_name = tokens[*ind].expects_keyword()?;

	*ind += 1;
	tokens[*ind].expects(LexerTokenType::BracketOpen)?;

	*ind += 1;

	let mut members: Vec<Box<ASTTreeNode>> = Vec::new();	

	while tokens[*ind].tok_type != LexerTokenType::BracketClose {
		members.push(parse_types_field_member(tokens, ind)?);
	}

	return Ok(Box::new(ASTTreeNode::StructLayoutDeclaration { name: WithHash::new(type_name.0), layout, members }));
}