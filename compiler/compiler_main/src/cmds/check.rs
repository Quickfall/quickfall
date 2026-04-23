use ast_parser::parse_ast_ctx;
use astoir::{run_astoir_hir, run_astoir_mir};
use lexer::lexer::lexer_parse_file;

use crate::{cli::IRLayer, quietlyquit_if_errors};

pub fn run_check(path: String, layer: IRLayer) {
    let lexer = lexer_parse_file(&path);
    quietlyquit_if_errors!();

    let ast = parse_ast_ctx(&lexer.unwrap());
    quietlyquit_if_errors!();

    match layer {
        IRLayer::HIR => {
            let _hir = run_astoir_hir(ast.unwrap());
            quietlyquit_if_errors!();
        }

        IRLayer::MIR => {
            let _mir = run_astoir_mir(ast.unwrap());
            quietlyquit_if_errors!();
        }
    }
}
