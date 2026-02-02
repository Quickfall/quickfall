//!
//! The main AST part of the parser.
//! The AST parsing is responsible for putting tokens into structures such as functions and other stuff.
//! It is an extremely important step.
//! 
//! Indexes passed to parsing functions SHOULD be the "detected" token rather than the next one.
//! 

use std::fmt::Debug;

use lexer::token::LexerToken;
use utils::hash::WithHash;

use crate::{ParserError, ParserResult, ast::{func::{call::parse_function_call, decl::parse_function_declaraction}, literals::{parse_integer_literal, parse_string_literal}, tree::ASTTreeNode, var::decl::parse_variable_declaration}};

pub mod tree;
pub mod func;
pub mod var;
pub mod literals;
pub mod cond;

pub fn parse_ast_value_post_l(tokens: &Vec<LexerToken>, ind: &mut usize, original: ParserResult<Box<ASTTreeNode>>) -> ParserResult<Box<ASTTreeNode>> {
	return original;
}

pub fn parse_ast_value(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	match &tokens[*ind] {
		LexerToken::INT_LIT(_) => {
			let int = parse_integer_literal(tokens, ind);
			return parse_ast_value_post_l(tokens, ind, int);
		},

		LexerToken::STRING_LIT(_) => {
			let str = parse_string_literal(tokens, ind);
			return parse_ast_value_post_l(tokens, ind, str);
		},

		LexerToken::KEYWORD(str, _) => {
			if tokens[*ind + 1] == LexerToken::PAREN_OPEN {
				let call = parse_function_call(tokens, ind);
				return parse_ast_value_post_l(tokens, ind, call);
			}

			let n = Ok(Box::new(ASTTreeNode::RepresentsElement { elementName: WithHash::new(String::clone(str)) }));
			return parse_ast_value_post_l(tokens, ind, n);
		}

		_ => return Err(ParserError::new(String::from("Cannot be parsed as val!"), 0))
	}	
}

pub fn parse_ast_node(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	println!("Ind: {}, tok at: {:#?}", ind, tokens[*ind]);

	match &tokens[*ind] {
		LexerToken::FUNCTION => {
			return parse_function_declaraction(tokens, ind);
		}

		LexerToken::VAR => {
			return parse_variable_declaration(tokens, ind);
		}

		_ => {
			return Err(ParserError::new(format!("err: {:#?}", tokens[*ind]), 0));
		}

	}
}