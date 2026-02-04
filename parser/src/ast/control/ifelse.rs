//!
//! Parsing for if and else statements
//! 

use lexer::token::LexerToken;

use crate::{ParserError, ParserResult, ast::{func::parse_node_body, parse_ast_value, tree::ASTTreeNode}};

pub fn parse_condition_member(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	if tokens[*ind] != LexerToken::PAREN_OPEN {
		return Err(ParserError::new(String::from("If statements must be followed by condition!"), 0));
	}

	*ind += 1;
	let cond = parse_ast_value(tokens, ind)?;

	if tokens[*ind] != LexerToken::PAREN_CLOSE {
		return Err(ParserError::new(String::from("Conditions must be closed by paren!"), 0));
	}

	*ind += 1;

	return Ok(cond);
}

pub fn parse_if_statement(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	*ind += 1;

	let cond = parse_condition_member(tokens, ind)?;

	if tokens[*ind] != LexerToken::BRACKET_OPEN {
		return Err(ParserError::new(String::from("Condition must be followed by body!"), 0));
	}

	let body = match parse_node_body(tokens, ind) {
		Ok(v) => v,
		Err(e) => return Err(e)
	};

	let mut elseStatement = None;

	if tokens[*ind + 1] == LexerToken::ELSE {
		*ind += 1;

		elseStatement = Some(parse_else_statement(tokens, ind)?);
	}

	return Ok(Box::new(ASTTreeNode::IfStatement { cond, body, elseStatement }));
}

pub fn parse_else_statement(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	*ind += 1;

	let mut cond = None;

	if tokens[*ind] == LexerToken::IF {
		*ind += 1;
		cond = Some(parse_condition_member(tokens, ind)?);
	}

	if tokens[*ind] != LexerToken::BRACKET_OPEN {
		return Err(ParserError::new(String::from("Condition must be followed by body!"), 0));
	}

	let body = match parse_node_body(tokens, ind) {
		Ok(v) => v,
		Err(e) => return Err(e)
	};

	if cond.is_some() {
		let mut elseStatement = None;

		if tokens[*ind + 1] == LexerToken::ELSE {
			*ind += 1;

			elseStatement = Some(parse_else_statement(tokens, ind)?);
		}

		return Ok(Box::new(ASTTreeNode::IfElseStatement { cond, body, elseStatement }));
	}

	return Ok(Box::new(ASTTreeNode::ElseStatement { body }));
}
