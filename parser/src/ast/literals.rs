use lexer::token::LexerToken;

use crate::{ParserError, ParserResult, ast::tree::ASTTreeNode};

pub fn parse_integer_literal(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	match &tokens[*ind] {
		LexerToken::INT_LIT(val) => {
			*ind += 1;
			return Ok(Box::new(ASTTreeNode::IntegerLit(*val)))
		},
		_ => Err(ParserError::new(String::from("Given token is not an integer literal"), 0))
	}
}

pub fn parse_string_literal(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	match &tokens[*ind] {
		LexerToken::STRING_LIT(val) => {
			*ind += 1;
			return Ok(Box::new(ASTTreeNode::StringLit(String::clone(val))))
		},
		_ => return Err(ParserError::new(String::from("Given token is not an string literal"), 0))
	};
}