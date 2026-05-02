use std::collections::HashMap;

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
    scope::{entry::ScopeEntry, key::EntryKey},
    types::ScopeStoredType,
};
use diagnostics::{DiagnosticResult, MaybeDiagnostic, builders::make_already_in_scope};
use typing::{
    TypeParameterContaining,
    constraints::TypeConstraintContainer,
    raw::RawType,
    structs::{StructContainer, StructuredField},
};

use crate::{
    funcs::lower_ast_function_declaration, types::lower_ast_type, values::lower_ast_value,
};

pub fn lower_ast_struct_init(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::StructInitializer { map } = node.kind.clone() {
        let mut new_map = HashMap::new();

        for entry in map {
            new_map.insert(entry.0, lower_ast_value(context, func_key, entry.1)?);
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::StructuredInit { fields: new_map },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node!")
}

pub fn lower_ast_struct_member(
    context: &mut HIRContext,
    node: Box<ASTTreeNode>,
    curr_t: RawType,
    container: &mut StructContainer,
) -> MaybeDiagnostic {
    if let ASTTreeNodeKind::StructFieldMember { name, member_type } = node.kind.clone() {
        let t = lower_ast_type(context, member_type, Some(curr_t), &*node)?;

        if !container
            .fields
            .insert(name.val.clone(), StructuredField::new(name.val.clone(), t))
        {
            return Err(make_already_in_scope(&*node, &name.val).into());
        }
    }

    panic!("Invalid node!")
}

pub fn lower_ast_struct_function(
    context: &mut HIRContext,
    node: Box<ASTTreeNode>,
    container: &mut StructContainer,
    t: RawType,
) -> MaybeDiagnostic {
    if let ASTTreeNodeKind::FunctionDeclaration {
        func_name,
        args,
        body: _,
        return_type,
        requires_this: _,
    } = node.kind.clone()
    {
        let mut arguments = vec![];

        for arg in args {
            let ty = lower_ast_type(context, arg.argument_type.clone(), Some(t.clone()), &*node)?;

            arguments.push(ty);
        }

        let ret_type;

        if return_type.is_some() {
            ret_type = Some(lower_ast_type(
                context,
                return_type.unwrap(),
                Some(t.clone()),
                &*node,
            )?);
        } else {
            ret_type = None;
        }

        container.append_function(func_name.val, ret_type, arguments);
        lower_ast_function_declaration(context, Some(t), node)?;
    }

    panic!("Invalid node!")
}

pub fn lower_ast_struct_declaration(
    context: &mut HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::StructLayoutDeclaration {
        name,
        layout: _, // TODO: add layouts back
        members,
        type_params,
    } = node.kind.clone()
    {
        let mut container = StructContainer::new(name.val.clone(), context.scope.entries.len());

        for type_param in type_params {
            container.append_type_parameter(type_param.0.val, TypeConstraintContainer::new()); // TODO: add constraint parsing
        }

        let mut ty = RawType::Struct(container.clone());

        // Register
        let key = EntryKey::new(name);
        let entry = ScopeEntry::new_type(ScopeStoredType {
            t: ty.clone(),
            function_implementations: vec![],
        });

        context.scope.append(key.clone(), entry, &*node)?;

        for member in members {
            match member.kind {
                ASTTreeNodeKind::FunctionDeclaration { .. } => {
                    lower_ast_struct_function(context, member, &mut container, ty.clone())?;
                }

                ASTTreeNodeKind::StructFieldMember { .. } => {
                    lower_ast_struct_member(context, member, ty.clone(), &mut container)?;
                }

                _ => panic!("Invalid node"),
            }

            ty = RawType::Struct(container.clone());
            context
                .scope
                .modify_type(&key, &*node, |f| f.t = ty.clone())?;
        }
    }

    panic!("Invalid node!")
}
