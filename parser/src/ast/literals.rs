use lexer::token::{LexerToken, LexerTokenType};

use crate::{ParserError, ParserResult, ast::tree::ASTTreeNode};

pub fn parse_integer_literal(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	let mut val;
	tokens[*ind].expects(lexer::token::LexerTokenType::INT_LIT(val));
	
	*ind += 1;

	return Ok(Box::new(ASTTreeNode::IntegerLit(val)));
}

pub fn parse_string_literal(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	let val;
	tokens[*ind].expects(LexerTokenType::STRING_LIT(val));
}