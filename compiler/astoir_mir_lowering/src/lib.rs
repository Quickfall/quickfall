use astoir_hir::{ctx::HIRContext, nodes::HIRNode};
use astoir_mir::ctx::MIRContext;
use compiler_errors::{AST_INVALID_TREE, errs::{BaseResult, base::BaseError}};

use crate::funcs::{lower_hir_function_decl, lower_hir_shadow_decl};

pub mod vars;
pub mod values;
pub mod math;
pub mod funcs;
pub mod body;
pub mod control;

pub struct MIRLoweringContext {
	pub hir_ctx: HIRContext,
	pub mir_ctx: MIRContext
}

pub fn lower_hir_top_level(node: Box<HIRNode>, ctx: &mut MIRLoweringContext) -> BaseResult<bool> {
	return match *node {
		HIRNode::FunctionDeclaration { .. } => lower_hir_function_decl(node, ctx),
		HIRNode::ShadowFunctionDeclaration { .. } => lower_hir_shadow_decl(node, ctx),
		HIRNode::StructDeclaration { .. } => {
			// Since Struct declarations are already fulled lowered in HIR, we do need handling here!

			return Ok(true);
		},

		_ => return Err(BaseError::err(AST_INVALID_TREE!().to_string()))
	}
}