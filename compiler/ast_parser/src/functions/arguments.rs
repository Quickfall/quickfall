//! Module for parsing arguments

use ast::{tree::FunctionDeclarationArgument, types::ASTType};
use compiler_utils::hash::HashedString;
use diagnostics::{DiagnosticResult, builders::make_unexpected_simple_error};
use lexer::token::{LexerToken, LexerTokenType};

use crate::types::parse_type;

pub fn parse_function_arguments(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
    struct_type: Option<ASTType>,
) -> DiagnosticResult<(Vec<FunctionDeclarationArgument>, bool)> {
    *ind += 1;

    let mut depends_on_this: bool = false;
    let mut args: Vec<FunctionDeclarationArgument> = Vec::new();

    while *ind < tokens.len() {
        if tokens[*ind].tok_type == LexerTokenType::ParenClose {
            break;
        }

        if tokens[*ind].tok_type == LexerTokenType::This {
            if struct_type.is_none() || !args.is_empty() {
                return Err(
                    make_unexpected_simple_error(&tokens[*ind], &tokens[*ind].tok_type).into(),
                );
            }

            depends_on_this = true;

            *ind += 1;

            args.push(FunctionDeclarationArgument {
                name: HashedString::new("this".to_string()),
                argument_type: struct_type.clone().unwrap(),
            })
        } else {
            let var_type = parse_type(tokens, ind)?;

            let var_name = tokens[*ind].expects_keyword()?;

            args.push(FunctionDeclarationArgument::new(var_name.0, var_type));

            *ind += 1;
        }

        if tokens[*ind].tok_type == LexerTokenType::ParenClose {
            break;
        }

        tokens[*ind].expects(LexerTokenType::Comma)?;
        *ind += 1;
    }

    tokens[*ind].expects(LexerTokenType::ParenClose)?;

    Ok((args, depends_on_this))
}
