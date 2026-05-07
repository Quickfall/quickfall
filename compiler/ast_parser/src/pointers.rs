use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use diagnostics::DiagnosticResult;
use lexer::token::LexerToken;

use crate::value::{parse_ast_value_full, parse_ast_value_post_l};

pub fn parse_deref_modify(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    *ind += 1; // *

    let val = parse_ast_value_full(tokens, ind, false)?;

    let deref = Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::DereferenceModify {
            pointer: val.clone(),
        },
        val.start.clone(),
        val.end,
    ));

    parse_ast_value_post_l(tokens, ind, Ok(deref), false)
}
