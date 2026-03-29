use ast::ctx::ParserCtx;
use astoir_hir::ctx::HIRContext;
use astoir_hir_lowering::lower_ast;
use astoir_mir::ctx::MIRContext;
use astoir_mir_lowering::lower_hir;
use compiler_errors::errs::{CompilerResult, normal::CompilerError};

pub enum IRLevel {
	HIR,
	MIR,
	LLVM
}

pub fn run_astoir_hir(ctx: ParserCtx) -> CompilerResult<HIRContext> {
	return lower_ast(ctx);
}

pub fn run_astoir_mir(ctx: ParserCtx) -> CompilerResult<MIRContext> {
	return match lower_hir(run_astoir_hir(ctx)?) {
		Ok(v) => Ok(v),
		Err(e) => Err(CompilerError::from_base_posless(e))
	}
}