//! AST parsing for number & string literals

use compiler_errors::errs::CompilerResult;
use lexer::token::{LexerToken};

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};

pub fn parse_integer_literal(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let val = tokens[*ind].expects_int_lit()?;
	*ind += 1;

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::IntegerLit { val: val.0, hash: val.1 }, tokens[*ind].pos.clone(), tokens[*ind].get_end_pos())));
}

pub fn parse_string_literal(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let val = tokens[*ind].expects_string_lit()?;
	*ind += 1;

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StringLit(val), tokens[*ind].pos.clone(), tokens[*ind].get_end_pos())));
}