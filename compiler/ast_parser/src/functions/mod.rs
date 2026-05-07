//! Parser module for functions

use ast::{
    tree::{ASTTreeNode, ASTTreeNodeKind},
    types::ASTType,
};
use compiler_utils::hash::HashedString;
use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::{
    functions::arguments::parse_function_arguments, parser::parse_ast_node_in_body,
    types::parse_type, value::parse_ast_value,
};

pub mod arguments;
pub mod returns;
pub mod shadow;

pub fn parse_function_return_type(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Option<ASTType>> {
    if tokens[*ind].tok_type == LexerTokenType::Minus {
        *ind += 1;

        tokens[*ind].expects(LexerTokenType::AngelBracketClose)?;
        *ind += 1;

        let ty = parse_type(tokens, ind)?;

        return Ok(Some(ty));
    } else {
        return Ok(None);
    }
}

pub fn parse_function_declaraction(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
    struct_type: Option<ASTType>,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();

    *ind += 1;
    let function_name = tokens[*ind].expects_keyword()?;

    *ind += 1;
    tokens[*ind].expects(LexerTokenType::ParenOpen)?;

    let args = parse_function_arguments(tokens, ind, struct_type)?;

    *ind += 1;

    let ret_type = parse_function_return_type(tokens, ind)?;

    tokens[*ind].expects(LexerTokenType::BracketOpen)?;

    let body = parse_node_body(tokens, ind)?;

    let end = tokens[*ind - 1].get_end_pos();

    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::FunctionDeclaration {
            func_name: HashedString::new(function_name.0),
            args: args.0,
            body,
            return_type: ret_type,
            requires_this: args.1,
        },
        start,
        end,
    )));
}

pub fn parse_function_call(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();

    let func = HashedString::new(tokens[*ind].expects_keyword()?.0);

    *ind += 1;

    tokens[*ind].expects(LexerTokenType::ParenOpen)?;

    *ind += 1;

    let mut vals: Vec<Box<ASTTreeNode>> = Vec::new();

    while tokens[*ind].tok_type != LexerTokenType::ParenClose {
        vals.push(parse_ast_value(tokens, ind)?);

        if tokens[*ind].tok_type == LexerTokenType::ParenClose {
            break;
        }

        tokens[*ind].expects(LexerTokenType::Comma)?;

        *ind += 1;
    }

    let end = tokens[*ind].get_end_pos().clone();

    *ind += 1;

    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::FunctionCall { func, args: vals },
        start,
        end,
    )));
}

pub fn parse_node_body(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Vec<Box<ASTTreeNode>>> {
    *ind += 1;

    let mut tok: &LexerToken = &tokens[*ind];
    let mut body: Vec<Box<ASTTreeNode>> = Vec::new();

    while tok.tok_type != LexerTokenType::EndOfFile && tok.tok_type != LexerTokenType::BracketClose
    {
        if tok.tok_type == LexerTokenType::SemiCollon {
            *ind += 1;
            tok = &tokens[*ind];

            continue;
        }

        let n = parse_ast_node_in_body(tokens, ind)?;

        body.push(n);

        tok = &tokens[*ind];
    }

    *ind += 1;

    return Ok(body);
}
