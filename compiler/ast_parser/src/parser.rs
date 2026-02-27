use ast::{make_node, tree::{ASTTreeNode, ASTTreeNodeKind}};
use compiler_errors::{UNEXPECTED_TOKEN, UNUSED_VAR_ACCESS, errs::{CompilerResult, ErrorKind}};
use compiler_utils::hash::{WithHash};
use lexer::token::{LexerToken, LexerTokenType};

use crate::{control::{for_loop::parse_for_loop, if_else::parse_if_statement, while_block::parse_while_block}, functions::{parse_function_call, parse_function_declaraction, returns::parse_function_return_statement, shadow::parse_shadow_function_declaration}, structs::parse_type_declaration, value::parse_ast_value_post_l, variables::{decl::parse_variable_declaration, static_decl::parse_static_variable_declaration}};

/// Parses an AST node outside of any other node.
/// 
/// # Examples
/// `parse_ast_node` is used to parse:
/// - Function declarations
/// - Struct declarations
/// - Layout declarations
pub fn parse_ast_node(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {
		LexerTokenType::Function => {
			return parse_function_declaraction(tokens, ind);
		},

		LexerTokenType::ShadowFunction => {
			return parse_shadow_function_declaration(tokens, ind);
		}

		LexerTokenType::Struct => {
			return parse_type_declaration(tokens, ind, false);
		},

		LexerTokenType::Static => {
			return parse_static_variable_declaration(tokens, ind);
		}

		LexerTokenType::Layout => {
			return parse_type_declaration(tokens, ind, true);
		},

		_ => {
			return Err(tokens[*ind].make_err(format!(UNEXPECTED_TOKEN!(), &tokens[*ind].tok_type), ErrorKind::Error));
		}
	}	
}

/// Parses an AST node inside of another compatible node (functions, control bodies)
pub fn parse_ast_node_in_body(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {

		LexerTokenType::Var => {
			return parse_variable_declaration(tokens, ind);
		},

		LexerTokenType::If => {
			return parse_if_statement(tokens, ind);
		},
		
		LexerTokenType::While => {
			return parse_while_block(tokens, ind);
		},

		LexerTokenType::For => {
			return parse_for_loop(tokens, ind);
		},
		
		LexerTokenType::Return  => {
			return parse_function_return_statement(tokens, ind);
		}

		LexerTokenType::KEYWORD(str, _) => {
			if tokens[*ind + 1].tok_type == LexerTokenType::ParenOpen {
				let call = parse_function_call(tokens, ind);
				return parse_ast_value_post_l(tokens, ind, call, true);
			}

			let n = Ok(make_node!(ASTTreeNodeKind::VariableReference(WithHash::new(str.clone())), &tokens[*ind], &tokens[*ind]));

			*ind += 1;

			let new =  parse_ast_value_post_l(tokens, ind, n, true)?;

			if new.kind.is_var_access() {
				return Err(tokens[*ind].make_err(UNUSED_VAR_ACCESS!().to_string(), ErrorKind::Warn));
			}

			return Ok(new);
		},

		_ => {
			return Err(tokens[*ind].make_err(format!(UNEXPECTED_TOKEN!(), tokens[*ind].tok_type), ErrorKind::Error));
		}
	}
}