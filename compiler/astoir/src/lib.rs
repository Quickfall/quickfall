use ast::ctx::ParserCtx;
use astoir_hir::ctx::HIRContext;
use astoir_hir_lowering::lower_ast;
use compiler_errors::errs::{CompilerResult, dump_errors};

pub enum IRLevel {
	HIR
}

pub fn run_astoir_hir(ctx: ParserCtx) -> CompilerResult<HIRContext> {
	let r = lower_ast(ctx);

	dump_errors();
	return r;
}
