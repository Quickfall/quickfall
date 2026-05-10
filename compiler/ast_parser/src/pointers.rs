use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::value::{parse_ast_value, parse_ast_value_full};

pub fn parse_deref_modify(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    *ind += 1; // *

    let pointer = parse_ast_value_full(tokens, ind, false)?;

    tokens[*ind].expects(LexerTokenType::EqualSign)?;
    *ind += 1; // =

    let val = parse_ast_value(tokens, ind)?;

    let deref = Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::DereferenceModify {
            pointer,
            val: val.clone(),
        },
        val.start.clone(),
        val.end,
    ));

    Ok(deref)
}
