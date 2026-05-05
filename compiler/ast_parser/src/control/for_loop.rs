use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};

use crate::{
    functions::parse_node_body, parser::parse_ast_node_in_body, ranges::parse_value_range,
    value::parse_ast_value, variables::decl::parse_variable_declaration,
};

pub fn parse_for_loop(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();

    if tokens[*ind + 1].tok_type != LexerTokenType::ParenOpen {
        return parse_for_ranged_loop(tokens, ind);
    }

    *ind += 1;

    tokens[*ind].expects(LexerTokenType::ParenOpen)?;
    *ind += 1;

    tokens[*ind].expects(LexerTokenType::Var)?;

    let initial = parse_variable_declaration(tokens, ind)?;

    tokens[*ind].expects(LexerTokenType::Comma)?;

    *ind += 1;
    let cond = parse_ast_value(tokens, ind)?;

    tokens[*ind].expects(LexerTokenType::Comma)?;
    *ind += 1;

    let increment = parse_ast_node_in_body(tokens, ind)?;

    tokens[*ind].expects(LexerTokenType::ParenClose)?;
    *ind += 1;

    tokens[*ind].expects(LexerTokenType::BracketOpen)?;

    let body = parse_node_body(tokens, ind)?;

    let end = tokens[*ind - 1].get_end_pos();

    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::ForBlock {
            initial_state: initial,
            cond,
            increment,
            body,
        },
        start,
        end,
    )));
}

pub fn parse_for_ranged_loop(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();

    let var = parse_variable_declaration(tokens, ind)?;

    tokens[*ind].expects(LexerTokenType::EqualSign)?;
    *ind += 1;

    tokens[*ind].expects(LexerTokenType::AngelBracketClose)?;
    *ind += 1;

    let range = parse_value_range(tokens, ind)?;

    tokens[*ind].expects(LexerTokenType::BracketOpen)?;
    *ind += 1;

    let body = parse_node_body(tokens, ind)?;

    let end = tokens[*ind - 1].get_end_pos();

    Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::RangedForBlock { var, range, body },
        start,
        end,
    )))
}
