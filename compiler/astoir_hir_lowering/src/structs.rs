use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    ctx::{HIRBranchedContext, HIRContext},
    nodes::{HIRNode, HIRNodeKind},
};
use compiler_global_scope::{entry::GlobalStorageEntryType, key::EntryKey};
use compiler_typing::{raw::RawType, structs::RawStructTypeContainer};
use compiler_utils::{hash::HashedString, utils::indexed::IndexStorage};
use diagnostics::{DiagnosticResult, builders::make_already_in_scope};

use crate::{lower_ast_body, types::lower_ast_type_struct, values::lower_ast_value};

fn lower_ast_struct_member(
    context: &mut HIRContext,
    node: Box<ASTTreeNode>,
    container: &mut RawStructTypeContainer,
) -> DiagnosticResult<bool> {
    if let ASTTreeNodeKind::StructFieldMember { name, member_type } = node.kind.clone() {
        let t = lower_ast_type_struct(context, member_type, container, &*node)?;

        container.fields.append(name.hash, t);
        return Ok(true);
    }

    panic!("Invalid node type")
}

fn lower_ast_struct_function_decl(
    context: &mut HIRContext,
    node: Box<ASTTreeNode>,
    container: &mut RawStructTypeContainer,
    ty: RawType,
) -> DiagnosticResult<(Box<HIRNode>, usize)> {
    if let ASTTreeNodeKind::FunctionDeclaration {
        func_name,
        args,
        body,
        return_type,
        requires_this,
    } = node.kind.clone()
    {
        let mut arguments = vec![];

        for arg in args {
            let lowered = lower_ast_type_struct(context, arg.argument_type, container, &*node)?;

            arguments.push((arg.name.hash, lowered));
        }

        let ret_type;

        if return_type.is_some() {
            let lowered = lower_ast_type_struct(context, return_type.unwrap(), container, &*node)?;

            ret_type = Some(lowered)
        } else {
            ret_type = None;
        }

        let mut curr_ctx = HIRBranchedContext::new();
        let body = lower_ast_body(context, &mut curr_ctx, body, true)?;

        let ind = container
            .functions
            .append(func_name.hash, (arguments.clone(), ret_type.clone()));

        let implementation = Box::new(HIRNode::new(
            HIRNodeKind::StructFunctionDeclaration {
                func_name: ind,
                arguments: arguments.clone(),
                return_type: ret_type.clone(),
                body,
                ctx: curr_ctx.clone(),
                requires_this,
            },
            &node.start,
            &node.end,
        ));

        let mut hasher = DefaultHasher::new();
        ty.hash(&mut hasher);

        let fnname = format!("{}$${}", hasher.finish(), func_name.hash);

        let ret_type2;
        let mut arguments2 = vec![];

        if let Some(v) = ret_type {
            ret_type2 = Some(v.as_resolved()) // TODO: This unsupports generics, maybe fix later
        } else {
            ret_type2 = None;
        }

        for arg in &arguments {
            arguments2.push((arg.0, arg.1.clone().as_resolved()));
        }

        context.global_scope.append_struct_function(
            EntryKey {
                name_hash: HashedString::new(fnname.clone()).hash,
            },
            (ret_type2, arguments2, fnname),
            implementation,
            curr_ctx,
            ty,
            &*node,
        )?;
    }

    panic!("Invalid node type")
}

pub fn lower_ast_struct_declaration(
    context: &mut HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::StructLayoutDeclaration {
        name,
        layout,
        members,
        type_params,
    } = node.kind.clone()
    {
        let mut container = RawStructTypeContainer {
            fields: IndexStorage::new(),
            functions: IndexStorage::new(),
            type_params,
            function_ids: vec![],
            self_ref: context.global_scope.scope.entries.len(),
        };

        let base = RawType::Struct(layout, container.clone());

        let ind = match context.global_scope.append(
            EntryKey {
                name_hash: name.hash,
            },
            GlobalStorageEntryType::Type(base),
            &*node,
        ) {
            Ok(v) => v,
            Err(_) => return Err(make_already_in_scope(&*node, &name.val).into()),
        };

        for member in members {
            match &member.kind {
                &ASTTreeNodeKind::StructFieldMember { .. } => {
                    lower_ast_struct_member(context, member, &mut container)?;

                    context.global_scope.scope.entries[ind].entry_type =
                        GlobalStorageEntryType::Type(RawType::Struct(layout, container.clone()));
                }
                &ASTTreeNodeKind::FunctionDeclaration { .. } => {
                    let body = lower_ast_struct_function_decl(
                        context,
                        member,
                        &mut container,
                        context.global_scope.scope.entries[ind].as_type_unsafe(),
                    )?;

                    container.function_ids.push(body.1);

                    context.global_scope.scope.entries[ind].entry_type =
                        GlobalStorageEntryType::Type(RawType::Struct(layout, container.clone()));
                }

                _ => panic!("Invalid node type"),
            };
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::StructDeclaration {
                type_name: ind,
                container,
                layout,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node type")
}

pub fn lower_ast_struct_initializer(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::StructInitializer { map } = node.kind.clone() {
        let mut new_map = HashMap::new();

        for (k, v) in map {
            new_map.insert(k, lower_ast_value(context, curr_ctx, v)?);
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::StructInitializer { fields: new_map },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node type")
}
