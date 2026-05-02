use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
    scope::key::EntryKey,
};
use diagnostics::{DiagnosticResult, builders::make_expected_simple_error};
use typing::{container::Type, raw::RawType};

use crate::{values::lower_ast_value, vars::lower_ast_variable_reference};

pub fn lower_ast_array_index_access(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::ArrayIndexAccess { val, index } = node.kind {
        let val = lower_ast_value(context, func_key, val)?;
        let val_type = val.get_type(context, func_key, &*val)?.unwrap();

        let index = lower_ast_value(context, func_key, index.clone())?.use_as(
            context,
            func_key,
            Type::make_raw(RawType::Integer(false, 64)),
            &*index,
        )?;

        if !val_type.is_array() {
            return Err(make_expected_simple_error(&*val, &"array".to_string(), &val_type).into());
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ArrayIndexAccess { val, index },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node!")
}

pub fn lower_ast_array_init(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::ArrayVariableInitializerValue { vals } = node.kind.clone() {
        let mut values = vec![];
        let first_type = lower_ast_value(context, func_key, vals[0].clone())?
            .get_type(context, func_key, &*vals[0])?
            .unwrap();

        for v in &vals {
            values.push(lower_ast_value(context, func_key, v.clone())?.use_as(
                context,
                func_key,
                first_type.clone(),
                &**v,
            )?)
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ArrayVariableInitValue { vals: values },
            &node.start,
            &node.end,
        )));
    }

    if let ASTTreeNodeKind::ArrayVariableInitializerValueSameValue { size, v } = node.kind.clone() {
        let val = lower_ast_value(context, func_key, v)?;

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ArrayVariableInitValueSame { size, val },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node!")
}

pub fn lower_ast_array_modify(
    context: &mut HIRContext,
    func_key: &EntryKey,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::ArrayIndexModifiy { array, index, val } = node.kind.clone() {
        let array = lower_ast_variable_reference(context, func_key, array)?;
        let index = lower_ast_value(context, Some(func_key), index)?;
        let val = lower_ast_value(context, Some(func_key), val)?;

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ArrayIndexModify {
                array,
                index,
                new_val: val,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node!")
}
