//!
//! Parsing for if and else statements
//! 

use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::{ParserError, ParserResult, ast::{func::parse_node_body, parse_ast_value, tree::ASTTreeNode}};

pub fn parse_condition_member(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	tokens[*ind].expects(LexerTokenType::PAREN_OPEN)?;

	*ind += 1;
	let cond = parse_ast_value(tokens, ind)?;

	tokens[*ind].expects(LexerTokenType::PAREN_CLOSE)?;

	*ind += 1;

	return Ok(cond);
}

pub fn parse_if_statement(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;

	let cond = parse_condition_member(tokens, ind)?;

	tokens[*ind].expects(LexerTokenType::BRACKET_OPEN)?;

	let body = match parse_node_body(tokens, ind) {
		Ok(v) => v,
		Err(e) => return Err(e)
	};

	let mut elseStatement = None;

	if tokens[*ind + 1].tok_type == LexerTokenType::ELSE {
		*ind += 1;

		elseStatement = Some(parse_else_statement(tokens, ind)?);
	}

	return Ok(Box::new(ASTTreeNode::IfStatement { cond, body, elseStatement }));
}

pub fn parse_else_statement(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	*ind += 1;

	let mut cond = None;

	if tokens[*ind].tok_type == LexerTokenType::IF {
		*ind += 1;
		cond = Some(parse_condition_member(tokens, ind)?);
	}

	tokens[*ind].expects(LexerTokenType::BRACKET_OPEN)?;

	let body = match parse_node_body(tokens, ind) {
		Ok(v) => v,
		Err(e) => return Err(e)
	};

	if cond.is_some() {
		let mut elseStatement = None;

		if tokens[*ind + 1].tok_type == LexerTokenType::ELSE {
			*ind += 1;

			elseStatement = Some(parse_else_statement(tokens, ind)?);
		}

		return Ok(Box::new(ASTTreeNode::IfElseStatement { cond, body, elseStatement }));
	}

	return Ok(Box::new(ASTTreeNode::ElseStatement { body }));
}
