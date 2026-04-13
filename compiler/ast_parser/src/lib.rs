//!
//! The parser for the Quickfall AST
//!

use ast::{ctx::ParserCtx, tree::ASTTreeNodeKind};
use diagnostics::{DiagnosticResult, builders::make_unexpected_simple_error};
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
pub mod unwraps;
pub mod use_statements;

pub fn parse_ast_ctx(tokens: &Vec<LexerToken>) -> DiagnosticResult<ParserCtx> {
	let mut ind = 0;

	let mut ctx = ParserCtx::new();

	while tokens[ind].tok_type != LexerTokenType::EndOfFile {
		let node = parse_ast_node(tokens, &mut ind)?;

		if let ASTTreeNodeKind::UseStatement { .. } = node.kind {
			ctx.uses.push(node);
			continue;
		}

		if !node.kind.is_tree_permissible() {
			return Err(make_unexpected_simple_error(&*node, &node).into())
		}

		ctx.insert(node.kind.get_tree_name().unwrap().val, node);
	}

	return Ok(ctx);
}
