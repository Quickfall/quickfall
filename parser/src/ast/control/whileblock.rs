use commons::err::PositionedResult;
use lexer::token::LexerToken;

use crate::{ast::{control::ifelse::parse_condition_member, func::parse_node_body, tree::ASTTreeNode}};

pub fn parse_while_block(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;

	let cond = parse_condition_member(tokens, ind)?;

	tokens[*ind].expects(lexer::token::LexerTokenType::BRACKET_OPEN)?;

	let body = match parse_node_body(tokens, ind) {
		Ok(v) => v,
		Err(e) => return Err(e)
	};

	return Ok(Box::new(ASTTreeNode::WhileBlock { cond, body }));
}