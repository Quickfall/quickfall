use std::path::{MAIN_SEPARATOR_STR};

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use ast_parser::parse_ast_ctx;
use astoir_hir::{ctx::HIRContext};
use diagnostics::{MaybeDiagnostic, builders::make_use_not_found};
use lexer::lexer::lexer_parse_file;

use crate::{lower_ast_toplevel};

pub fn handle_ast_use_statement(context: &mut HIRContext, node: Box<ASTTreeNode>) -> MaybeDiagnostic {
	if let ASTTreeNodeKind::UseStatement { shards, use_clauses } = node.kind.clone() {
		let mut path = ".".to_string();

		for shard in shards {
			path += &MAIN_SEPARATOR_STR .to_owned();
			path += &shard.val;
		}

		path += &".qf";

		let lexer = lexer_parse_file(&path)?;
		let ast = parse_ast_ctx(&lexer)?;

		for clause in use_clauses {
			if !ast.map.contains_key(&clause.val) {
				return Err(make_use_not_found(&*node, &clause.val, &path).into())
			}

			lower_ast_toplevel(context, ast.map[&clause.val].clone())?;
		}

		return Ok(())
	}

	panic!("Invalid node")
}