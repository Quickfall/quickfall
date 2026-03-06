use ast::ctx::ParserCtx;
use astoir_hir::ctx::HIRContext;
use astoir_hir_lowering::lower_ast;
use compiler_errors::errs::CompilerResult;

pub enum IRLevel {
	HIR
}

pub fn run_astoir_hir(ctx: ParserCtx) -> CompilerResult<HIRContext> {
	return lower_ast(ctx);
}