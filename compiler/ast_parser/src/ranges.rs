use ast::{
    ranges::ASTRange,
    tree::{ASTTreeNode, ASTTreeNodeKind},
};
use compiler_utils::hash;
use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::value::parse_ast_value;

pub fn parse_value_range(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<ASTRange> {
    tokens[*ind].expects(LexerTokenType::ArrayOpen)?;
    *ind += 1;

    let min;
    let mut got_first_dot = false;

    if tokens[*ind].tok_type == LexerTokenType::Dot {
        min = Box::new(ASTTreeNode::new(
            ASTTreeNodeKind::IntegerLit {
                val: 0,
                hash: hash!("s64"),
            },
            tokens[*ind].pos.clone(),
            tokens[*ind].get_end_pos(),
        ));

        got_first_dot = true;

        *ind += 1;
    } else {
        min = parse_ast_value(tokens, ind)?;
    }

    tokens[*ind].expects(LexerTokenType::Dot)?;
    *ind += 1;

    if !got_first_dot {
        tokens[*ind].expects(LexerTokenType::Dot)?;
        *ind += 1;
    }

    let max = parse_ast_value(tokens, ind)?;

    tokens[*ind].expects(LexerTokenType::ArrayClose)?;

    *ind += 1;

    Ok(ASTRange { min, max })
}
