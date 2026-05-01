use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
    scope::key::EntryKey,
};
use diagnostics::DiagnosticResult;
use typing::{
    container::Type,
    raw::{InformationRawType, RawType},
};

use crate::{
    booleans::{lower_ast_boolean_compare, lower_ast_boolean_condition},
    math::lower_ast_math_operation,
    vars::lower_ast_variable_reference,
};

pub fn lower_ast_generic(
    context: &mut HIRContext,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::IntegerLit { val, hash } = node.kind.clone() {
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

        _ => panic!("Invalid node"),
    }
}
