use astoir_hir::ctx::HIRContext;
use astoir_mir::ctx::MIRContext;

pub mod vars;
pub mod values;
pub mod math;
pub mod funcs;
pub mod body;

pub struct MIRLoweringContext {
	pub hir_ctx: HIRContext,
	pub mir_ctx: MIRContext
}
