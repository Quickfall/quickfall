use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    ctx::{HIRContext, branched::HIRBranchedContext},
    nodes::{HIRNode, HIRNodeKind},
};
use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin, diagnostic::Level, errors::MATH_OPERATION_ASSIGNS,
};

use crate::values::lower_ast_value;

pub fn lower_ast_math_operation(
    context: &mut HIRContext,
    curr_ctx: &mut HIRBranchedContext,
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
            return Err(node
                .make_simple_diagnostic(
                    MATH_OPERATION_ASSIGNS.0,
                    Level::Error,
                    MATH_OPERATION_ASSIGNS.1.to_string(),
                    None,
                    vec![],
                    vec!["consider assigning this to variable".to_string()],
                    vec!["add = at the end of the operator".to_string()],
                )
                .into());
        }

        let left = lower_ast_value(context, curr_ctx, lval)?;

        let right = Box::new(lower_ast_value(context, curr_ctx, rval)?.use_as(
            context,
            curr_ctx,
            left.get_node_type(context, curr_ctx).unwrap(),
            &*node,
            None,
        )?);

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

    panic!("Invalid node type")
}
