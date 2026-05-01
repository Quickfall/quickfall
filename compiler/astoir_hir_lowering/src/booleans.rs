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

use crate::values::lower_ast_value;

pub fn lower_ast_boolean_condition(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::BooleanBasedConditionMember { val, negate } = node.kind.clone() {
        let val = lower_ast_value(context, func_key, val)?.use_as(
            context,
            func_key,
            Type::Raw {
                raw: InformationRawType::new(RawType::Boolean),
            },
            &*node,
        )?;

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::BooleanCondition {
                value: val,
                negation: negate,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node!")
}

pub fn lower_ast_boolean_compare(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::OperatorBasedConditionMember {
        lval,
        rval,
        operator,
    } = node.kind.clone()
    {
        let left_val = lower_ast_value(context, func_key, lval)?;
        let left_type = left_val.get_type(context, func_key, &*node)?.unwrap();

        let right_val = lower_ast_value(context, func_key, rval)?.use_as(
            context,
            func_key,
            left_type.clone(),
            &*node,
        )?;

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::BooleanOperator {
                left: left_val,
                right: right_val,
                operator,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node!")
}
