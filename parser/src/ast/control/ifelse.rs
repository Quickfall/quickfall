//!
//! Parsing for if and else statements
//! 

use commons::err::{PositionedResult};
use lexer::token::{LexerToken, LexerTokenType};

use crate::{ast::{func::parse_node_body, parse_ast_value, tree::ASTTreeNode}};

pub fn parse_condition_member(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	tokens[*ind].expects(LexerTokenType::ParenOpen)?;

	*ind += 1;
	let cond = parse_ast_value(tokens, ind)?;

	tokens[*ind].expects(LexerTokenType::ParenClose)?;

	*ind += 1;

	return Ok(cond);
}

pub fn parse_if_statement(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;

	let cond = parse_condition_member(tokens, ind)?;

	tokens[*ind].expects(LexerTokenType::BracketOpen)?;

	let body = match parse_node_body(tokens, ind) {
		Ok(v) => v,
		Err(e) => return Err(e)
	};

	let mut depth = 1;
	let mut branches: Vec<Box<ASTTreeNode>> = vec![];

	if tokens[*ind].tok_type == LexerTokenType::Else {

		parse_else_statement(tokens, ind, &mut depth, &mut branches)?;
	}

	return Ok(Box::new(ASTTreeNode::IfStatement { cond, body, branches, depth }));
}

pub fn parse_else_statement(tokens: &Vec<LexerToken>, ind: &mut usize, depth: &mut usize, branches: &mut Vec<Box<ASTTreeNode>>) -> PositionedResult<bool> {
	*ind += 1;

	let mut cond = None;

	*depth += 1;

	if tokens[*ind].tok_type == LexerTokenType::If {
		*ind += 1;
		cond = Some(parse_condition_member(tokens, ind)?);
	}

	tokens[*ind].expects(LexerTokenType::BracketOpen)?;

	let body = match parse_node_body(tokens, ind) {
		Ok(v) => v,
		Err(e) => return Err(e)
	};

	if cond.is_some() {
		branches.push(Box::new(ASTTreeNode::IfElseStatement { cond, body }));

		if tokens[*ind].tok_type == LexerTokenType::Else {
			parse_else_statement(tokens, ind, depth, branches)?;
		}

		return Ok(true);
	}

	branches.push(Box::new(ASTTreeNode::ElseStatement { body }));
	return Ok(true);
}
