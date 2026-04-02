use ast::ctx::ParserCtx;
use astoir_hir::ctx::HIRContext;
use astoir_hir_lowering::lower_ast;
use astoir_mir::ctx::MIRContext;
use astoir_mir_lowering::lower_hir;
use diagnostics::DiagnosticResult;

pub enum IRLevel {
	HIR,
	MIR,
	LLVM
}

pub fn run_astoir_hir(ctx: ParserCtx) -> DiagnosticResult<HIRContext> {
	return lower_ast(ctx);
}

pub fn run_astoir_mir(ctx: ParserCtx) -> DiagnosticResult<MIRContext> {
	return lower_hir(run_astoir_hir(ctx)?);
}