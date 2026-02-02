use lexer::token::LexerToken;

use crate::{ParserError, ParserResult, ast::tree::ASTTreeNode};

pub fn parse_function_call(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	return Err(ParserError::new(String::from("Invalid"), 0));
}