//! Utility crate to interact quickly with the different compiler tools

use ast::ctx::ParserCtx;
use ast_parser::parse_ast_ctx;
use astoir::IRLevel;
use compiler_errors::errs::CompilerResult;
use compiler_utils::debug;
use lexer::lexer::lexer_parse_file;

/// Runs the AST toolchain on the given file path
pub fn run_ast_toolchain(file_path: String) -> CompilerResult<ParserCtx> {
	debug!("Running AST toolchain...");

	let lexer_result = lexer_parse_file(&file_path)?;
	
	return parse_ast_ctx(&lexer_result);
}

pub fn run_astoir_toolchain(ctx: ParserCtx, level: IRLevel) {
	
}