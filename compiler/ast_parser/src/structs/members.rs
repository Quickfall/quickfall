use compiler_utils::hash::HashedString;
use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken};

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};

use crate::types::parse_type;

/// Parses a struct/layout member (field)
pub fn parse_types_field_member(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();
	let member_type = parse_type(tokens, ind)?;

	let field_name = tokens[*ind].expects_keyword()?;

	let end = tokens[*ind].get_end_pos().clone();

	*ind += 1;

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StructFieldMember { name: HashedString::new(field_name.0), member_type }, start, end)))
}