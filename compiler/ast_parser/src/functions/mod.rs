//! Parser module for functions

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use compiler_errors::errs::CompilerResult;
use compiler_utils::hash::WithHash;
use lexer::token::{LexerToken, LexerTokenType};

use crate::{functions::arguments::parse_function_arguments, parser::parse_ast_node_in_body, types::parse_type, value::parse_ast_value};

pub mod shadow;
pub mod arguments;
pub mod returns;

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
		ret_type = Some(parse_type(tokens, ind)?);
		*ind += 1;
	}

	tokens[*ind].expects(LexerTokenType::BracketOpen)?;

	let body = parse_node_body(tokens, ind)?;

	let end = tokens[*ind - 1].get_end_pos();

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::FunctionDeclaration { func_name: WithHash::new(function_name.0), args, body, return_type: ret_type }, start, end)));
}

pub fn parse_function_call(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	let func = WithHash::new(tokens[*ind].as_keyword().unwrap().0);

	*ind += 1;

	tokens[*ind].expects(LexerTokenType::ParenOpen)?;

	*ind += 1;

	let mut vals: Vec<Box<ASTTreeNode>> = Vec::new();
	
	while tokens[*ind].tok_type != LexerTokenType::ParenClose {
		vals.push(parse_ast_value(tokens, ind)?);

		if tokens[*ind].tok_type == LexerTokenType::ParenClose {
			break;
		}

		tokens[*ind].expects(LexerTokenType::Comma)?;

		*ind += 1;
	}

	let end = tokens[*ind].get_end_pos().clone();

	*ind += 1;

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::FunctionCall { func , args: vals }, start, end)))
}

pub fn parse_node_body(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Vec<Box<ASTTreeNode>>> {
    *ind += 1;

    let mut tok: &LexerToken = &tokens[*ind];
    let mut body: Vec<Box<ASTTreeNode>> = Vec::new();

    while tok.tok_type != LexerTokenType::EndOfFile && tok.tok_type != LexerTokenType::BracketClose {
        let n = parse_ast_node_in_body(tokens, ind)?;

        body.push(n);

        tok = &tokens[*ind];
    }

	*ind += 1;

    return Ok(body);
}