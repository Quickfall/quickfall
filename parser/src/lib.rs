//!
//! The parsing module of Quickfall. Contains all of the parsing code required for the Quickfall language.
//! 
//! # Introduction
//! The `parser` module mostly contains the AST processor for Quickfall. Every element of the language is represented as an AST node which is then passed onto the AST tree. 
//! The AST tree is then sent to the IR writer to actually compile.

use errors::{AST_INVALID_TREE, errs::{CompilerResult, ErrorKind}};
use lexer::token::{LexerToken, LexerTokenType};

use crate::{ast::parse_ast_node, ctx::ParserCtx};

pub mod ast;
pub mod ctx;

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