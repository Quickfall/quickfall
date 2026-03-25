//!
//! The parser for the Quickfall AST
//!

use ast::ctx::ParserCtx;
use compiler_errors::{AST_INVALID_TREE, errs::{CompilerResult, ErrorKind}};
use lexer::token::{LexerToken, LexerTokenType};

use crate::parser::parse_ast_node;

pub mod parser;
pub mod functions;
pub mod value;
pub mod math;
pub mod structs;
pub mod literals;
pub mod control;
pub mod variables;
pub mod types;
pub mod arrays;

pub fn parse_ast_ctx(tokens: &Vec<LexerToken>) -> CompilerResult<ParserCtx> {
	let mut ind = 0;

	let mut ctx = ParserCtx::new();

	while tokens[ind].tok_type != LexerTokenType::EndOfFile {
		let node = parse_ast_node(tokens, &mut ind)?;

		if !node.kind.is_tree_permissible() {
			return Err(tokens[ind - 1].make_err(format!(AST_INVALID_TREE!(), node), ErrorKind::Critical));
		}

		ctx.insert(node.kind.get_tree_name().unwrap().val, node); // might cause panic if tree name is null which SHOULD NOT happen.
	}

	return Ok(ctx);
}
