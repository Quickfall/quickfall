//! Parser module for return statements

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use compiler_errors::errs::CompilerResult;
use lexer::token::LexerToken;

use crate::value::parse_ast_value;

pub fn parse_function_return_statement(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	*ind += 1;

	let val = parse_ast_value(tokens, ind)?;

	let end = tokens[*ind].get_end_pos();

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::ReturnStatement { val: Some(val) }, start, end)))
}