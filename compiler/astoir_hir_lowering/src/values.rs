use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
    scope::key::EntryKey,
};
use diagnostics::DiagnosticResult;
use typing::raw::RawType;

use crate::{
    arrays::{lower_ast_array_index_access, lower_ast_array_init},
    booleans::{lower_ast_boolean_compare, lower_ast_boolean_condition},
    funcs::lower_ast_function_call,
    math::lower_ast_math_operation,
    structs::lower_ast_struct_init,
    unwraps::{lower_ast_unwrap_condition, lower_ast_unwrap_value},
    vars::lower_ast_variable_reference,
};

pub fn lower_ast_generic(
    _context: &mut HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::IntegerLit { val, hash: _ } = node.kind.clone() {
        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::IntegerLiteral(
                val,
                RawType::Integer(true, 128), // TODO: change hash to an actual type to allow for a non placeholder type here
            ),
            &node.start,
            &node.end,
        )));
    };

    if let ASTTreeNodeKind::StringLit(val) = node.kind.clone() {
        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::StringLiteral(val),
            &node.start,
            &node.end,
        )));
    };

    panic!("Invalid node")
}

pub fn lower_ast_value(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    match node.kind {
        ASTTreeNodeKind::StringLit(_) | ASTTreeNodeKind::IntegerLit { .. } => {
            return lower_ast_generic(context, node);
        }

        ASTTreeNodeKind::VariableReference(_) => {
            return lower_ast_variable_reference(context, func_key.unwrap(), node);
        }

        ASTTreeNodeKind::MathResult { .. } => {
            return lower_ast_math_operation(context, func_key, node, false);
        }

        ASTTreeNodeKind::BooleanBasedConditionMember { .. } => {
            return lower_ast_boolean_condition(context, func_key, node);
        }

        ASTTreeNodeKind::OperatorBasedConditionMember { .. } => {
            return lower_ast_boolean_compare(context, func_key, node);
        }

        ASTTreeNodeKind::ArrayIndexAccess { .. } => {
            return lower_ast_array_index_access(context, func_key, node);
        }

        ASTTreeNodeKind::ArrayVariableInitializerValue { .. }
        | ASTTreeNodeKind::ArrayVariableInitializerValueSameValue { .. } => {
            return lower_ast_array_init(context, func_key, node);
        }

        ASTTreeNodeKind::StructInitializer { .. } => {
            return lower_ast_struct_init(context, func_key, node);
        }

        ASTTreeNodeKind::PointerGrab(_) | ASTTreeNodeKind::ReferenceGrab(_) => {
            return lower_ast_grab(context, func_key, node);
        }

        ASTTreeNodeKind::UnwrapCondition { .. } => {
            return lower_ast_unwrap_condition(context, func_key, node);
        }

        ASTTreeNodeKind::UnwrapValue { .. } => {
            return lower_ast_unwrap_value(context, func_key, node);
        }

        ASTTreeNodeKind::FunctionCall { .. } => {
            return lower_ast_function_call(context, func_key, node);
        }

        _ => panic!("Invalid node"),
    }
}

pub fn lower_ast_grab(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::PointerGrab(inner) = node.kind.clone() {
        let inner = lower_ast_value(context, func_key, inner)?;

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::PointerGrab { val: inner },
            &node.start,
            &node.end,
        )));
    }

    if let ASTTreeNodeKind::ReferenceGrab(inner) = node.kind.clone() {
        let inner = lower_ast_value(context, func_key, inner)?;

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::ReferenceGrab { val: inner },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node!")
}
