use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::{ast::{func::parse_node_body, parse_ast_node, parse_ast_value, tree::ASTTreeNode, var::decl::parse_variable_declaration}};

pub fn parse_for_loop(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;

	tokens[*ind].expects(LexerTokenType::ParenOpen)?;

	let initial = parse_variable_declaration(tokens, ind)?;

	tokens[*ind].expects(LexerTokenType::Comma)?;

	*ind += 1;
	let cond = parse_ast_value(tokens, ind)?;

	tokens[*ind].expects(LexerTokenType::Comma)?;
	*ind += 1;

	let increment = parse_ast_node(tokens, ind)?;

	*ind += 1;

	tokens[*ind].expects(LexerTokenType::ParenClose)?;
	*ind += 1;

	tokens[*ind].expects(LexerTokenType::BracketOpen)?;

	let body = parse_node_body(tokens, ind)?;

	return Ok(Box::new(ASTTreeNode::ForBlock { initial_state: initial, cond, increment, body }));
}