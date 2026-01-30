use lexer::token::LexerToken;

use crate::{ParserError, ParserResult, ast::tree::ASTTreeNode};

pub mod operators;

pub fn parse_condition_seperator(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	*ind += 1;

	if tokens[*ind] == LexerToken::AMPERSAND {
		return Ok(Box::new(ASTTreeNode::ConditionSeperator));
	}

	return Err(ParserError::new(String::from("Given pattern is not a condition seperator!"), 0));
}
