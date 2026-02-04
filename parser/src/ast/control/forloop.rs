use lexer::token::LexerToken;

use crate::{ParserError, ParserResult, ast::{func::parse_node_body, parse_ast_node, parse_ast_value, tree::ASTTreeNode, var::decl::parse_variable_declaration}};

pub fn parse_for_loop(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	*ind += 1;

	if tokens[*ind] != LexerToken::PAREN_OPEN {
		return Err(ParserError::new(String::from("Requires ("), 0));
	}

	let initial = parse_variable_declaration(tokens, ind)?;

	if tokens[*ind] != LexerToken::COMMA {
		return Err(ParserError::new(String::from("Requires for bodies to seperated by commas!"), 0));
	}

	*ind += 1;
	let cond = parse_ast_value(tokens, ind)?;

	if tokens[*ind] != LexerToken::COMMA {
		return Err(ParserError::new(String::from("Requires for bodies to seperated by commas!"), 0));
	}
	*ind += 1;


	let increment = parse_ast_node(tokens, ind)?;

	*ind += 1;

	if tokens[*ind] != LexerToken::PAREN_CLOSE {
		return Err(ParserError::new(String::from("Requires )"), 0));
	}

	*ind += 1;

	if tokens[*ind] != LexerToken::BRACKET_OPEN {
		return Err(ParserError::new(String::from("Requires {"), 0));
	}

	let body = parse_node_body(tokens, ind)?;

	return Ok(Box::new(ASTTreeNode::ForBlock { initialState: initial, cond, increment, body }));
}