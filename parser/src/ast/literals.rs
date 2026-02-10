use commons::err::{PositionedResult};
use lexer::token::{LexerToken};

use crate::{ast::tree::ASTTreeNode};

pub fn parse_integer_literal(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	let val = tokens[*ind].expects_int_lit()?;
	*ind += 1;

	return Ok(Box::new(ASTTreeNode::IntegerLit(val)));
}

pub fn parse_string_literal(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	let val = tokens[*ind].expects_string_lit()?;
	*ind += 1;

	return Ok(Box::new(ASTTreeNode::StringLit(val)));
}