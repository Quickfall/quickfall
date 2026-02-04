use lexer::token::LexerToken;
use utils::hash::WithHash;

use crate::{ParserError, ParserResult, ast::{parse_ast_value, tree::ASTTreeNode}};

pub fn parse_function_call(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {

	let func = WithHash::new(tokens[*ind].as_keyword().unwrap().0);

	*ind += 1;

	if tokens[*ind] != LexerToken::PAREN_OPEN {
		return Err(ParserError::new(String::from("Function must start with paren given arguments!"), 0));
	}

	*ind += 1;

	let mut vals: Vec<Box<ASTTreeNode>> = Vec::new();
	
	while tokens[*ind] != LexerToken::PAREN_CLOSE {
		vals.push(parse_ast_value(tokens, ind)?);
		
		if tokens[*ind] != LexerToken::COMMA {
			return Err(ParserError::new(String::from("Expected comma seperated args"), 0));
		}

		*ind += 1;
	}

	*ind += 1;

	return Ok(Box::new(ASTTreeNode::FunctionCall { func , args: vals }))
}