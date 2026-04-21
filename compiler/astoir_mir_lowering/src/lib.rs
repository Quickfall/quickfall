use std::collections::HashMap;

use astoir_hir::{
    ctx::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
};
use astoir_mir::{ctx::MIRContext, funcs::MIRFunction};
use compiler_typing::{
    SizedType, TypedGlobalScopeEntry, raw::RawType, structs::LoweredStructTypeContainer, tree::Type,
};
use compiler_utils::utils::indexed::IndexStorage;
use diagnostics::{DiagnosticResult, unsure_panic};

use crate::funcs::{lower_hir_function_decl, lower_hir_shadow_decl};

pub mod arrays;
pub mod body;
pub mod casts;
pub mod control;
pub mod funcs;
pub mod introductions;
pub mod lru;
pub mod math;
pub mod type_tools;
pub mod values;
pub mod vars;

pub struct MIRLoweringContext {
    pub hir_ctx: HIRContext,
    pub mir_ctx: MIRContext,
    pub block_introduction_var_queue: Vec<Box<HIRNode>>,
}

pub fn lower_hir_top_level(
    node: Box<HIRNode>,
    ctx: &mut MIRLoweringContext,
) -> DiagnosticResult<bool> {
    return match node.kind {
        HIRNodeKind::FunctionDeclaration { .. } => lower_hir_function_decl(node, ctx),
        HIRNodeKind::ShadowFunctionDeclaration { .. } => lower_hir_shadow_decl(node, ctx),
        HIRNodeKind::StructDeclaration { .. } => {
            // Since Struct declarations are already fulled lowered in HIR, we do need handling here!

            return Ok(true);
        }

        _ => panic!("Invalid tree"),
    };
}

pub fn lower_hir(ctx: HIRContext) -> DiagnosticResult<MIRContext> {
    let mut lowering_ctx = MIRLoweringContext {
        hir_ctx: ctx,
        mir_ctx: MIRContext::new(),
        block_introduction_var_queue: vec![],
    };

    for entry in lowering_ctx.hir_ctx.global_scope.scope.entries.clone() {
        match entry.entry_type {
            TypedGlobalScopeEntry::Function {
                descriptor_ind: _,
                impl_ind,
            } => {
                let node = lowering_ctx.hir_ctx.global_scope.implementations[impl_ind]
                    .1
                    .clone();

                lower_hir_top_level(node, &mut lowering_ctx)?;
            }

            TypedGlobalScopeEntry::ImplLessFunction(descriptor_ind) => {
                let descriptor =
                    lowering_ctx.hir_ctx.global_scope.descriptors[descriptor_ind].clone();

                let name = descriptor.2.clone();

                let mut args = vec![];

                for argument in descriptor.1 {
                    args.push(lower_hir_type(&mut lowering_ctx, argument.1)?);
                }

                let ret_type;

                if descriptor.0.is_some() {
                    ret_type = Some(lower_hir_type(
                        &mut lowering_ctx,
                        descriptor.0.clone().unwrap(),
                    )?)
                } else {
                    ret_type = None
                }

                let func = MIRFunction::new(name, args, ret_type, false, &mut lowering_ctx.mir_ctx);

                lowering_ctx.mir_ctx.append_function(func);
            }

            _ => todo!("Add support for remaining nodes"),
        };
    }

    return Ok(lowering_ctx.mir_ctx);
}

pub fn lower_hir_generic(
    ctx: &MIRLoweringContext,
    t: &Type,
    generic: &RawType,
) -> DiagnosticResult<Type> {
    match generic {
        RawType::Struct(a, b) => {
            let mut lowered_container = LoweredStructTypeContainer {
                fields: IndexStorage::new(),
                functions: IndexStorage::new(),
                is_lowered_enum_child: false,
                is_lowered_enum_parent: false,
                lowered_enum_child: None,
                lowered_enum_parent: None,
                hir_mir_indexes: HashMap::new(),
            };

            for field in &b.fields.vals {
                lowered_container
                    .fields
                    .vals
                    .push(lower_hir_type(ctx, field.clone().resolve(t))?);
            }

            return Ok(Type::GenericLowered(RawType::LoweredStruct(
                *a,
                lowered_container,
            )));
        }

        RawType::EnumEntry(container) => {
            let mut lowered_container = LoweredStructTypeContainer {
                fields: IndexStorage::new(),
                functions: IndexStorage::new(),
                is_lowered_enum_child: true,
                is_lowered_enum_parent: false,
                lowered_enum_child: Some(container.clone()),
                lowered_enum_parent: None,
                hir_mir_indexes: HashMap::new(),
            };

            let parent =
                match &ctx.hir_ctx.global_scope.scope.entries[container.parent].as_type_unsafe() {
                    RawType::Enum(container) => container.clone(),
                    _ => panic!("Enum parent not enum"),
                };

            lowered_container
                .fields
                .vals
                .push(Type::GenericLowered(parent.get_hint_type())); // Enum entry hint

            let mut ind = 0;
            for field in &container.fields.vals {
                lowered_container.append_hir_index_conv(ind, lowered_container.fields.vals.len()); // Allow for LRU to work correctly

                lowered_container
                    .fields
                    .vals
                    .push(lower_hir_type(ctx, field.clone().resolve(t))?);

                ind += 1;
            }

            return Ok(Type::GenericLowered(RawType::LoweredStruct(
                false,
                lowered_container,
            )));
        }

        RawType::Enum(container) => {
            let mut lowered_container = LoweredStructTypeContainer {
                fields: IndexStorage::new(),
                functions: IndexStorage::new(),
                is_lowered_enum_child: false,
                is_lowered_enum_parent: true,
                lowered_enum_parent: Some(container.clone()),
                lowered_enum_child: None,
                hir_mir_indexes: HashMap::new(),
            };

            let mut entry_size = 0;

            let info = t.get_generic_info();

            for entry in &container.entries {
                let lowered = lower_hir_type(
                    ctx,
                    Type::Generic(entry.1.clone(), info.0.clone(), info.1.clone()),
                )?;

                entry_size = entry_size.max(lowered.get_generic().get_size(
                    &lowered,
                    false,
                    &ctx.hir_ctx.global_scope.scope,
                ))
            }

            lowered_container
                .fields
                .vals
                .push(Type::GenericLowered(container.get_hint_type()));

            lowered_container
                .fields
                .vals
                .push(Type::GenericLowered(RawType::Integer(entry_size, false)));

            return Ok(Type::GenericLowered(RawType::LoweredStruct(
                false,
                lowered_container,
            )));
        }

        _ => return Ok(Type::GenericLowered(generic.clone())),
    };
}

pub fn lower_hir_type(ctx: &MIRLoweringContext, t: Type) -> DiagnosticResult<Type> {
    match &t {
        Type::Generic(a, _, _) => return lower_hir_generic(ctx, &t, a),

        Type::Array(a, b) => {
            return Ok(Type::Array(*a, Box::new(lower_hir_type(ctx, *b.clone())?)));
        }
        Type::Pointer(a, b) => {
            return Ok(Type::Pointer(
                *a,
                Box::new(lower_hir_type(ctx, *b.clone())?),
            ));
        }
        Type::Reference(inner) => {
            return Ok(Type::Reference(Box::new(lower_hir_type(
                ctx,
                *inner.clone(),
            )?)));
        }

        _ => unsure_panic!("type is already lowered"),
    }
}
