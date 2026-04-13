use std::path::{MAIN_SEPARATOR_STR};

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use ast_parser::parse_ast_ctx;
use astoir_hir::{ctx::HIRContext};
use diagnostics::{MaybeDiagnostic, builders::make_use_not_found};
use lexer::lexer::lexer_parse_file;

use crate::{lower_ast_toplevel, types::lower_ast_type};

pub fn handle_ast_use_statement_function_decl(context: &mut HIRContext, node: Box<ASTTreeNode>) -> MaybeDiagnostic {
	if let ASTTreeNodeKind::FunctionDeclaration { func_name, args, body: _, return_type, requires_this: _ } = node.kind.clone() {
		let ret_type;

		if return_type.is_some() {
			let lower = lower_ast_type(context, return_type.unwrap(), &*node)?;
			
			ret_type = Some(lower)
		} else {
			ret_type = None;
		}

		let mut arguments = vec![];
		let mut types = vec![];

		for arg in args {
			types.push(arg.argument_type.clone());
			let t = lower_ast_type(context, arg.argument_type, &*node)?;

			arguments.push((arg.name.hash, t));
		}

		context.functions.append(func_name.hash, (ret_type.clone(), arguments.clone(), func_name.val.clone()));
		context.function_contexts.push(None);

		return Ok(())
	}

	panic!("Invalid node")
}

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

			let n = ast.map[&clause.val].clone();

			match n.kind {
				ASTTreeNodeKind::FunctionDeclaration { .. } => {
					handle_ast_use_statement_function_decl(context, n.clone())?;
				} 
				_ => {
					println!(" --> {:#?}", n);
					lower_ast_toplevel(context, n.clone())?;
				}
			};
		}

		return Ok(())
	}

	panic!("Invalid node")
}