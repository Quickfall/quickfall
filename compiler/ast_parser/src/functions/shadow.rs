//! Shadow function parsing

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use compiler_utils::hash::HashedString;
use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::functions::{arguments::parse_function_arguments, parse_function_return_type};

pub fn parse_extern_function_definition(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    let start = tokens[*ind].pos.clone();

    *ind += 1;
    let function_name = tokens[*ind].expects_keyword()?;

    *ind += 1;
    tokens[*ind].expects(LexerTokenType::ParenOpen)?;

    let args = parse_function_arguments(tokens, ind, None)?;

    *ind += 1;

    let ret_type = parse_function_return_type(tokens, ind)?;
    let end = tokens[*ind].get_end_pos();

    return Ok(Box::new(ASTTreeNode::new(
        ASTTreeNodeKind::ExternFunctionDeclaration {
            func_name: HashedString::new(function_name.0),
            args: args.0,
            return_type: ret_type,
        },
        start,
        end,
    )));
}
