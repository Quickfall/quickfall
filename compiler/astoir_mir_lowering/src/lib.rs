use astoir_hir::{ctx::HIRContext, nodes::HIRNode};
use astoir_mir::ctx::MIRContext;
use compiler_errors::{AST_INVALID_TREE, errs::{BaseResult, IS_MIR_STAGE, base::BaseError}};
use compiler_typing::{raw::RawType, structs::LoweredStructTypeContainer, tree::Type};
use compiler_utils::utils::indexed::IndexStorage;

use crate::funcs::{lower_hir_function_decl, lower_hir_shadow_decl};

pub mod vars;
pub mod values;
pub mod math;
pub mod funcs;
pub mod body;
pub mod control;
pub mod arrays;

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

pub fn lower_hir(ctx: HIRContext) -> BaseResult<MIRContext> {
	IS_MIR_STAGE.with_borrow_mut(|e| *e = true);
	 
	let mut lowering_ctx = MIRLoweringContext { hir_ctx: ctx, mir_ctx: MIRContext::new() };

	let declarations = lowering_ctx.hir_ctx.function_declarations.clone();

	for decl in declarations {
		if let Some(node) = decl {
			lower_hir_top_level(node, &mut lowering_ctx)?;
		}
	}

	return Ok(lowering_ctx.mir_ctx);
}

pub fn lower_hir_generic(ctx: &MIRLoweringContext, t: &Type, generic: &RawType) -> BaseResult<Type> {
	match generic {
		RawType::Struct(a, b) => {
			let mut lowered_container = LoweredStructTypeContainer { fields: IndexStorage::new(), functions: IndexStorage::new() };

			for field in &b.fields.vals {
				lowered_container.fields.vals.push(lower_hir_type(ctx, field.clone().resolve(t))?);
			}

			return Ok(Type::GenericLowered(RawType::LoweredStruct(*a, lowered_container)));
		},

		_ => return Ok(Type::GenericLowered(generic.clone()))
	};
}

pub fn lower_hir_type(ctx: &MIRLoweringContext, t: Type) -> BaseResult<Type> {
	match &t {
		Type::Generic(a, _, _) => {
			return lower_hir_generic(ctx, &t, &ctx.hir_ctx.type_storage.types.vals[*a])
		},

		Type::Array(a, b) => return Ok(Type::Array(*a, Box::new(lower_hir_type(ctx, *b.clone())?))), 
		Type::Pointer(a, b) => return Ok(Type::Pointer(*a, Box::new(lower_hir_type(ctx, *b.clone())?))),

		_ => return Err(BaseError::err("Type is already lowered!".to_string()))
	}
}