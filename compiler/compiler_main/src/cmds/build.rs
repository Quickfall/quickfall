use std::{fs, path::PathBuf};

use ast_parser::parse_ast_ctx;
use astoir::run_astoir_mir;
use lexer::lexer::lexer_parse_file;

use crate::quietlyquit_if_errors;

pub fn build_mir(path: String, out: PathBuf) {
    let lexer = lexer_parse_file(&path);
    quietlyquit_if_errors!();

    let ast = parse_ast_ctx(&lexer.unwrap());
    quietlyquit_if_errors!();

    let mir = run_astoir_mir(ast.unwrap());
    quietlyquit_if_errors!();

    fs::write(out, format!("{}", mir.unwrap())).unwrap();
}
