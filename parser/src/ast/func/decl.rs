use errors::errs::CompilerResult;
use lexer::token::{LexerToken, LexerTokenType};
use utils::hash::WithHash;

use crate::ast::{func::{parse_function_arguments, parse_node_body}, tree::{ASTTreeNode, ASTTreeNodeKind}};

pub fn parse_shadow_function_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	*ind += 1;
	let function_name = tokens[*ind].expects_keyword()?;

	*ind += 1;
	tokens[*ind].expects(LexerTokenType::ParenOpen)?;

	let args = parse_function_arguments(tokens, ind)?;

	*ind += 1;

	let mut ret_type = None;
	let end;

	if tokens[*ind].is_keyword() {
		ret_type = Some(tokens[*ind].expects_keyword()?.1);
		*ind += 1;

		end = tokens[*ind].get_end_pos().clone();
	} else {
		end = tokens[*ind - 1].get_end_pos().clone();
	}

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::ShadowFunctionDeclaration { func_name: WithHash::new(function_name.0), args, return_type: ret_type }, start, end)))
}

pub fn parse_function_declaraction(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	*ind += 1;
	let function_name = tokens[*ind].expects_keyword()?;

	*ind += 1;
	tokens[*ind].expects(LexerTokenType::ParenOpen)?;

	let args = parse_function_arguments(tokens, ind)?;

	*ind += 1;

	let mut ret_type = None;

	if tokens[*ind].is_keyword() {
		ret_type = Some(tokens[*ind].expects_keyword()?.1);
		*ind += 1;
	}

	tokens[*ind].expects(LexerTokenType::BracketOpen)?;

	let body = parse_node_body(tokens, ind)?;

	let end = tokens[*ind - 1].get_end_pos();

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::FunctionDeclaration { func_name: WithHash::new(function_name.0), args, body, return_type: ret_type }, start, end)));
}