use diagnostics::DiagnosticResult;
use lexer::token::LexerToken;

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};

use crate::{control::if_else::parse_condition_member, functions::parse_node_body};

pub fn parse_while_block(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();

    *ind += 1;

    let cond = parse_condition_member(tokens, ind)?;

    tokens[*ind].expects(lexer::token::LexerTokenType::BracketOpen)?;

    let body = match parse_node_body(tokens, ind) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let end = tokens[*ind - 1].get_end_pos().clone();

    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::WhileBlock { cond, body },
        start,
        end,
    )));
}
