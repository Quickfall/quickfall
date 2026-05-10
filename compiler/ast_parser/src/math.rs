use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin, diagnostic::Level, errors::MATH_OPERATION_ASSIGNS,
};
use lexer::token::LexerToken;

use crate::value::parse_ast_value;
use ast::{
    operators::parse_math_operator,
    tree::{ASTTreeNode, ASTTreeNodeKind},
};

pub fn parse_math_operation(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
    original: Box<ASTTreeNode>,
    restricts_to_assigns: bool,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let oper = parse_math_operator(tokens, ind)?;

    if !oper.assigns && restricts_to_assigns {
        return Err(tokens[*ind]
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

    let right_member = parse_ast_value(tokens, ind)?;

    let start = original.start.clone();
    let end = right_member.end.clone();

    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::MathResult {
            lval: original,
            rval: right_member,
            operator: oper,
        },
        start,
        end,
    )));
}
