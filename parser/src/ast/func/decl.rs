use lexer::token::LexerToken;
use utils::hash::WithHash;

use crate::{ParserError, ParserResult, ast::{func::{parse_function_arguments, parse_node_body}, tree::ASTTreeNode}};

pub fn parse_function_declaraction(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	*ind += 1;
	let functionName = match tokens[*ind].as_keyword() {
		Ok(val) => val,
		Err(e) => return Err(ParserError::new(String::from("Function name wasn't a keyword!"), 0))
	};

	*ind += 1;
	if tokens[*ind] != LexerToken::PAREN_OPEN {
		return Err(ParserError::new(String::from("Function name must be followed by arguments!"), 0));
	}

	let args = parse_function_arguments(tokens, ind)?;

	*ind += 1;
	if tokens[*ind] != LexerToken::BRACKET_OPEN {
		return Err(ParserError::new(String::from("Expected function body declaration after arguments!"), 0));
	}

	let body = parse_node_body(tokens, ind)?;

	return Ok(Box::new(ASTTreeNode::FunctionDeclaration { funcName: WithHash::new(functionName.0), args, body }));
}