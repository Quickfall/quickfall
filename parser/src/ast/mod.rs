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

use crate::{ParserError, ParserResult, ast::{cond::operators::parse_condition_operator, control::{ifelse::parse_if_statement, whileblock::parse_while_block}, func::{call::parse_function_call, decl::parse_function_declaraction}, literals::{parse_integer_literal, parse_string_literal}, tree::ASTTreeNode, var::decl::parse_variable_declaration}};

pub mod tree;
pub mod func;
pub mod var;
pub mod literals;
pub mod cond;
pub mod control;

pub fn parse_ast_value_post_l(tokens: &Vec<LexerToken>, ind: &mut usize, original: ParserResult<Box<ASTTreeNode>>) -> ParserResult<Box<ASTTreeNode>> {
	match &tokens[*ind] {
		LexerToken::DOT => {
			let o = &original?;
			let k = Box::new(ASTTreeNode::clone(o.as_ref()));

			if !o.is_function_call() && !o.is_var_access() {
				return Err(ParserError::new(String::from("Tried using field/func access on non-value element!"), 0));
			}

			*ind += 1;
			let r = parse_ast_value(tokens, ind)?;

			if r.is_function_call() {
				return Ok(Box::new(ASTTreeNode::StructLRFunction { l: k, r }))
			} else if r.is_var_access() {
				return Ok(Box::new(ASTTreeNode::StructLRVariable { l: k, r }))
			}

			return Err(ParserError::new(String::from("Next member isn't any valid field/func access type!"), 0));
		},

		LexerToken::ANGEL_BRACKET_CLOSE | LexerToken::EQUAL_SIGN | LexerToken::ANGEL_BRACKET_OPEN => {
			let operator = parse_condition_operator(tokens, ind)?;

			let o = &original?;
			let k = Box::new(ASTTreeNode::clone(o.as_ref()));

			*ind += 1;
			let right_val = parse_ast_value(tokens, ind)?;

			return Ok(Box::new(ASTTreeNode::OperatorBasedConditionMember { lval: k, rval: right_val, operator }));
		}

		_ => return original
	}
}

pub fn parse_ast_value(tokens: &Vec<LexerToken>, ind: &mut usize) -> ParserResult<Box<ASTTreeNode>> {
	match &tokens[*ind] {

		LexerToken::EXCLAMATION_MARK => {
			*ind += 1;
			let ast = parse_ast_value(tokens, ind)?;

			if ast.is_function_call() || ast.is_var_access() {
				return Ok(Box::new(ASTTreeNode::BooleanBasedConditionMember { val: ast, negate: true }))
			}

			return Err(ParserError::new(String::from("Boolean negation requires either function or variable usage!"), 0));
		},

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

			let n = Ok(Box::new(ASTTreeNode::VariableReference(WithHash::new(String::clone(str)))));

			*ind += 1;

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
		},

		LexerToken::IF => {
			return parse_if_statement(tokens, ind);
		},
		
		LexerToken::WHILE => {
			return parse_while_block(tokens, ind);
		}

		_ => {
			return Err(ParserError::new(format!("err: {:#?}", tokens[*ind]), 0));
		}

	}
}