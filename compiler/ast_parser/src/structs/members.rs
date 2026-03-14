use compiler_errors::errs::CompilerResult;
use compiler_utils::hash::HashedString;
use lexer::token::{LexerToken};

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};

use crate::types::parse_type;

/// Parses a struct/layout member (field)
pub fn parse_types_field_member(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();
	let member_type = parse_type(tokens, ind)?;

//	*ind += 1;

	let field_name = tokens[*ind].expects_keyword()?;

	let end = tokens[*ind].get_end_pos().clone();

	*ind += 1;

	println!("End: {:#?}", tokens[*ind]);

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StructFieldMember { name: HashedString::new(field_name.0), member_type }, start, end)))
}