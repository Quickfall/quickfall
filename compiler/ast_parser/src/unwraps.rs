use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use compiler_utils::hash::HashedString;
use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::{types::parse_type, value::parse_ast_value};

pub fn parse_unwrap_condition(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();

    let unsafe_unwrap = tokens[*ind].tok_type == LexerTokenType::UnwrapUnsafe;
    *ind += 1;

    tokens[*ind].expects(LexerTokenType::AngelBracketOpen)?;
    *ind += 1;

    let original = parse_ast_value(tokens, ind)?;

    tokens[*ind].expects(LexerTokenType::Comma)?;
    *ind += 1;

    let new_type = parse_type(tokens, ind)?;

    let new_var: Option<HashedString>;
    if tokens[*ind].tok_type == LexerTokenType::AngelBracketClose {
        new_var = None;
    } else {
        tokens[*ind].expects(LexerTokenType::Comma)?;
        *ind += 1;

        let kwd = tokens[*ind].expects_keyword()?;

        new_var = Some(HashedString::new(kwd.0));

        *ind += 1;
        tokens[*ind].expects(LexerTokenType::AngelBracketClose)?;
    }

    *ind += 1;
    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::UnwrapCondition {
            original,
            target_type: new_type,
            unsafe_unwrap,
            target_var: new_var,
        },
        start,
        tokens[*ind].get_end_pos(),
    )));
}

pub fn parse_unwrap_value(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();

    let unsafe_unwrap = tokens[*ind].tok_type == LexerTokenType::UnwrapUnsafe;
    *ind += 1;

    tokens[*ind].expects(LexerTokenType::AngelBracketOpen)?;
    *ind += 1;

    let original = parse_ast_value(tokens, ind)?;

    tokens[*ind].expects(LexerTokenType::Comma)?;
    *ind += 1;

    let new_type = parse_type(tokens, ind)?;

    tokens[*ind].expects(LexerTokenType::AngelBracketClose)?;

    *ind += 1;
    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::UnwrapValue {
            original,
            target_type: new_type,
            unsafe_unwrap,
        },
        start,
        tokens[*ind].get_end_pos(),
    )));
}
