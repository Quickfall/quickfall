use commons::err::PositionedResult;
use lexer::token::LexerToken;
use utils::hash::WithHash;

use crate::ast::tree::ASTTreeNode;

/// Parses a struct/layout member (field)
pub fn parse_types_field_member(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	let typeName = tokens[*ind].expects_keyword()?;

	*ind += 1;

	let fieldName = tokens[*ind].expects_keyword()?;

	return Ok(Box::new(ASTTreeNode::StructFieldMember { name: WithHash::new(fieldName.0), memberType: typeName.1 }))
}