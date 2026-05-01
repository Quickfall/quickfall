use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::HIRContext,
    func,
    nodes::{HIRNode, HIRNodeKind},
    scope::key::EntryKey,
};
use diagnostics::{
    DiagnosticResult,
    builders::{make_expected_simple_error, make_math_operation_req_assign},
};

use crate::values::lower_ast_value;

pub fn lower_ast_math_operation(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
    enforce_assign: bool,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::MathResult {
        lval,
        rval,
        operator,
    } = node.kind.clone()
    {
        if enforce_assign && !operator.assigns {
            return Err(make_math_operation_req_assign(&*node).into());
        }

        let left = lower_ast_value(context, func_key, lval)?;
        let left_type = left.get_type(context, func_key, &*node)?.unwrap();

        let right = lower_ast_value(context, func_key, rval)?.use_as(
            context,
            func_key,
            left_type.clone(),
            &*node,
        )?;

        if !left_type.get_raw().t.has_math_operations() {
            return Err(make_expected_simple_error(
                &*node,
                &"math operation compatible type".to_string(),
                &left_type,
            )
            .into());
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::MathOperation {
                left,
                right,
                operation: operator,
            },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node")
}
