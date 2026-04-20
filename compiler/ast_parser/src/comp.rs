use ast::{
    operators::parse_compare_operator,
    tree::{ASTTreeNode, ASTTreeNodeKind},
};
use diagnostics::DiagnosticResult;
use lexer::token::LexerToken;

use crate::value::parse_ast_value;

pub fn parse_ast_compare(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
    original: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let operator = parse_compare_operator(tokens, ind)?;

    let right_val = parse_ast_value(tokens, ind)?;

    let start_pos = original.start.clone();
    let end_pos = right_val.end.clone();

    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::OperatorBasedConditionMember {
            lval: original,
            rval: right_val,
            operator,
        },
        start_pos,
        end_pos,
    )));
}
