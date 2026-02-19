use commons::err::{PositionedError, PositionedResult};
use lexer::token::{LexerToken, LexerTokenType};
use utils::hash::WithHash;

use crate::{ast::{func::{parse_function_arguments, parse_node_body}, tree::ASTTreeNode}};

pub fn parse_shadow_function_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;
	let function_name = tokens[*ind].expects_keyword()?;

	*ind += 1;
	tokens[*ind].expects(LexerTokenType::ParenOpen)?;

	let args = parse_function_arguments(tokens, ind)?;

	*ind += 1;

	let mut retType = None;

	if tokens[*ind].is_keyword() {
		retType = Some(tokens[*ind].expects_keyword()?.1);
		*ind += 1;
	}

	return Ok(Box::new(ASTTreeNode::ShadowFunctionDeclaration { func_name: WithHash::new(function_name.0), args, returnType: retType }))
}

pub fn parse_function_declaraction(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;
	let function_name = tokens[*ind].expects_keyword()?;

	*ind += 1;
	tokens[*ind].expects(LexerTokenType::ParenOpen)?;

	let args = parse_function_arguments(tokens, ind)?;

	*ind += 1;

	let mut retType = None;

	if tokens[*ind].is_keyword() {
		retType = Some(tokens[*ind].expects_keyword()?.1);
		*ind += 1;
	}

	tokens[*ind].expects(LexerTokenType::BracketOpen)?;

	let body = parse_node_body(tokens, ind)?;

	return Ok(Box::new(ASTTreeNode::FunctionDeclaration { func_name: WithHash::new(function_name.0), args, body, returnType: retType }));
}