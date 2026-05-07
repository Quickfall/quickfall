use ast::{
    make_node,
    tree::{ASTTreeNode, ASTTreeNodeKind},
};
use compiler_utils::hash::HashedString;
use diagnostics::{
    DiagnosticResult,
    builders::{make_unexpected_simple_error, make_unused_variable},
};
use lexer::token::{LexerToken, LexerTokenType};

use crate::{
    control::{
        for_loop::parse_for_loop, if_else::parse_if_statement, while_block::parse_while_block,
    },
    functions::{
        parse_function_call, parse_function_declaraction, returns::parse_function_return_statement,
        shadow::parse_extern_function_definition,
    },
    pointers::parse_deref_modify,
    structs::{enums::parse_enum_declaration, parse_type_declaration},
    use_statements::parse_use_statement,
    value::parse_ast_value_post_l,
    variables::{decl::parse_variable_declaration, static_decl::parse_static_variable_declaration},
};

/// Parses an AST node outside of any other node.
///
/// # Examples
/// `parse_ast_node` is used to parse:
/// - Function declarations
/// - Struct declarations
/// - Layout declarations
pub fn parse_ast_node(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    match &tokens[*ind].tok_type {
        LexerTokenType::Function => {
            return parse_function_declaraction(tokens, ind, None);
        }

        LexerTokenType::ExternFunc => {
            return parse_extern_function_definition(tokens, ind);
        }

        LexerTokenType::Struct => {
            return parse_type_declaration(tokens, ind, false);
        }

        LexerTokenType::Static => {
            return parse_static_variable_declaration(tokens, ind);
        }

        LexerTokenType::Layout => {
            return parse_type_declaration(tokens, ind, true);
        }

        LexerTokenType::Enum => {
            return parse_enum_declaration(tokens, ind);
        }

        LexerTokenType::Use => {
            return parse_use_statement(tokens, ind);
        }

        _ => {
            return Err(make_unexpected_simple_error(&tokens[*ind], &tokens[*ind].tok_type).into());
        }
    }
}

/// Parses an AST node inside of another compatible node (functions, control bodies)
pub fn parse_ast_node_in_body(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Box<ASTTreeNode>> {
    match &tokens[*ind].tok_type {
        LexerTokenType::Var => {
            return parse_variable_declaration(tokens, ind, true);
        }

        LexerTokenType::If => {
            return parse_if_statement(tokens, ind);
        }

        LexerTokenType::While => {
            return parse_while_block(tokens, ind);
        }

        LexerTokenType::For => {
            return parse_for_loop(tokens, ind);
        }

        LexerTokenType::Asterisk => return parse_deref_modify(tokens, ind),

        LexerTokenType::Return => {
            return parse_function_return_statement(tokens, ind);
        }

        LexerTokenType::Keyword(str, _) => {
            if tokens[*ind + 1].tok_type == LexerTokenType::ParenOpen {
                let call = parse_function_call(tokens, ind);
                return parse_ast_value_post_l(tokens, ind, call, true);
            }

            let n = Ok(make_node!(
                ASTTreeNodeKind::VariableReference(HashedString::new(str.clone())),
                &tokens[*ind],
                &tokens[*ind]
            ));

            *ind += 1;

            let new = parse_ast_value_post_l(tokens, ind, n, true)?;

            if new.kind.is_var_access() {
                return Err(make_unused_variable(&*new, &"access".to_string()).into());
            }

            return Ok(new);
        }

        _ => {
            return Err(make_unexpected_simple_error(&tokens[*ind], &tokens[*ind].tok_type).into());
        }
    }
}
