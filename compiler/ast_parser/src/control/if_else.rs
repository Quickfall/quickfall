//!
//! Parsing for if and else statements
//! 

use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};

use ast::{tree::{ASTTreeNode, ASTTreeNodeKind}};

use crate::{functions::parse_node_body, value::parse_ast_value};

pub fn parse_condition_member(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<Box<ASTTreeNode>> {
	tokens[*ind].expects(LexerTokenType::ParenOpen)?;

	*ind += 1;
	let cond = parse_ast_value(tokens, ind)?;

	tokens[*ind].expects(LexerTokenType::ParenClose)?;

	*ind += 1;

	return Ok(cond);
}

pub fn parse_if_statement(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();
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

	let end = tokens[*ind].get_end_pos();

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::IfStatement { cond, body, branches, depth }, start, end)));
}

pub fn parse_else_statement(tokens: &Vec<LexerToken>, ind: &mut usize, depth: &mut usize, branches: &mut Vec<Box<ASTTreeNode>>) -> DiagnosticResult<bool> {
	let start = tokens[*ind].pos.clone();

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

	let end = tokens[*ind].get_end_pos();

	if cond.is_some() {
		branches.push(Box::new(ASTTreeNode::new(ASTTreeNodeKind::IfElseStatement { cond, body }, start, end)));

		if tokens[*ind].tok_type == LexerTokenType::Else {
			parse_else_statement(tokens, ind, depth, branches)?;
		}

		return Ok(true);
	}

	branches.push(Box::new(ASTTreeNode::new(ASTTreeNodeKind::ElseStatement { body }, start, end)));
	return Ok(true);
}