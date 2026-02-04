use lexer::token::LexerToken;

use crate::{ParserError, ParserResult, ast::{control::ifelse::parse_condition_member, func::parse_node_body, tree::ASTTreeNode}};

pub fn parse_while_block(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	*ind += 1;

	let cond = parse_condition_member(tokens, ind)?;

	if tokens[*ind] != LexerToken::BRACKET_OPEN {
		return Err(ParserError::new(String::from("Expected block body!"), 0));
	}

	let body = match parse_node_body(tokens, ind) {
		Ok(v) => v,
		Err(e) => return Err(e)
	};

	return Ok(Box::new(ASTTreeNode::WhileBlock { cond, body }));
}