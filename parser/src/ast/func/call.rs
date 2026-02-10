use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};
use utils::hash::WithHash;

use crate::{ast::{parse_ast_value, tree::ASTTreeNode}};

pub fn parse_function_call(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {

	let func = WithHash::new(tokens[*ind].as_keyword().unwrap().0);

	*ind += 1;

	tokens[*ind].expects(LexerTokenType::ParenOpen)?;

	*ind += 1;

	let mut vals: Vec<Box<ASTTreeNode>> = Vec::new();
	
	while tokens[*ind].tok_type != LexerTokenType::ParenClose {
		vals.push(parse_ast_value(tokens, ind)?);
		
		tokens[*ind].expects(LexerTokenType::Comma)?;

		*ind += 1;
	}

	*ind += 1;

	return Ok(Box::new(ASTTreeNode::FunctionCall { func , args: vals }))
}