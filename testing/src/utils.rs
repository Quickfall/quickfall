use ast_parser::parse_ast_ctx;
use astoir::run_astoir_hir;
use compiler_errors::errs::{dump_errors, has_errors};
use lexer::lexer::lexer_parse_file;

pub fn run_test(path: String) -> bool {
	let lexer_result = lexer_parse_file(&path);

	if has_errors() {
		dump_errors();
		return false;
	}	

	let ast = parse_ast_ctx(&lexer_result.unwrap());

	if has_errors() {
		dump_errors();
		return false;
	}

	let hir_result = run_astoir_hir(ast.unwrap());
	
	if has_errors() {
		dump_errors();
		return false;
	}

	return true;
}