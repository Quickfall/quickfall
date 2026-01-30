//!
//! The main AST part of the parser.
//! The AST parsing is responsible for putting tokens into structures such as functions and other stuff.
//! It is an extremely important step.
//! 
//! Indexes passed to parsing functions SHOULD be the "detected" token rather than the next one.
//! 

use std::fmt::Debug;

use lexer::token::LexerToken;

use crate::{ParserError, ParserResult, ast::{func::decl::parse_function_declaraction, literals::{parse_integer_literal, parse_string_literal}, tree::ASTTreeNode, var::decl::parse_variable_declaration}};

pub mod tree;
pub mod func;
pub mod var;
pub mod literals;

pub fn parse_ast_node(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	println!("Ind: {}, tok at: {:#?}", ind, tokens[*ind]);

	match &tokens[*ind] {
		LexerToken::FUNCTION => {
			return parse_function_declaraction(tokens, ind);
		}

		LexerToken::VAR => {
			return parse_variable_declaration(tokens, ind);
		}

		LexerToken::INT_LIT(_) => {
			return parse_integer_literal(tokens, ind);
		}

		LexerToken::STRING_LIT(_) => {
			return parse_string_literal(tokens, ind);
		}
		
		_ => {
			return Err(ParserError::new(format!("err: {:#?}", tokens[*ind]), 0));
		}

	}
}