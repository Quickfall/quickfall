use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::value::parse_ast_value;

pub fn parse_array_access(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
    original: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();

    *ind += 1;

    let index = parse_ast_value(tokens, ind)?;

    tokens[*ind].expects(LexerTokenType::ArrayClose)?;

    *ind += 1;

    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::ArrayIndexAccess {
            val: original,
            index,
        },
        start,
        tokens[*ind].get_end_pos(),
    )));
}
