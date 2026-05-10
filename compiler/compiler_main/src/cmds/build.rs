use std::{fs, path::PathBuf};

use ast_parser::parse_ast_ctx;
use astoir::run_astoir_mir;
use astoir_mir::fmt::DisplayWithCtx;
use lexer::lexer::lexer_parse_file;

#[cfg(feature = "llvm")]
use llvm_ir_bridge::bridge_llvm;

use crate::quietlyquit_if_errors;

pub fn build_mir(path: String, out: PathBuf) {
    let lexer = lexer_parse_file(&path);
    quietlyquit_if_errors!();

    let ast = parse_ast_ctx(&lexer.unwrap());
    quietlyquit_if_errors!();

    let mir = run_astoir_mir(ast.unwrap());
    quietlyquit_if_errors!();

    let mir = mir.unwrap();

    fs::write(out, format!("{}", DisplayWithCtx::new(&mir, &mir))).unwrap();
}

#[cfg(feature = "llvm")]
pub fn build_llvm(path: String, out: PathBuf) {
    let lexer = lexer_parse_file(&path);
    quietlyquit_if_errors!();

    let ast = parse_ast_ctx(&lexer.unwrap());
    quietlyquit_if_errors!();

    let mir = run_astoir_mir(ast.unwrap());
    quietlyquit_if_errors!();

    let llvm = bridge_llvm(&mir.unwrap());

    llvm.module.print_to_file(out).unwrap();
}

#[cfg(not(feature = "llvm"))]
pub fn build_llvm(_path: String, _out: PathBuf) {
    println!("LLVM bridge is disabled in this version of the compiler.")
}
