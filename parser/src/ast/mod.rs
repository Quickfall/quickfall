//!
//! The main AST part of the parser.
//! The AST parsing is responsible for putting tokens into structures such as functions and other stuff.
//! It is an extremely important step.
//! 
//! Indexes passed to parsing functions SHOULD be the "detected" token rather than the next one.
//! 

use lexer::token::LexerToken;

use crate::{ParserError, ParserResult, ast::{func::decl::parse_function_declaraction, tree::ASTTreeNode}};

pub mod tree;
pub mod func;

pub fn parse_ast_node(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	match &tokens[*ind] {
		LexerToken::FUNCTION => {
			return parse_function_declaraction(tokens, ind);
		}

		_ => {
			return Err(ParserError::new(String::from("Unkown token type!"), 0));
		}

	}
}