use ast_parser::parse_ast_ctx;
use astoir::run_astoir_mir;
use diagnostics::{diagnostic::Diagnostic, get_diagnostics, has_diagnostics};
use lexer::lexer::lexer_parse_file;

pub fn check_for_file(url: String) -> Vec<Diagnostic> {
    let lexer = lexer_parse_file(&url);
    if has_diagnostics() {
        return get_diagnostics();
    }

    let ast = parse_ast_ctx(&lexer.unwrap());
    if has_diagnostics() {
        return get_diagnostics();
    }

    run_astoir_mir(ast.unwrap());

    return get_diagnostics();
}
