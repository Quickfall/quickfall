use ast_parser::parse_ast_ctx;
use astoir::run_astoir_mir;
use diagnostics::{clear_diagnostics, diagnostic::Diagnostic, get_diagnostics, has_diagnostics};
use lexer::lexer::lexer_parse;

pub fn check_for_file(url: String, text: String) -> Vec<Diagnostic> {
    clear_diagnostics();

    let lexer = lexer_parse(text, &url);
    if has_diagnostics() {
        return get_diagnostics();
    }

    let ast = parse_ast_ctx(&lexer.unwrap());
    if has_diagnostics() {
        return get_diagnostics();
    }

    let _ = run_astoir_mir(ast.unwrap());

    return get_diagnostics();
}
